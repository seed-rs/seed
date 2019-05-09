use crate::{
    dom_types,
    dom_types::{El, ElContainer, Namespace},
    routing, util, websys_bridge,
};
use futures::{future, Future};
use std::{cell::RefCell, collections::HashMap, panic, rc::Rc};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Document, Element, Event, EventTarget, Window};

/// Determines if an update should cause the VDom to rerender or not.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShouldRender {
    Render,
    Skip,
}

impl Default for ShouldRender {
    fn default() -> Self {
        ShouldRender::Render
    }
}

pub enum Effect<Ms> {
    Msg(Ms),
    FutureNoMsg(Box<dyn Future<Item = (), Error = ()> + 'static>),
    FutureMsg(Box<dyn Future<Item = Ms, Error = Ms> + 'static>),
}

impl<Ms> From<Ms> for Effect<Ms> {
    fn from(message: Ms) -> Self {
        Effect::Msg(message)
    }
}

impl<Ms> Effect<Ms> {
    /// Apply a function to the message. If the effect is a future, the map function
    /// will be called after the future is finished running.
    pub fn map<F, Ms2>(self, f: F) -> Effect<Ms2>
    where
        Ms: 'static,
        Ms2: 'static,
        F: Fn(Ms) -> Ms2 + 'static,
    {
        match self {
            Effect::Msg(msg) => Effect::Msg(f(msg)),
            Effect::FutureNoMsg(fut) => Effect::FutureNoMsg(fut),
            Effect::FutureMsg(fut) => Effect::FutureMsg(Box::new(fut.then(move |res| {
                let res = res.map(&f).map_err(&f);
                future::result(res)
            }))),
        }
    }
}

pub struct Update<Ms> {
    should_render: ShouldRender,
    effect: Option<Effect<Ms>>,
}

impl<Ms> From<ShouldRender> for Update<Ms> {
    fn from(should_render: ShouldRender) -> Self {
        Self {
            should_render,
            effect: None,
        }
    }
}

impl<Ms> Default for Update<Ms> {
    fn default() -> Self {
        Self::from(ShouldRender::Render)
    }
}

impl<Ms> Update<Ms> {
    pub fn with_msg(effect_msg: Ms) -> Self {
        Self {
            effect: Some(effect_msg.into()),
            ..Default::default()
        }
    }

    pub fn with_future<F>(future: F) -> Self
    where
        F: Future<Item = (), Error = ()> + 'static,
    {
        Self {
            effect: Some(Effect::FutureNoMsg(Box::new(future))),
            ..Default::default()
        }
    }

    pub fn with_future_msg<F>(future: F) -> Self
    where
        F: Future<Item = Ms, Error = Ms> + 'static,
    {
        Self {
            effect: Some(Effect::FutureMsg(Box::new(future))),
            ..Default::default()
        }
    }

    /// Modify this Update to skip rendering
    pub fn skip(mut self) -> Self {
        self.should_render = ShouldRender::Skip;
        self
    }

    /// Force rendering for this Update. Cancels `skip()`.
    pub fn render(mut self) -> Self {
        self.should_render = ShouldRender::Render;
        self
    }

    /// Apply a function to the message produced by the update effect, if one is present.
    /// If the effect is a future, the map function will be called after the future is
    /// finished running.
    pub fn map<F, Ms2>(self, f: F) -> Update<Ms2>
    where
        Ms: 'static,
        Ms2: 'static,
        F: Fn(Ms) -> Ms2 + 'static,
    {
        let Update {
            should_render,
            effect,
        } = self;
        let effect = effect.map(|effect| effect.map(f));
        Update {
            should_render,
            effect,
        }
    }
}

type UpdateFn<Ms, Mdl> = fn(Ms, &mut Mdl) -> Update<Ms>;
type ViewFn<Mdl, ElC> = fn(&Mdl) -> ElC;
type RoutesFn<Ms> = fn(&routing::Url) -> Ms;
type WindowEvents<Ms, Mdl> = fn(&Mdl) -> Vec<dom_types::Listener<Ms>>;
type MsgListeners<Ms> = Vec<Box<Fn(&Ms)>>;

pub struct Mailbox<Message: 'static> {
    func: Rc<Fn(Message)>,
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

type StoredPopstate = RefCell<Option<Closure<FnMut(Event)>>>;

/// Used as part of an interior-mutability pattern, ie Rc<RefCell<>>
pub struct AppData<Ms: 'static, Mdl> {
    // Model is in a RefCell here so we can modify it in self.update().
    pub model: RefCell<Mdl>,
    main_el_vdom: RefCell<Option<El<Ms>>>,
    pub popstate_closure: StoredPopstate,
    pub routes: RefCell<Option<RoutesFn<Ms>>>,
    window_listeners: RefCell<Vec<dom_types::Listener<Ms>>>,
    msg_listeners: RefCell<MsgListeners<Ms>>,
    //    mount_pt: RefCell<web_sys::Element>
}

pub struct AppCfg<Ms: 'static, Mdl: 'static, ElC: ElContainer<Ms>> {
    document: web_sys::Document,
    mount_point: web_sys::Element,
    pub update: UpdateFn<Ms, Mdl>,
    view: ViewFn<Mdl, ElC>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
}

pub struct App<Ms: 'static, Mdl: 'static, ElC: ElContainer<Ms>> {
    /// Stateless app configuration
    pub cfg: Rc<AppCfg<Ms, Mdl, ElC>>,
    /// Mutable app state
    pub data: Rc<AppData<Ms, Mdl>>,
}

impl<Ms: 'static, Mdl: 'static, ElC: ElContainer<Ms>> ::std::fmt::Debug for App<Ms, Mdl, ElC> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "App")
    }
}

pub trait MountPoint {
    fn element(self) -> Element;
}

impl MountPoint for &str {
    fn element(self) -> Element {
        // We log an error instead of relying on panic/except due to the panic hook not yet
        // being active.
        util::document().get_element_by_id(self).unwrap_or_else(|| {
            let text = format!(
                concat!(
                    "Can't find parent div with id={:?} (defaults to \"app\", or can be set with the .mount() method)",
                ),
                self,
            );
            crate::error(&text);
            panic!(text);
        })
    }
}

impl MountPoint for Element {
    fn element(self) -> Element {
        self
    }
}

impl MountPoint for web_sys::HtmlElement {
    fn element(self) -> Element {
        self.into()
    }
}

/// Used to create and store initial app configuration, ie items passed by the app creator
#[derive(Clone)]
pub struct AppBuilder<Ms: 'static, Mdl: 'static, ElC: ElContainer<Ms>> {
    model: Mdl,
    update: UpdateFn<Ms, Mdl>,
    view: ViewFn<Mdl, ElC>,
    mount_point: Option<Element>,
    routes: Option<RoutesFn<Ms>>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
}

impl<Ms, Mdl, ElC: ElContainer<Ms> + 'static> AppBuilder<Ms, Mdl, ElC> {
    pub fn mount(mut self, mount_point: impl MountPoint) -> Self {
        self.mount_point = Some(mount_point.element());
        self
    }

    #[deprecated(since = "0.3.3", note = "please use `mount` instead")]
    pub fn mount_el(mut self, el: Element) -> Self {
        self.mount_point = Some(el);
        self
    }

    pub fn routes(mut self, routes: RoutesFn<Ms>) -> Self {
        self.routes = Some(routes);
        self
    }

    pub fn window_events(mut self, evts: WindowEvents<Ms, Mdl>) -> Self {
        self.window_events = Some(evts);
        self
    }

    pub fn finish(mut self) -> App<Ms, Mdl, ElC> {
        if self.mount_point.is_none() {
            self = self.mount("app")
        }
        App::new(
            self.model,
            self.update,
            self.view,
            self.mount_point.unwrap(),
            self.routes,
            self.window_events,
        )
    }
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetitive sequences of parameters.
impl<Ms, Mdl, ElC: ElContainer<Ms> + 'static> App<Ms, Mdl, ElC> {
    pub fn build(
        model: Mdl,
        update: UpdateFn<Ms, Mdl>,
        view: ViewFn<Mdl, ElC>,
    ) -> AppBuilder<Ms, Mdl, ElC> {
        AppBuilder {
            model,
            update,
            view,
            mount_point: None,
            routes: None,
            window_events: None,
        }
    }

    fn new(
        model: Mdl,
        update: UpdateFn<Ms, Mdl>,
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
                view,
                window_events,
            }),
            data: Rc::new(AppData {
                model: RefCell::new(model),
                // This is filled for the first time in run()
                main_el_vdom: RefCell::new(None),
                popstate_closure: RefCell::new(None),
                routes: RefCell::new(routes),
                window_listeners: RefCell::new(Vec::new()),
                msg_listeners: RefCell::new(Vec::new()),
            }),
        }
    }

    pub fn setup_window_listeners(&self) {
        if let Some(window_events) = self.cfg.window_events {
            let mut new_listeners = (window_events)(&self.data.model.borrow());
            setup_window_listeners(
                &util::window(),
                &mut self.data.window_listeners.borrow_mut(),
                &mut new_listeners,
                &self.mailbox(),
            );
            self.data.window_listeners.replace(new_listeners);
        }
    }

    /// App initialization: Collect its fundamental components, setup, and perform
    /// an initial render.
    pub fn run(self) -> Self {
        // Our initial render. Can't initialize in new due to mailbox() requiring self.
        // "new" name is for consistency with update_inner.
        let mut new = El::empty(dom_types::Tag::Section);
        new.children = (self.cfg.view)(&self.data.model.borrow()).els();

        self.setup_window_listeners();

        setup_input_listeners(&mut new);
        setup_websys_el_and_children(&util::document(), &mut new);

        attach_listeners(&mut new, &self.mailbox());

        // Attach all top-level elements to the mount point: This is where our initial render occurs.
        for top_child in &mut new.children {
            websys_bridge::attach_el_and_children(top_child, &self.cfg.mount_point, &self)
        }

        self.data.main_el_vdom.replace(Some(new));

        let self_for_closure = self.clone();
        let self_for_closure2 = self.clone();
        let self_for_closure3 = self.clone();
        // Update the state on page load, based
        // on the starting URL. Must be set up on the server as well.
        if let Some(routes) = self.data.routes.borrow().clone() {
            routing::initial(|msg| self.update(msg), routes);
            routing::setup_popstate_listener(
                move |msg| self_for_closure.update(msg),
                move |closure| {
                    self_for_closure2
                        .data
                        .popstate_closure
                        .replace(Some(closure));
                },
                routes,
            );
            routing::setup_link_listener(move |msg| self_for_closure3.update(msg), routes);
        }

        // Allows panic messages to output to the browser console.error.
        panic::set_hook(Box::new(console_error_panic_hook::hook));

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
    pub fn update_inner(
        &self,
        message: Ms,
    ) -> Option<Box<dyn Future<Item = (), Error = ()> + 'static>> {
        for l in self.data.msg_listeners.borrow().iter() {
            (l)(&message)
        }

        let Update {
            should_render,
            effect,
        } = (self.cfg.update)(message, &mut self.data.model.borrow_mut());

        self.setup_window_listeners();

        if should_render == ShouldRender::Render {
            // Create a new vdom: The top element, and all its children. Does not yet
            // have associated web_sys elements.
            let mut new = El::empty(dom_types::Tag::Section);
            new.children = (self.cfg.view)(&self.data.model.borrow()).els();

            let mut old = self
                .data
                .main_el_vdom
                .borrow_mut()
                .take()
                .expect("missing main_el_vdom");

            // Detach all old listeners before patching. We'll re-add them as required during patching.
            // We'll get a runtime panic if any are left un-removed.
            detach_listeners(&mut old);

            // todo copied from children loop in patch fn (DRY)
            let num_children_in_both = old.children.len().min(new.children.len());
            let mut old_children_iter = old.children.into_iter();
            let mut new_children_iter = new.children.iter_mut();

            //            let mut last_visited_node: Option<web_sys::Node> = None;
            //
            //            if let Some(update_actions) = &mut placeholder_topel.hooks.did_update {
            //                (update_actions.actions)(&old_el_ws) // todo put in / back
            //            }

            for _i in 0..num_children_in_both {
                let child_old = old_children_iter.next().unwrap();
                let child_new = new_children_iter.next().unwrap();

                patch(
                    &self.cfg.document,
                    child_old,
                    child_new,
                    &self.cfg.mount_point,
                    //                    match last_visited_node.as_ref() {
                    //                        Some(node) => node.next_sibling(),
                    //                        None => old_el_ws.first_child(),
                    //                    },
                    None, // todo make it the next item in new?
                    &self.mailbox(),
                    &self.clone(),
                );
            }

            // Now that we've re-rendered, replace our stored El with the new one;
            // it will be used as the old El next time.
            self.data.main_el_vdom.borrow_mut().replace(new);
        }

        if let Some(effect) = effect {
            match effect {
                Effect::Msg(msg) => self.update_inner(msg),

                Effect::FutureNoMsg(fut) => Some(fut),

                Effect::FutureMsg(fut) => {
                    let self2 = self.clone();
                    Some(Box::new(fut.then(move |res| {
                        // Collapse Ok(Msg) and Err(Msg) to a Msg.
                        let msg = res.unwrap_or_else(std::convert::identity);
                        // Get next Some(future)
                        let fut2 = self2.update_inner(msg);
                        // We need to return a future anyway, so if we don't have one,
                        // return a trivial one
                        fut2.unwrap_or_else(|| Box::new(future::ok(())))
                    })))
                }
            }
        } else {
            None
        }
    }

    pub fn update(&self, message: Ms) {
        self.update_inner(message)
            .map(|fut| future_to_promise(fut.then(|_res| future::ok(JsValue::UNDEFINED))));
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

    fn mailbox(&self) -> Mailbox<Ms> {
        let cloned = self.clone();
        Mailbox::new(move |message| {
            cloned.update(message);
        })
    }
}

/// Set up controlled components: Input, Select, and TextArea elements must stay in sync with the
/// model; don't let them get out of sync from typing or other events, which can occur if a change
/// doesn't trigger a re-render, or if something else modifies them using a side effect.
/// Handle controlled inputs: Ie force sync with the model.
fn setup_input_listener<Ms>(el: &mut El<Ms>)
where
    Ms: 'static,
{
    if el.tag == dom_types::Tag::Input
        || el.tag == dom_types::Tag::Select
        || el.tag == dom_types::Tag::TextArea
    {
        let listener = if let Some(checked) = el.attrs.vals.get(&dom_types::At::Checked) {
            let checked_bool = match checked.as_ref() {
                "true" => true,
                "false" => false,
                _ => panic!("checked must be true or false."),
            };
            dom_types::Listener::new_control_check(checked_bool)
        } else if let Some(control_val) = el.attrs.vals.get(&dom_types::At::Value) {
            dom_types::Listener::new_control(control_val.to_string())
        } else {
            // If Value is not specified, force the field to be blank.
            dom_types::Listener::new_control("".to_string())
        };
        el.listeners.push(listener); // Add to the El, so we can deattach later.
    }
}

// Create the web_sys element; add it to the working tree; store it in its corresponding vdom El.
fn setup_websys_el<Ms>(document: &Document, el: &mut El<Ms>)
where
    Ms: 'static,
{
    if el.el_ws.is_none() {
        el.el_ws = Some(websys_bridge::make_websys_el(el, document));
    }
}

/// Recursively sets up input listeners
fn setup_input_listeners<Ms>(el_vdom: &mut El<Ms>)
where
    Ms: 'static,
{
    el_vdom.walk_tree_mut(setup_input_listener);
}

/// Recursively sets up web_sys elements
fn setup_websys_el_and_children<Ms>(document: &Document, el: &mut El<Ms>)
where
    Ms: 'static,
{
    el.walk_tree_mut(|el| setup_websys_el(document, el));
}

impl<Ms, Mdl, ElC: ElContainer<Ms>> Clone for App<Ms, Mdl, ElC> {
    fn clone(&self) -> Self {
        App {
            cfg: Rc::clone(&self.cfg),
            data: Rc::clone(&self.data),
        }
    }
}

/// Recursively attach all event-listeners. Run this after creating fresh elements.
fn attach_listeners<Ms>(el: &mut dom_types::El<Ms>, mailbox: &Mailbox<Ms>) {
    el.walk_tree_mut(|el| {
        if let Some(el_ws) = el.el_ws.as_ref() {
            for listener in &mut el.listeners {
                // todo ideally we unify attach as one method
                if listener.control_val.is_some() || listener.control_checked.is_some() {
                    listener.attach_control(&el_ws);
                } else {
                    listener.attach(el_ws, mailbox.clone());
                }
            }
        }
    });
}

/// Recursively detach event-listeners. Run this before patching.
fn detach_listeners<Ms>(el: &mut dom_types::El<Ms>) {
    el.walk_tree_mut(|el| {
        if let Some(el_ws) = el.el_ws.as_ref() {
            for listener in &mut el.listeners {
                listener.detach(el_ws);
            }
        }
    });
}

/// We reattach all listeners, as with normal Els, since we have no
/// way of diffing them.
fn setup_window_listeners<Ms>(
    window: &Window,
    old: &mut Vec<dom_types::Listener<Ms>>,
    new: &mut Vec<dom_types::Listener<Ms>>,
    mailbox: &Mailbox<Ms>,
) {
    for listener in old {
        listener.detach(window);
    }

    for listener in new {
        listener.attach(window, mailbox.clone());
    }
}

pub(crate) fn patch<'a, Ms, Mdl, ElC: ElContainer<Ms>>(
    document: &Document,
    mut old: El<Ms>,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    next_node: Option<web_sys::Node>,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, ElC>,
) -> Option<&'a web_sys::Node> {
    // Old_el_ws is what we're patching, with items from the new vDOM el; or replacing.
    // TODO: Current sceme is that if the parent changes, redraw all children...
    // TODO: fix this later.
    // We make an assumption that most of the page is not dramatically changed
    // by each event, to optimize.

    // Assume all listeners have been removed from the old el_ws (if any), and the
    // old el vdom's elements are still attached.

    // take removes the interior value from the Option; otherwise we run into problems
    // about not being able to remove from borrowed content.
    // We remove it from the old el_vodom now, and at the end... add it to the new one.
    // We don't run attach_children() when patching, hence this approach.

    if old != *new {
        // At this step, we already assume we have the right element - either
        // by entering this func directly for the top-level, or recursively after
        // analyzing children

        // If the tag's different, we must redraw the element and its children; there's
        // no way to patch one element type into another.
        // TODO: forcing a rerender for differnet listeners is inefficient
        // TODO:, but I'm not sure how to patch them.
        if new.empty && !old.empty {
            let old_el_ws = old
                .el_ws
                .take()
                .expect("old el_ws missing in call to unmount_actions");

            parent
                .remove_child(&old_el_ws)
                .expect("Problem removing old we_el when updating to empty");

            if let Some(unmount_actions) = &mut old.hooks.will_unmount {
                (unmount_actions.actions)(&old_el_ws);
                //                if let Some(message) = unmount_actions.message.clone() {
                //                    app.update(message);
                //                }
            }

            return None;
        // If new and old are empty, we don't need to do anything.
        } else if new.empty && old.empty {
            return None;
        }
        // Namespaces can't be patched, since they involve create_element_ns instead of create_element.
        // Something about this element itself is different: patch it.
        else if old.tag != new.tag
            || old.namespace != new.namespace
            || old.empty != new.empty
            || old.text.is_some() != new.text.is_some()
        {
            // TODO: DRY here between this and later in func.

            let old_el_ws = old.el_ws.take();

            if let Some(unmount_actions) = &mut old.hooks.will_unmount {
                (unmount_actions.actions)(
                    old_el_ws
                        .as_ref()
                        .expect("old el_ws missing in call to unmount_actions"),
                );
                //                if let Some(message) = unmount_actions.message.clone() {
                //                            app.update(message);
                //                }
            }

            // todo: Perhaps some of this next segment should be moved to websys_bridge
            setup_websys_el_and_children(document, new);
            websys_bridge::attach_children(new, app);

            let new_el_ws = new.el_ws.as_ref().expect("Missing websys el");

            if old.empty {
                parent
                    .insert_before(new_el_ws, next_node.as_ref())
                    .expect("Problem adding element to replace previously empty one");
            } else {
                parent
                    .replace_child(
                        new_el_ws,
                        &old_el_ws.expect("old el_ws missing in call to replace_child"),
                    )
                    .expect("Problem replacing element");
            }

            // Perform side-effects specified for mounting.
            if let Some(mount_actions) = &mut new.hooks.did_mount {
                (mount_actions.actions)(new_el_ws);
                //                if let Some(message) = mount_actions.message.clone() {
                //                            app.update_inner(message);
                //                }
            }

            attach_listeners(new, &mailbox);
            // We've re-rendered this child and all children; we're done with this recursion.
            return new.el_ws.as_ref();
        } else {
            // Patch parts of the Element.
            let old_el_ws = old
                .el_ws
                .as_ref()
                .expect("missing old el_ws when patching non-empty el")
                .clone();
            websys_bridge::patch_el_details(&mut old, new, &old_el_ws);
        }
    }

    if old.empty && new.empty {
        return None;
    }

    let old_el_ws = old.el_ws.take().unwrap();

    // Before running patch, assume we've removed all listeners from the old element.
    // Perform this attachment after we've verified we can patch this element, ie
    // it has the same tag - otherwise  we'd have to detach after the parent.remove_child step.
    // Note that unlike the attach_listeners function, this only attaches for the current
    // element.
    for listener in &mut new.listeners {
        if listener.control_val.is_some() || listener.control_checked.is_some() {
            listener.attach_control(&old_el_ws);
        } else {
            listener.attach(&old_el_ws, mailbox.clone());
        }
    }

    let num_children_in_both = old.children.len().min(new.children.len());
    let mut old_children_iter = old.children.into_iter();
    let mut new_children_iter = new.children.iter_mut();

    let mut last_visited_node: Option<web_sys::Node> = None;

    // TODO: Lines below commented out, because they were breaking `lifecycle_hooks` test
    //       - did_update was called 2x instead of 1x after 2nd call_patch
    //
    //  if let Some(update_actions) = &mut new.hooks.did_update {
    //      (update_actions.actions)(&old_el_ws) // todo
    //  }

    // Not using .zip() here to make sure we don't miss any of the children when one array is
    // longer than the other.
    for _i in 0..num_children_in_both {
        let child_old = old_children_iter.next().unwrap();
        let child_new = new_children_iter.next().unwrap();

        // Don't compare equality here; we do that at the top of this function
        // in the recursion.
        if let Some(new_el_ws) = patch(
            document,
            child_old,
            child_new,
            &old_el_ws,
            match last_visited_node.as_ref() {
                Some(node) => node.next_sibling(),
                None => old_el_ws.first_child(),
            },
            &mailbox,
            app,
        ) {
            last_visited_node = Some(new_el_ws.clone());
        }
    }

    // Now one of the iterators is entirely consumed, and any items left in one iterator
    // don't have any matching items in the other.

    while let Some(child_new) = new_children_iter.next() {
        // We ran out of old children to patch; create new ones.
        setup_websys_el_and_children(document, child_new);
        websys_bridge::attach_el_and_children(child_new, &old_el_ws, app);
        attach_listeners(child_new, &mailbox);
    }

    //    // Now pair up children as best we can.
    //    // If there are the same number of children, assume there's a 1-to-1 mapping,
    //    // where we will not add or remove any; but patch as needed.
    //    let avail_old_children = &mut old.children;
    //    let mut prev_child: Option<web_sys::Node> = None;
    //    let mut best_match;
    ////    let mut t;
    //    for (i_new, child_new) in new.children.iter_mut().enumerate() {
    //        if avail_old_children.is_empty() {
    //            // One or more new children has been added, or much content has
    //            // changed, or we've made a mistake: Attach new children.
    //            websys_bridge::attach_els(child_new, &old_el_ws);
    //            let mut child_new = child_new;
    //            attach_listeners(&mut child_new, &mailbox);
    //
    //        } else {
    //            // We still have old children to pick a match from. If we pick
    //            // incorrectly, or there is no "good" match, we'll have some
    //            // patching and/or attaching (rendering) to do in subsequent recursions.
    //            let mut scores: Vec<(u32, f32)> = avail_old_children
    //                .iter()
    //                .enumerate()
    //                .map(|(i_old, c_old)| (c_old.id.unwrap(), match_score(c_old, i_old, child_new, i_new)))
    //                .collect();
    //
    //            // should put highest score at the end.
    //            scores.sort_by(|b, a| b.1.partial_cmp(&a.1).unwrap());
    //
    //            // Sorting children vice picking the best one makes this easier to handle
    //            // without irking the borrow checker, despite appearing less counter-intuitive,
    //            // due to the convenient pop method.
    //            avail_old_children.sort_by(|b, a| {
    //                scores
    //                    .iter()
    //                    .find(|s| s.0 == b.id.unwrap())
    //                    .unwrap()
    //                    .1
    //                    .partial_cmp(&scores.iter().find(|s| s.0 == a.id.unwrap()).unwrap().1)
    //                    .unwrap()
    //            });
    //
    //            best_match = avail_old_children.pop().expect("Problem popping");

    // Now purge any existing no-longer-needed children; they're not part of the new vdom.
    while let Some(mut child) = old_children_iter.next() {
        let child_el_ws = child.el_ws.take().expect("Missing child el_ws");

        // TODO: DRY here between this and earlier in func
        if let Some(unmount_actions) = &mut child.hooks.will_unmount {
            (unmount_actions.actions)(&child_el_ws);
        }

        // todo get to the bottom of this: Ie why we need this code sometimes when using raw html elements.
        match old_el_ws.remove_child(&child_el_ws) {
            Ok(_) => {}
            Err(_) => {
                crate::log("Minor error patching html element. (remove)");
            }
        }
    }

    new.el_ws = Some(old_el_ws);
    new.el_ws.as_ref()
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
/// Assumes dependency on web_sys.
// TODO:: Do we need <Ms> ?
pub trait _DomEl<Ms>: Sized + PartialEq + DomElLifecycle {
    type Tg: PartialEq + ToString; // TODO: tostring
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
    // TODO: tying to dom_types is temp - defeats the urpose of the trait
    fn namespace(self) -> Option<Namespace>;

    // Methods
    fn empty(self) -> Self;

    // setters
    fn set_id(&mut self, id: Option<u32>);
    fn set_websys_el(&mut self, el: Option<Element>);
}

pub trait DomElLifecycle {
    fn did_mount(self) -> Option<Box<FnMut(&Element)>>;
    fn did_update(self) -> Option<Box<FnMut(&Element)>>;
    fn will_unmount(self) -> Option<Box<FnMut(&Element)>>;
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    use crate as seed; // required for macros to work.
    use crate::{class, prelude::*};
    use wasm_bindgen::JsCast;
    use web_sys::{Node, Text};

    #[derive(Clone, Debug)]
    enum Msg {}
    struct Model {}

    fn create_app() -> App<Msg, Model, El<Msg>> {
        App::build(Model {}, |_, _| Update::default(), |_| seed::empty())
            // mount to the element that exists even in the default test html
            .mount(util::body())
            .finish()
    }

    fn call_patch(
        doc: &Document,
        parent: &Element,
        mailbox: &Mailbox<Msg>,
        old_vdom: El<Msg>,
        mut new_vdom: El<Msg>,
        app: &App<Msg, Model, El<Msg>>,
    ) -> El<Msg> {
        patch(&doc, old_vdom, &mut new_vdom, parent, None, mailbox, &app);
        new_vdom
    }

    fn iter_nodelist(list: web_sys::NodeList) -> impl Iterator<Item = Node> {
        (0..list.length()).map(move |i| list.item(i).unwrap())
    }

    fn iter_child_nodes(node: &Node) -> impl Iterator<Item = Node> {
        iter_nodelist(node.child_nodes())
    }

    #[wasm_bindgen_test]
    fn el_added() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = El::empty(seed::dom_types::Tag::Div);
        setup_websys_el(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
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
    }

    #[wasm_bindgen_test]
    fn el_removed() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = El::empty(seed::dom_types::Tag::Div);
        setup_websys_el(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
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

    #[wasm_bindgen_test]
    fn el_changed() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = El::empty(seed::dom_types::Tag::Div);
        setup_websys_el(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
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
    }

    /// Test that if the first child was a seed::empty() and it is changed to a non-empty El,
    /// then the new element is inserted at the correct position.
    #[wasm_bindgen_test]
    fn empty_changed_in_front() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = El::empty(seed::dom_types::Tag::Div);
        setup_websys_el(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
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
    }

    /// Test that if a middle child was a seed::empty() and it is changed to a non-empty El,
    /// then the new element is inserted at the correct position.
    #[wasm_bindgen_test]
    fn empty_changed_in_the_middle() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = El::empty(seed::dom_types::Tag::Div);
        setup_websys_el(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
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
        let el_ws = vdom.el_ws.as_ref().expect("el_ws missing");
        assert!(el_ws.is_same_node(parent.first_child().as_ref()));
        assert_eq!(
            iter_child_nodes(&el_ws)
                .map(|node| node.text_content().unwrap())
                .collect::<Vec<_>>(),
            &["a", "c"],
        );
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

    /// Test that a text Node is correctly patched to an Element and vice versa
    #[wasm_bindgen_test]
    fn text_to_element_to_text() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = seed::empty();
        vdom = call_patch(&doc, &parent, &mailbox, vdom, El::new_text("abc"), &app);
        assert_eq!(parent.child_nodes().length(), 1);
        let text = parent
            .first_child()
            .unwrap()
            .dyn_ref::<Text>()
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
        call_patch(&doc, &parent, &mailbox, vdom, El::new_text("abc"), &app);
        assert_eq!(parent.child_nodes().length(), 1);
        let text = parent
            .first_child()
            .unwrap()
            .dyn_ref::<Text>()
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

        let node_ref: Rc<RefCell<Option<Node>>> = Default::default();
        let mount_op_counter: Rc<AtomicUsize> = Default::default();
        let update_counter: Rc<AtomicUsize> = Default::default();

        // A real view() function would recreate these closures on each call.
        // We create the closures once and then clone them, which is hopefully close enough.
        let did_mount_func = {
            let node_ref = node_ref.clone();
            let mount_op_counter = mount_op_counter.clone();
            move |node: &Node| {
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
            move |_node: &Node| {
                update_counter.fetch_add(1, SeqCst);
            }
        };
        let will_unmount_func = {
            let node_ref = node_ref.clone();
            move |_node: &Node| {
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

    /// Tests an update() function that repeatedly uses a future with a Msg to modify the model
    #[wasm_bindgen_test(async)]
    fn update_promises() -> impl Future<Item = (), Error = JsValue> {
        struct Model(u32);

        //        #[derive(Clone)]
        struct Msg;

        fn update(_: Msg, model: &mut Model) -> Update<Msg> {
            model.0 += 1;

            if model.0 < 100 {
                Update::with_future_msg(future::ok(Msg)).skip()
            } else {
                Skip.into()
            }
        }

        fn view(_: &Model) -> El<Msg> {
            div!["test"]
        }

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let app = App::new(Model(0), update, view, parent, None, None).run();

        let app2 = app.clone();
        app.update_inner(Msg)
            .unwrap()
            .map_err(|_: ()| JsValue::UNDEFINED)
            .and_then(move |_| {
                assert_eq!(app2.data.model.borrow_mut().0, 100);
                Ok(())
            })
    }
}
