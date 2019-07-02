use crate::{
    dom_types::{self, El, ElContainer, MessageMapper, Namespace},
    events, next_tick, routing, util, websys_bridge,
};
use enclose::enclose;
use futures::Future;
use next_tick::NextTick;
use std::{
    cell::RefCell,
    collections::{vec_deque::VecDeque, HashMap},
    rc::Rc,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, Element, Event, EventTarget, Window};

/// Function `call_update` is useful for calling submodules' `update`.
///
/// # Example
///
/// ```rust,no_run
///fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
///    match msg {
///        Msg::ExampleA(msg) => {
///            *orders = call_update(example_a::update, msg, &mut model.example_a)
///                .map_message(Msg::ExampleA);
///        }
///   }
///}
/// ```
pub fn call_update<Ms, Mdl>(update: UpdateFn<Ms, Mdl>, msg: Ms, model: &mut Mdl) -> Orders<Ms> {
    let mut orders = Orders::<Ms>::default();
    (update)(msg, model, &mut orders);
    orders
}

pub enum Effect<Ms> {
    Msg(Ms),
    Cmd(Box<dyn Future<Item = Ms, Error = Ms> + 'static>),
}

impl<Ms> From<Ms> for Effect<Ms> {
    fn from(message: Ms) -> Self {
        Effect::Msg(message)
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Effect<Ms> {
    type SelfWithOtherMs = Effect<OtherMs>;
    fn map_message(self, f: fn(Ms) -> OtherMs) -> Effect<OtherMs> {
        match self {
            Effect::Msg(msg) => Effect::Msg(f(msg)),
            Effect::Cmd(cmd) => Effect::Cmd(Box::new(cmd.map(f).map_err(f))),
        }
    }
}

/// Determines if an update should cause the `VDom` to rerender or not.
pub enum ShouldRender {
    Render,
    ForceRenderNow,
    Skip,
}

pub struct Orders<Ms> {
    should_render: ShouldRender,
    effects: VecDeque<Effect<Ms>>,
}

impl<Ms> Default for Orders<Ms> {
    fn default() -> Self {
        Self {
            should_render: ShouldRender::Render,
            effects: VecDeque::new(),
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Orders<Ms> {
    type SelfWithOtherMs = Orders<OtherMs>;
    fn map_message(self, f: fn(Ms) -> OtherMs) -> Orders<OtherMs> {
        Orders {
            should_render: self.should_render,
            effects: self
                .effects
                .into_iter()
                .map(|effect| effect.map_message(f))
                .collect(),
        }
    }
}

impl<Ms: 'static> Orders<Ms> {
    /// Schedule web page rerender after model update. It's the default behaviour.
    pub fn render(&mut self) -> &mut Self {
        self.should_render = ShouldRender::Render;
        self
    }

    /// Force web page to rerender immediately after model update.
    pub fn force_render_now(&mut self) -> &mut Self {
        self.should_render = ShouldRender::ForceRenderNow;
        self
    }

    /// Don't rerender web page after model update.
    pub fn skip(&mut self) -> &mut Self {
        self.should_render = ShouldRender::Skip;
        self
    }

    /// Call function `update` with the given `msg` after model update.
    /// You can call this function more times - messages will be sent in the same order.
    pub fn send_msg(&mut self, msg: Ms) -> &mut Self {
        self.effects.push_back(msg.into());
        self
    }

    /// Schedule given future `cmd` to be executed after model update.
    /// You can call this function more times - futures will be scheduled in the same order.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn write_emoticon_after_delay() -> impl Future<Item=Msg, Error=Msg> {
    ///    TimeoutFuture::new(2_000)
    ///        .map(|_| Msg::WriteEmoticon)
    ///        .map_err(|_| Msg::TimeoutError)
    ///}
    ///orders.perform_cmd(write_emoticon_after_delay());
    /// ```
    pub fn perform_cmd<C>(&mut self, cmd: C) -> &mut Self
    where
        C: Future<Item = Ms, Error = Ms> + 'static,
    {
        self.effects.push_back(Effect::Cmd(Box::new(cmd)));
        self
    }
}

type UpdateFn<Ms, Mdl> = fn(Ms, &mut Mdl, &mut Orders<Ms>);
type ViewFn<Mdl, ElC> = fn(&Mdl) -> ElC;
type RoutesFn<Ms> = fn(routing::Url) -> Ms;
type WindowEvents<Ms, Mdl> = fn(&Mdl) -> Vec<events::Listener<Ms>>;
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
    window_listeners: RefCell<Vec<events::Listener<Ms>>>,
    msg_listeners: RefCell<MsgListeners<Ms>>,
    scheduled_render_handle: RefCell<Option<util::RequestAnimationFrameHandle>>,
}

pub struct AppCfg<Ms, Mdl, ElC>
where
    Ms: 'static,
    Mdl: 'static,
    ElC: ElContainer<Ms>,
{
    document: web_sys::Document,
    mount_point: web_sys::Element,
    pub update: UpdateFn<Ms, Mdl>,
    view: ViewFn<Mdl, ElC>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
}

pub struct App<Ms, Mdl, ElC>
where
    Ms: 'static,
    Mdl: 'static,
    ElC: ElContainer<Ms>,
{
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
        util::document().get_element_by_id(self).unwrap_or_else(|| {
            panic!(
                "Can't find element with id={:?} - app cannot be mounted!\n\
                 (Id defaults to \"app\", or can be set with the .mount() method)",
                self
            )
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
    /// Choose the element where the application will be mounted.
    /// The default one is the element with `id` = "app".
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// // argument is `&str`
    /// mount("another_id")
    ///
    /// // argument is `HTMLElement`
    /// // NOTE: Be careful with mounting into body,
    /// // it can cause hard-to-debug bugs when there are other scripts in the body.
    /// mount(seed::body())
    ///
    /// // argument is `Element`
    /// mount(seed::body().querySelector("section").unwrap().unwrap())
    /// ```
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
        // Allows panic messages to output to the browser console.error.
        console_error_panic_hook::set_once();

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
                scheduled_render_handle: RefCell::new(None),
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
        // "new" name is for consistency with `update` function.
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

        // Update the state on page load, based
        // on the starting URL. Must be set up on the server as well.
        if let Some(routes) = *self.data.routes.borrow() {
            routing::initial(|msg| self.update(msg), routes);
            routing::setup_popstate_listener(
                enclose!((self => s) move |msg| s.update(msg)),
                enclose!((self => s) move |closure| {
                    s.data.popstate_closure.replace(Some(closure));
                }),
                routes,
            );
            routing::setup_link_listener(enclose!((self => s) move |msg| s.update(msg)), routes);
        }
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
    ///
    /// If you have no access to the [`App`](struct.App.html) instance you can use
    /// alternatively the [`seed::update`](fn.update.html) function.
    pub fn update(&self, message: Ms) {
        let mut msg_and_cmd_queue: VecDeque<Effect<Ms>> = VecDeque::new();
        msg_and_cmd_queue.push_front(message.into());

        while let Some(effect) = msg_and_cmd_queue.pop_front() {
            match effect {
                Effect::Msg(msg) => {
                    let mut new_effects = self.process_queue_message(msg);
                    msg_and_cmd_queue.append(&mut new_effects);
                }
                Effect::Cmd(cmd) => self.process_queue_cmd(cmd),
            }
        }
    }

    fn process_queue_message(&self, message: Ms) -> VecDeque<Effect<Ms>> {
        for l in self.data.msg_listeners.borrow().iter() {
            (l)(&message)
        }

        let mut orders = Orders::default();
        (self.cfg.update)(message, &mut self.data.model.borrow_mut(), &mut orders);

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

    fn schedule_render(&self) {
        let mut scheduled_render_handle = self.data.scheduled_render_handle.borrow_mut();

        if scheduled_render_handle.is_none() {
            let cb = Closure::wrap(Box::new(enclose!((self => s) move |_| {
                s.rerender_vdom();
                s.data.scheduled_render_handle.borrow_mut().take();
            }))
                as Box<FnMut(util::RequestAnimationFrameTime)>);

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

        // todo much of the code below is copied from the patch fn (DRY). The issue driving
        // todo this lies with the patch fn's `parent` parameter.
        let num_children_in_both = old.children.len().min(new.children.len());
        let mut old_children_iter = old.children.into_iter();
        let mut new_children_iter = new.children.iter_mut();

        let mut last_visited_node: Option<web_sys::Node> = None;

        for _i in 0..num_children_in_both {
            let child_old = old_children_iter.next().unwrap();
            let child_new = new_children_iter.next().unwrap();

            if let Some(new_el_ws) = patch(
                &self.cfg.document,
                child_old,
                child_new,
                &self.cfg.mount_point,
                match last_visited_node.as_ref() {
                    Some(node) => node.next_sibling(),
                    None => self.cfg.mount_point.first_child(),
                },
                &self.mailbox(),
                &self.clone(),
            ) {
                last_visited_node = Some(new_el_ws.clone());
            }
        }

        for child_new in new_children_iter {
            // We ran out of old children to patch; create new ones.
            setup_websys_el_and_children(&self.cfg.document, child_new);
            websys_bridge::attach_el_and_children(child_new, &self.cfg.mount_point, &self.clone());
            attach_listeners(child_new, &self.mailbox());
        }

        // Now purge any existing no-longer-needed children; they're not part of the new vdom.
        //    while let Some(mut child) = old_children_iter.next() {
        for mut child in old_children_iter {
            if child.empty {
                continue;
            }

            let child_el_ws = child.el_ws.take().expect("Missing child el_ws");

            if let Some(unmount_actions) = &mut child.hooks.will_unmount {
                (unmount_actions.actions)(&child_el_ws);
            }
        }

        // todo end DRY with patch().

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

    fn _find(&self, ref_: &str) -> Option<El<Ms>> {
        // todo expensive? We're cloning the whole vdom tree.
        // todo: Let's iterate through refs instead, once this is working.

        let top_el = &self
            .data
            .main_el_vdom
            .borrow()
            .clone()
            .expect("Can't find main vdom el in find");

        find_el(ref_, top_el)
    }

    fn mailbox(&self) -> Mailbox<Ms> {
        Mailbox::new(enclose!((self => s) move |message| {
            s.update(message);
        }))
    }
}

/// Set up controlled components: Input, Select, and `TextArea` elements must stay in sync with the
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
            events::Listener::new_control_check(checked_bool)
        } else if let Some(control_val) = el.attrs.vals.get(&dom_types::At::Value) {
            events::Listener::new_control(control_val.to_string())
        } else {
            // If Value is not specified, force the field to be blank.
            events::Listener::new_control("".to_string())
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

/// Recursively sets up `web_sys` elements
fn setup_websys_el_and_children<Ms>(document: &Document, el: &mut El<Ms>)
where
    Ms: 'static,
{
    el.walk_tree_mut(|el| setup_websys_el(document, el));
}

impl<Ms, Mdl, ElC: ElContainer<Ms>> Clone for App<Ms, Mdl, ElC> {
    fn clone(&self) -> Self {
        Self {
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
                    listener.attach_control(el_ws);
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
    old: &mut Vec<events::Listener<Ms>>,
    new: &mut Vec<events::Listener<Ms>>,
    mailbox: &Mailbox<Ms>,
) {
    // todo: Temporary shim to group all events using the same trigger
    // todo inton one, to prevent them from interupting each other.
    //    let mut by_trigger = HashMap::new();
    //    for l in new {
    //        match by_trigger.contains_key(l.trigger) {
    //            Some(v) => {
    //                let new_handlers = hand
    //                by_trigger.insert(l.trigger, );
    //            }
    //        }
    //    }
    //
    //    let grouped_listeners = events::Listener::new(
    //        |l|
    //    )
    //
    //    let mut new = Vec::new();
    //    for (trigger, closure) in grouped_listeners {
    //        new.push(trigger, );
    //    }

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
                .expect("Problem removing old el_ws when updating to empty");

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
                match next_node {
                    Some(n) => {
                        parent
                            .insert_before(new_el_ws, Some(&n))
                            .expect("Problem adding element to replace previously empty one");
                    }
                    None => {
                        parent
                            .append_child(new_el_ws)
                            .expect("Problem adding element to replace previously empty one");
                    }
                }
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
                //                            app.update(message);
                //                }
            }

            attach_listeners(new, mailbox);
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
            mailbox,
            app,
        ) {
            last_visited_node = Some(new_el_ws.clone());
        }
    }

    // Now one of the iterators is entirely consumed, and any items left in one iterator
    // don't have any matching items in the other.

    //    while let Some(child_new) = new_children_iter.next() {
    for child_new in new_children_iter {
        // We ran out of old children to patch; create new ones.
        setup_websys_el_and_children(document, child_new);
        websys_bridge::attach_el_and_children(child_new, &old_el_ws, app);
        attach_listeners(child_new, mailbox);
    }

    // Now purge any existing no-longer-needed children; they're not part of the new vdom.
    //    while let Some(mut child) = old_children_iter.next() {
    for mut child in old_children_iter {
        if child.empty {
            continue;
        }

        let child_el_ws = child.el_ws.take().expect("Missing child el_ws");

        // TODO: DRY here between this and earlier in func
        if let Some(unmount_actions) = &mut child.hooks.will_unmount {
            (unmount_actions.actions)(&child_el_ws);
        }

        // todo get to the bottom of this: Ie why we need this code sometimes when using raw html elements.
        match old_el_ws.remove_child(&child_el_ws) {
            Ok(_) => {}
            Err(_) => {
                crate::error("Minor error patching html element. (remove)");
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

/// Find the first element that matches the ref specified.
//pub fn find_el<'a, Msg>(ref_: &str, top_el: &'a El<Msg>) -> Option<&'a El<Msg>> {
pub fn find_el<Msg>(ref_: &str, top_el: &El<Msg>) -> Option<El<Msg>> {
    if top_el.ref_ == Some(ref_.to_string()) {
        return Some(top_el.clone());
    }

    for child in &top_el.children {
        let result = find_el(ref_, child);
        if result.is_some() {
            return result;
        }
    }
    None
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    use crate as seed;
    // required for macros to work.
    use crate::{class, prelude::*};
    use futures::future;
    use wasm_bindgen::JsCast;
    use web_sys::{Node, Text};

    #[derive(Clone, Debug)]
    enum Msg {}

    struct Model {}

    fn create_app() -> App<Msg, Model, El<Msg>> {
        App::build(Model {}, |_, _, _| (), |_| seed::empty())
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
        enum Msg {
            MessageReceived,
            CommandPerformed,
            Start,
        }

        fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
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
            Model {
                test_value_sender: Some(test_value_sender),
                ..Default::default()
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
