use std::{
    cell::RefCell,
    collections::{vec_deque::VecDeque, HashMap},
    rc::Rc,
};

use futures::Future;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Event, EventTarget};

use enclose::enclose;
use next_tick::NextTick;

pub mod alias;
pub use alias::*;
pub mod builder;
pub use builder::{Builder as AppBuilder, Init, UrlHandling};

use crate::{
    dom_types::{self, El, MessageMapper, Namespace, Node, View},
    events, next_tick,
    orders::OrdersContainer,
    patch, routing,
    util::{self, ClosureNew},
    websys_bridge,
};

pub enum Effect<Ms, GMs> {
    Msg(Ms),
    Cmd(Box<dyn Future<Item = Ms, Error = Ms> + 'static>),
    GMsg(GMs),
    GCmd(Box<dyn Future<Item = GMs, Error = GMs> + 'static>),
}

impl<Ms, GMs> From<Ms> for Effect<Ms, GMs> {
    fn from(message: Ms) -> Self {
        Effect::Msg(message)
    }
}

impl<Ms: 'static, OtherMs: 'static, GMs> MessageMapper<Ms, OtherMs> for Effect<Ms, GMs> {
    type SelfWithOtherMs = Effect<OtherMs, GMs>;
    fn map_message(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Effect<OtherMs, GMs> {
        match self {
            Effect::Msg(msg) => Effect::Msg(f(msg)),
            Effect::Cmd(cmd) => Effect::Cmd(Box::new(cmd.map(f.clone()).map_err(f))),
            Effect::GMsg(g_msg) => Effect::GMsg(g_msg),
            Effect::GCmd(g_cmd) => Effect::GCmd(g_cmd),
        }
    }
}

/// Determines if an update should cause the `VDom` to rerender or not.
pub enum ShouldRender {
    Render,
    ForceRenderNow,
    Skip,
}

pub struct Mailbox<Message: 'static> {
    func: Rc<dyn Fn(Message)>,
}

impl<Ms> Mailbox<Ms> {
    pub fn new(func: impl Fn(Ms) + 'static) -> Self {
        Mailbox {
            func: Rc::new(func),
        }
    }

    pub fn send(&self, message: Ms) {
        (self.func)(message)
    }
}

impl<Ms> Clone for Mailbox<Ms> {
    fn clone(&self) -> Self {
        Mailbox {
            func: self.func.clone(),
        }
    }
}

// TODO: Examine what needs to be ref cells, rcs etc

type StoredPopstate = RefCell<Option<Closure<dyn FnMut(Event)>>>;

/// Used as part of an interior-mutability pattern, ie Rc<RefCell<>>
pub struct AppData<Ms: 'static, Mdl> {
    // Model is in a RefCell here so we can modify it in self.update().
    pub model: RefCell<Option<Mdl>>,
    main_el_vdom: RefCell<Option<El<Ms>>>,
    pub popstate_closure: StoredPopstate,
    pub hashchange_closure: StoredPopstate,
    pub routes: RefCell<Option<RoutesFn<Ms>>>,
    window_listeners: RefCell<Vec<events::Listener<Ms>>>,
    msg_listeners: RefCell<MsgListeners<Ms>>,
    scheduled_render_handle: RefCell<Option<util::RequestAnimationFrameHandle>>,
}

pub struct AppCfg<Ms, Mdl, ElC, GMs>
where
    Ms: 'static,
    Mdl: 'static,
    ElC: View<Ms>,
{
    document: web_sys::Document,
    mount_point: web_sys::Element,
    pub update: UpdateFn<Ms, Mdl, ElC, GMs>,
    pub sink: Option<SinkFn<Ms, Mdl, ElC, GMs>>,
    view: ViewFn<Mdl, ElC>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
    initial_orders: RefCell<Option<OrdersContainer<Ms, Mdl, ElC, GMs>>>,
}

pub struct App<Ms, Mdl, ElC, GMs = ()>
where
    Ms: 'static,
    Mdl: 'static,
    ElC: View<Ms>,
{
    /// Stateless app configuration
    pub cfg: Rc<AppCfg<Ms, Mdl, ElC, GMs>>,
    /// Mutable app state
    pub data: Rc<AppData<Ms, Mdl>>,
}

impl<Ms: 'static, Mdl: 'static, ElC: View<Ms>, GMs> ::std::fmt::Debug for App<Ms, Mdl, ElC, GMs> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "App")
    }
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetitive sequences of parameters.
impl<Ms, Mdl, ElC: View<Ms> + 'static, GMs: 'static> App<Ms, Mdl, ElC, GMs> {
    pub fn build(
        init: impl FnOnce(routing::Url, &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl> + 'static,
        update: UpdateFn<Ms, Mdl, ElC, GMs>,
        view: ViewFn<Mdl, ElC>,
    ) -> AppBuilder<Ms, Mdl, ElC, GMs> {
        // Allows panic messages to output to the browser console.error.
        console_error_panic_hook::set_once();

        AppBuilder::new(Box::new(init), update, view)
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        update: UpdateFn<Ms, Mdl, ElC, GMs>,
        sink: Option<SinkFn<Ms, Mdl, ElC, GMs>>,
        view: ViewFn<Mdl, ElC>,
        mount_point: Element,
        routes: Option<RoutesFn<Ms>>,
        window_events: Option<WindowEvents<Ms, Mdl>>,
    ) -> Self {
        let window = util::window();
        let document = window.document().expect("Can't find the window's document");

        Self {
            cfg: Rc::new(AppCfg {
                document,
                mount_point,
                update,
                sink,
                view,
                window_events,
                initial_orders: RefCell::new(None),
            }),
            data: Rc::new(AppData {
                model: RefCell::new(None),
                // This is filled for the first time in run()
                main_el_vdom: RefCell::new(None),
                popstate_closure: RefCell::new(None),
                hashchange_closure: RefCell::new(None),
                routes: RefCell::new(routes),
                window_listeners: RefCell::new(Vec::new()),
                msg_listeners: RefCell::new(Vec::new()),
                scheduled_render_handle: RefCell::new(None),
            }),
        }
    }

    pub fn setup_window_listeners(&self) {
        if let Some(window_events) = self.cfg.window_events {
            let mut new_listeners = (window_events)(self.data.model.borrow().as_ref().unwrap());
            patch::setup_window_listeners(
                &util::window(),
                &mut self.data.window_listeners.borrow_mut(),
                &mut new_listeners,
                &self.mailbox(),
            );
            self.data.window_listeners.replace(new_listeners);
        }
    }

    fn bootstrapped_vdom(&self) -> El<Ms> {
        // "new" name is for consistency with `update` function.
        // this section parent is a placeholder, so we can iterate over children
        // in a way consistent with patching code.
        let mut new = El::empty(dom_types::Tag::Placeholder);

        self.setup_window_listeners();
        patch::setup_input_listeners(&mut new);
        patch::attach_listeners(&mut new, &self.mailbox());

        websys_bridge::assign_ws_nodes_to_el(&util::document(), &mut new);

        // Attach all top-level elements to the mount point.
        for child in &mut new.children {
            match child {
                Node::Element(child_el) => {
                    websys_bridge::attach_el_and_children(child_el, &self.cfg.mount_point);
                    patch::attach_listeners(child_el, &self.mailbox());
                }
                Node::Text(top_child_text) => {
                    websys_bridge::attach_text_node(top_child_text, &self.cfg.mount_point);
                }
                Node::Empty => (),
            }
        }

        new
    }

    /// App initialization: Collect its fundamental components, setup, and perform
    /// an initial render.
    pub fn run(self) -> Self {
        // Bootstrap the virtual DOM.
        self.data
            .main_el_vdom
            .replace(Some(self.bootstrapped_vdom()));

        // Update the state on page load, based
        // on the starting URL. Must be set up on the server as well.
        if let Some(routes) = *self.data.routes.borrow() {
            routing::setup_popstate_listener(
                enclose!((self => s) move |msg| s.update(msg)),
                enclose!((self => s) move |closure| {
                    s.data.popstate_closure.replace(Some(closure));
                }),
                routes,
            );
            routing::setup_hashchange_listener(
                enclose!((self => s) move |msg| s.update(msg)),
                enclose!((self => s) move |closure| {
                    s.data.hashchange_closure.replace(Some(closure));
                }),
                routes,
            );
            routing::setup_link_listener(enclose!((self => s) move |msg| s.update(msg)), routes);
        }

        // Our initial render. Can't initialize in new due to mailbox() requiring self.
        self.process_cmd_and_msg_queue(
            self.cfg
                .initial_orders
                .replace(None)
                .expect("initial_orders should be set in AppBuilder::finish")
                .effects,
        );
        // TODO: In the future, only run the following line if the above statement does not
        // TODO: call `rerender_vdom` for efficiency.
        self.rerender_vdom();

        self
    }

    /// This runs whenever the state is changed, ie the user-written update function is called.
    /// It updates the state, and any DOM elements affected by this change.
    /// todo this is where we need to compare against differences and only update nodes affected
    /// by the state change.
    ///
    /// We re-create the whole virtual dom each time (Is there a way around this? Probably not without
    /// knowing what vars the model holds ahead of time), but only edit the rendered, web_sys dom
    /// for things that have been changed.
    /// We re-render the virtual DOM on every change, but (attempt to) only change
    /// the actual DOM, via web_sys, when we need.
    /// The model stored in inner is the old model; updated_model is a newly-calculated one.
    pub fn update(&self, message: Ms) {
        let mut queue: VecDeque<Effect<Ms, GMs>> = VecDeque::new();
        queue.push_front(message.into());
        self.process_cmd_and_msg_queue(queue);
    }

    pub fn sink(&self, g_msg: GMs) {
        let mut queue: VecDeque<Effect<Ms, GMs>> = VecDeque::new();
        queue.push_front(Effect::GMsg(g_msg));
        self.process_cmd_and_msg_queue(queue);
    }

    pub fn process_cmd_and_msg_queue(&self, mut queue: VecDeque<Effect<Ms, GMs>>) {
        while let Some(effect) = queue.pop_front() {
            match effect {
                Effect::Msg(msg) => {
                    let mut new_effects = self.process_queue_message(msg);
                    queue.append(&mut new_effects);
                }
                Effect::GMsg(g_msg) => {
                    let mut new_effects = self.process_queue_global_message(g_msg);
                    queue.append(&mut new_effects);
                }
                Effect::Cmd(cmd) => self.process_queue_cmd(cmd),
                Effect::GCmd(g_cmd) => self.process_queue_global_cmd(g_cmd),
            }
        }
    }

    fn process_queue_message(&self, message: Ms) -> VecDeque<Effect<Ms, GMs>> {
        for l in self.data.msg_listeners.borrow().iter() {
            (l)(&message)
        }

        let mut orders = OrdersContainer::new(self.clone());
        (self.cfg.update)(
            message,
            &mut self.data.model.borrow_mut().as_mut().unwrap(),
            &mut orders,
        );

        self.setup_window_listeners();

        match orders.should_render {
            ShouldRender::Render => self.schedule_render(),
            ShouldRender::ForceRenderNow => {
                self.cancel_scheduled_render();
                self.rerender_vdom();
            }
            ShouldRender::Skip => (),
        };
        orders.effects
    }

    fn process_queue_global_message(&self, g_message: GMs) -> VecDeque<Effect<Ms, GMs>> {
        let mut orders = OrdersContainer::new(self.clone());

        if let Some(sink) = self.cfg.sink {
            sink(
                g_message,
                &mut self.data.model.borrow_mut().as_mut().unwrap(),
                &mut orders,
            );
        }

        self.setup_window_listeners();

        match orders.should_render {
            ShouldRender::Render => self.schedule_render(),
            ShouldRender::ForceRenderNow => {
                self.cancel_scheduled_render();
                self.rerender_vdom();
            }
            ShouldRender::Skip => (),
        };
        orders.effects
    }

    fn process_queue_cmd(&self, cmd: Box<dyn Future<Item = Ms, Error = Ms>>) {
        let lazy_schedule_cmd = enclose!((self => s) move |_| {
            // schedule future (cmd) to be executed
            spawn_local(cmd.then(move |res| {
                let msg_returned_from_effect = res.unwrap_or_else(|err_msg| err_msg);
                // recursive call which can blow the call stack
                s.update(msg_returned_from_effect);
                Ok(())
            }))
        });
        // we need to clear the call stack by NextTick so we don't exceed it's capacity
        spawn_local(NextTick::new().map(lazy_schedule_cmd));
    }

    fn process_queue_global_cmd(&self, g_cmd: Box<dyn Future<Item = GMs, Error = GMs>>) {
        let lazy_schedule_cmd = enclose!((self => s) move |_| {
            // schedule future (g_cmd) to be executed
            spawn_local(g_cmd.then(move |res| {
                let msg_returned_from_effect = res.unwrap_or_else(|err_msg| err_msg);
                // recursive call which can blow the call stack
                s.sink(msg_returned_from_effect);
                Ok(())
            }))
        });
        // we need to clear the call stack by NextTick so we don't exceed it's capacity
        spawn_local(NextTick::new().map(lazy_schedule_cmd));
    }

    fn schedule_render(&self) {
        let mut scheduled_render_handle = self.data.scheduled_render_handle.borrow_mut();

        if scheduled_render_handle.is_none() {
            let cb = Closure::new(enclose!((self => s) move |_| {
                s.rerender_vdom();
                s.data.scheduled_render_handle.borrow_mut().take();
            }));

            *scheduled_render_handle = Some(util::request_animation_frame(cb));
        }
    }

    fn cancel_scheduled_render(&self) {
        // Cancel animation frame request by dropping it.
        self.data.scheduled_render_handle.borrow_mut().take();
    }

    fn rerender_vdom(&self) {
        // Create a new vdom: The top element, and all its children. Does not yet
        // have associated web_sys elements.
        let mut new = El::empty(dom_types::Tag::Placeholder);
        new.children = (self.cfg.view)(self.data.model.borrow().as_ref().unwrap()).els();

        let mut old = self
            .data
            .main_el_vdom
            .borrow_mut()
            .take()
            .expect("missing main_el_vdom");

        // Detach all old listeners before patching. We'll re-add them as required during patching.
        // We'll get a runtime panic if any are left un-removed.
        patch::detach_listeners(&mut old);

        patch::patch_els(
            &self.cfg.document,
            &self.mailbox(),
            &self.clone(),
            &self.cfg.mount_point,
            old.children.into_iter(),
            new.children.iter_mut(),
        );

        // Now that we've re-rendered, replace our stored El with the new one;
        // it will be used as the old El next time.
        self.data.main_el_vdom.borrow_mut().replace(new);
    }

    pub fn add_message_listener<F>(&self, listener: F)
    where
        F: Fn(&Ms) + 'static,
    {
        self.data
            .msg_listeners
            .borrow_mut()
            .push(Box::new(listener));
    }

    // todo add back once you sort how to handle with Node refactor
    //    fn _find(&self, ref_: &str) -> Option<Node<Ms>> {
    //        // todo expensive? We're cloning the whole vdom tree.
    //        // todo: Let's iterate through refs instead, once this is working.
    //
    //        let top_el = &self
    //            .data
    //            .main_el_vdom
    //            .borrow()
    //            .clone()
    //            .expect("Can't find main vdom el in find");
    //
    //        find_el(ref_, top_el)
    //    }

    fn mailbox(&self) -> Mailbox<Ms> {
        Mailbox::new(enclose!((self => s) move |message| {
            s.update(message);
        }))
    }
}

impl<Ms, Mdl, ElC: View<Ms>, GMs> Clone for App<Ms, Mdl, ElC, GMs> {
    fn clone(&self) -> Self {
        Self {
            cfg: Rc::clone(&self.cfg),
            data: Rc::clone(&self.data),
        }
    }
}

pub trait _Attrs: PartialEq + ToString {
    fn vals(self) -> HashMap<String, String>;
}

pub trait _Style: PartialEq + ToString {
    fn vals(self) -> HashMap<String, String>;
}

pub trait _Listener<Ms>: Sized {
    fn attach<T: AsRef<EventTarget>>(&mut self, el_ws: &T, mailbox: Mailbox<Ms>);
    fn detach<T: AsRef<EventTarget>>(&self, el_ws: &T);
}

/// WIP towards a modular VDOM
/// Assumes dependency on `web_sys`.
// TODO:: Do we need <Ms> ?
pub trait _DomEl<Ms>: Sized + PartialEq + DomElLifecycle {
    // TODO: tostring
    type Tg: PartialEq + ToString;
    type At: _Attrs;
    type St: _Style;
    type Ls: _Listener<Ms>;
    type Tx: PartialEq + ToString + Clone + Default;

    // Fields
    fn tag(self) -> Self::Tg;
    fn attrs(self) -> Self::At;
    fn style(self) -> Self::St;
    fn listeners(self) -> Vec<Self::Ls>;
    fn text(self) -> Option<Self::Tx>;
    fn children(self) -> Vec<Self>;
    fn websys_el(self) -> Option<web_sys::Element>;
    fn id(self) -> Option<u32>;
    // TODO: tying to dom_types is temp - defeats the purpose of the trait
    fn namespace(self) -> Option<Namespace>;

    // Methods
    fn empty(self) -> Self;

    // setters
    fn set_id(&mut self, id: Option<u32>);
    fn set_websys_el(&mut self, el: Option<Element>);
}

pub trait DomElLifecycle {
    fn did_mount(self) -> Option<Box<dyn FnMut(&Element)>>;
    fn did_update(self) -> Option<Box<dyn FnMut(&Element)>>;
    fn will_unmount(self) -> Option<Box<dyn FnMut(&Element)>>;
}

// todo add back once you sort out how to handle with Node
///// Find the first element that matches the ref specified.
//pub fn find_el<Msg>(ref_: &str, top_el: &El<Msg>) -> Option<El<Msg>> {
//    if top_el.ref_ == Some(ref_.to_string()) {
//        return Some(top_el.clone());
//    }
//
//    for child in &top_el.children {
//        let result = find_el(ref_, child);
//        if result.is_some() {
//            return result;
//        }
//    }
//    None
//}

#[cfg(test)]
pub mod tests {
    use futures::future;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    use web_sys;

    use crate as seed;
    // required for macros to work.
    use crate::{class, prelude::*};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, Debug)]
    enum Msg {}

    struct Model {}

    fn create_app() -> App<Msg, Model, Node<Msg>> {
        App::build(|_,_| Init::new(Model {}), |_, _, _| (), |_| seed::empty())
            // mount to the element that exists even in the default test html
            .mount(util::body())
            .finish()
    }

    fn call_patch(
        doc: &web_sys::Document,
        parent: &Element,
        mailbox: &Mailbox<Msg>,
        old_vdom: Node<Msg>,
        mut new_vdom: Node<Msg>,
        app: &App<Msg, Model, Node<Msg>>,
    ) -> Node<Msg> {
        patch::patch(&doc, old_vdom, &mut new_vdom, parent, None, mailbox, &app);
        new_vdom
    }

    fn iter_nodelist(list: web_sys::NodeList) -> impl Iterator<Item = web_sys::Node> {
        (0..list.length()).map(move |i| list.item(i).unwrap())
    }

    fn iter_child_nodes(node: &web_sys::Node) -> impl Iterator<Item = web_sys::Node> {
        iter_nodelist(node.child_nodes())
    }

    #[wasm_bindgen_test]
    fn el_added() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(seed::dom_types::Tag::Div));
        websys_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            vdom = call_patch(&doc, &parent, &mailbox, vdom, div!["text"], &app);
            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(old_ws.child_nodes().length(), 1);
            assert_eq!(
                old_ws.first_child().unwrap().text_content().unwrap(),
                "text"
            );

            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div!["text", "more text", vec![li!["even more text"]]],
                &app,
            );

            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(old_ws.child_nodes().length(), 3);
            assert_eq!(
                old_ws
                    .child_nodes()
                    .item(0)
                    .unwrap()
                    .text_content()
                    .unwrap(),
                "text"
            );
            assert_eq!(
                old_ws
                    .child_nodes()
                    .item(1)
                    .unwrap()
                    .text_content()
                    .unwrap(),
                "more text"
            );
            let child3 = old_ws.child_nodes().item(2).unwrap();
            assert_eq!(child3.node_name(), "LI");
            assert_eq!(child3.text_content().unwrap(), "even more text");
        } else {
            panic!("Node not Element")
        }
    }

    #[wasm_bindgen_test]
    fn el_removed() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(seed::dom_types::Tag::Div));
        websys_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            // First add some child nodes using the vdom
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div!["text", "more text", vec![li!["even more text"]]],
                &app,
            );

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 3);
            let old_child1 = old_ws.child_nodes().item(0).unwrap();

            // Now test that patch function removes the last 2 nodes
            call_patch(&doc, &parent, &mailbox, vdom, div!["text"], &app);

            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(old_ws.child_nodes().length(), 1);
            assert!(old_child1.is_same_node(old_ws.child_nodes().item(0).as_ref()));
        }
    }

    #[wasm_bindgen_test]
    fn el_changed() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(seed::dom_types::Tag::Div));
        websys_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(el) = vdom.clone() {
            let old_ws = el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            // First add some child nodes using the vdom
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![span!["hello"], ", ", span!["world"]],
                &app,
            );

            assert_eq!(parent.child_nodes().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 3);

            // Now add some attributes
            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![
                    span![class!["first"], "hello"],
                    ", ",
                    span![class!["second"], "world"],
                ],
                &app,
            );

            let child1 = old_ws
                .child_nodes()
                .item(0)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(child1.get_attribute("class"), Some("first".to_string()));
            let child3 = old_ws
                .child_nodes()
                .item(2)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(child3.get_attribute("class"), Some("second".to_string()));
        } else {
            panic!("Node not Element")
        }
    }

    #[wasm_bindgen_test]
    fn els_changed_correct_order() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = div![];
        websys_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(el) = vdom.clone() {
            let old_ws = el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            vdom = call_patch(&doc, &parent, &mailbox, vdom, div!["1", a!["2"]], &app);
            let html_result = old_ws.clone().dyn_into::<Element>().unwrap().inner_html();
            assert_eq!(html_result, "1<a>2</a>");

            call_patch(&doc, &parent, &mailbox, vdom, div![a!["A"], "B"], &app);
            let html_result = old_ws.dyn_into::<Element>().unwrap().inner_html();
            assert_eq!(html_result, "<a>A</a>B");
        } else {
            panic!("Node not Element")
        }
    }

    /// Test if attribute `disabled` is correctly added and then removed.
    #[wasm_bindgen_test]
    fn attr_disabled() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(seed::dom_types::Tag::Div));
        websys_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            // First add button without attribute `disabled`
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![button![attrs! { At::Disabled => false.as_at_value() }]],
                &app,
            );

            assert_eq!(parent.child_nodes().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 1);
            let button = old_ws
                .child_nodes()
                .item(0)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(button.has_attribute("disabled"), false);

            // Now add attribute `disabled`
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![button![attrs! { At::Disabled => true.as_at_value() }]],
                &app,
            );

            let button = old_ws
                .child_nodes()
                .item(0)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(
                button
                    .get_attribute("disabled")
                    .expect("button hasn't got attribute `disabled`!"),
                ""
            );

            // And remove attribute `disabled`
            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![button![attrs! { At::Disabled => false.as_at_value() }]],
                &app,
            );

            let button = old_ws
                .child_nodes()
                .item(0)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(button.has_attribute("disabled"), false);
        } else {
            panic!("Node not El")
        }
    }

    /// Test that if the first child was a seed::empty() and it is changed to a non-empty El,
    /// then the new element is inserted at the correct position.
    #[wasm_bindgen_test]
    fn empty_changed_in_front() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(seed::dom_types::Tag::Div));
        websys_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![seed::empty(), "b", "c"],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&old_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["b", "c"],
            );

            call_patch(&doc, &parent, &mailbox, vdom, div!["a", "b", "c"], &app);

            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&old_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["a", "b", "c"],
            );
        } else {
            panic!("Not Element node")
        }
    }

    /// Test that if a middle child was a seed::empty() and it is changed to a non-empty El,
    /// then the new element is inserted at the correct position.
    #[wasm_bindgen_test]
    fn empty_changed_in_the_middle() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(seed::dom_types::Tag::Div));
        websys_bridge::assign_ws_nodes(&doc, &mut vdom);
        if let Node::Element(vdom_el) = vdom.clone() {
            // clone so we can keep using it after vdom is modified
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div!["a", seed::empty(), "c"],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&old_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["a", "c"],
            );

            call_patch(&doc, &parent, &mailbox, vdom, div!["a", "b", "c"], &app);

            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&old_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["a", "b", "c"],
            );
        } else {
            panic!("Not Element node")
        }
    }

    /// Test that if the old_el passed to patch was itself an empty, it is correctly patched to a non-empty.
    #[wasm_bindgen_test]
    fn root_empty_changed() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = seed::empty();

        vdom = call_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div!["a", seed::empty(), "c"],
            &app,
        );
        assert_eq!(parent.children().length(), 1);
        if let Node::Element(vdom_el) = vdom {
            let el_ws = vdom_el.node_ws.as_ref().expect("el_ws missing");
            assert!(el_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&el_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["a", "c"],
            );
        } else {
            panic!("Node not Element type")
        }
    }

    /// Test that an empty->empty transition is handled correctly.
    #[wasm_bindgen_test]
    fn root_empty_to_empty() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let old = seed::empty();
        call_patch(&doc, &parent, &mailbox, old, seed::empty(), &app);
        assert_eq!(parent.children().length(), 0);
    }

    /// Test that a text Node is correctly patched to an Element and vice versa.
    #[wasm_bindgen_test]
    fn text_to_element_to_text() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = seed::empty();
        vdom = call_patch(&doc, &parent, &mailbox, vdom, Node::new_text("abc"), &app);
        assert_eq!(parent.child_nodes().length(), 1);
        let text = parent
            .first_child()
            .unwrap()
            .dyn_ref::<web_sys::Text>()
            .expect("not a Text node")
            .clone();
        assert_eq!(text.text_content().unwrap(), "abc");

        // change to a span (that contains a text node and styling).
        // span was specifically chosen here because text Els are saved with the span tag.
        // (or at least they were when the test was written.)
        vdom = call_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            span![style!["color" => "red"], "def"],
            &app,
        );
        assert_eq!(parent.child_nodes().length(), 1);
        let element = parent
            .first_child()
            .unwrap()
            .dyn_ref::<Element>()
            .expect("not an Element node")
            .clone();
        assert_eq!(&element.tag_name().to_lowercase(), "span");

        // change back to a text node
        call_patch(&doc, &parent, &mailbox, vdom, Node::new_text("abc"), &app);
        assert_eq!(parent.child_nodes().length(), 1);
        let text = parent
            .first_child()
            .unwrap()
            .dyn_ref::<web_sys::Text>()
            .expect("not a Text node")
            .clone();
        assert_eq!(text.text_content().unwrap(), "abc");
    }

    /// Test that the lifecycle hooks are called correctly.
    #[wasm_bindgen_test]
    fn lifecycle_hooks() {
        let app = create_app();
        use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = seed::empty();

        let node_ref: Rc<RefCell<Option<web_sys::Node>>> = Default::default();
        let mount_op_counter: Rc<AtomicUsize> = Default::default();
        let update_counter: Rc<AtomicUsize> = Default::default();

        // A real view() function would recreate these closures on each call.
        // We create the closures once and then clone them, which is hopefully close enough.
        let did_mount_func = {
            let node_ref = node_ref.clone();
            let mount_op_counter = mount_op_counter.clone();
            move |node: &web_sys::Node| {
                node_ref.borrow_mut().replace(node.clone());
                assert_eq!(
                    mount_op_counter.fetch_add(1, SeqCst),
                    0,
                    "did_mount was called more than once"
                );
            }
        };
        let did_update_func = {
            let update_counter = update_counter.clone();
            move |_node: &web_sys::Node| {
                update_counter.fetch_add(1, SeqCst);
            }
        };
        let will_unmount_func = {
            let node_ref = node_ref.clone();
            move |_node: &web_sys::Node| {
                node_ref.borrow_mut().take();
                // If the counter wasn't 1, then either:
                // * did_mount wasn't called - we already check this elsewhere
                // * did_mount was called more than once - we already check this elsewhere
                // * will_unmount was called more than once
                assert_eq!(
                    mount_op_counter.fetch_add(1, SeqCst),
                    1,
                    "will_unmount was called more than once"
                );
            }
        };

        vdom = call_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div![
                "a",
                did_mount(did_mount_func.clone()),
                did_update(did_update_func.clone()),
                will_unmount(will_unmount_func.clone()),
            ],
            &app,
        );
        assert!(
            node_ref.borrow().is_some(),
            "did_mount wasn't called and should have been"
        );
        assert_eq!(
            update_counter.load(SeqCst),
            0,
            "did_update was called and shouldn't have been"
        );
        let first_child = parent.first_child().unwrap();
        assert!(node_ref
            .borrow()
            .as_ref()
            .unwrap()
            .is_same_node(Some(&first_child)));

        // now modify the element, see if did_update gets called.
        vdom = call_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div![
                "a",
                attrs! {At::Href => "#"},
                did_mount(did_mount_func.clone()),
                did_update(did_update_func.clone()),
                will_unmount(will_unmount_func.clone()),
            ],
            &app,
        );
        assert!(
            node_ref
                .borrow()
                .as_ref()
                .expect("will_unmount was called early")
                .is_same_node(Some(&first_child)),
            "node reference changed"
        );
        assert_eq!(
            update_counter.load(SeqCst),
            1,
            "did_update wasn't called and should have been"
        );

        // and now unmount the element to see if will_unmount gets called.
        call_patch(&doc, &parent, &mailbox, vdom, seed::empty(), &app);
        assert!(node_ref.borrow().is_none(), "will_unmount wasn't called");
    }

    /// Tests an update() function that repeatedly sends messages or performs commands.
    #[wasm_bindgen_test(async)]
    fn update_promises() -> impl Future<Item = (), Error = JsValue> {
        // ARRANGE

        // when we call `test_value_sender.send(..)`, future `test_value_receiver` will be marked as resolved
        let (test_value_sender, test_value_receiver) = futures::oneshot::<Counters>();

        // big numbers because we want to test if it doesn't blow call-stack
        // Note: Firefox has bigger call stack then Chrome - see http://2ality.com/2014/04/call-stack-size.html
        const MESSAGES_TO_SEND: i32 = 5_000;
        const COMMANDS_TO_PERFORM: i32 = 4_000;

        #[derive(Default, Copy, Clone, Debug)]
        struct Counters {
            messages_sent: i32,
            commands_scheduled: i32,
            messages_received: i32,
            commands_performed: i32,
        }

        #[derive(Default)]
        struct Model {
            counters: Counters,
            test_value_sender: Option<futures::sync::oneshot::Sender<Counters>>,
        }
        #[derive(Clone)]
        enum Msg {
            MessageReceived,
            CommandPerformed,
            Start,
        }

        fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
            orders.skip();

            match msg {
                Msg::MessageReceived => model.counters.messages_received += 1,
                Msg::CommandPerformed => model.counters.commands_performed += 1,
                Msg::Start => (),
            }

            if model.counters.messages_sent < MESSAGES_TO_SEND {
                orders.send_msg(Msg::MessageReceived);
                model.counters.messages_sent += 1;
            }
            if model.counters.commands_scheduled < MESSAGES_TO_SEND {
                orders.perform_cmd(future::ok(Msg::CommandPerformed));
                model.counters.commands_scheduled += 1;
            }

            if model.counters.messages_received == MESSAGES_TO_SEND
                && model.counters.commands_performed == COMMANDS_TO_PERFORM
            {
                model
                    .test_value_sender
                    .take()
                    .unwrap()
                    .send(model.counters)
                    .unwrap()
            }
        }

        let app = App::build(
            |_, _| {
                Init::new(Model {
                    test_value_sender: Some(test_value_sender),
                    ..Default::default()
                })
            },
            update,
            |_| seed::empty(),
        )
        .mount(seed::body())
        .finish()
        .run();

        // ACT
        app.update(Msg::Start);

        // ASSERT
        test_value_receiver
            .map(|counters| {
                assert_eq!(counters.messages_received, MESSAGES_TO_SEND);
                assert_eq!(counters.commands_performed, COMMANDS_TO_PERFORM);
            })
            .map_err(|_| panic!("test_value_sender.send probably wasn't called!"))
    }
}
