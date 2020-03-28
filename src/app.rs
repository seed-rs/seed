use crate::browser::dom::virtual_dom_bridge;
use crate::browser::{
    service::routing,
    url,
    util::{self, window, ClosureNew},
    Url,
};
use crate::virtual_dom::{patch, El, EventHandlerManager, IntoNodes, Mailbox, Node, Tag};
use builder::{
    init::{Init, InitFn as BuilderInitFn},
    IntoAfterMount, MountPointInitInitAPI, UndefinedInitAPI, UndefinedMountPoint,
};
use enclose::{enc, enclose};
use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::VecDeque,
    marker::PhantomData,
    rc::Rc,
};
use types::*;
use wasm_bindgen::closure::Closure;
use web_sys::Element;

pub mod builder;
pub mod cfg;
pub mod cmd_manager;
pub mod cmds;
pub mod data;
pub mod effects;
pub mod get_element;
pub mod message_mapper;
pub mod orders;
pub mod render_timestamp_delta;
pub mod stream_manager;
pub mod streams;
pub mod sub_manager;
pub mod subs;
pub mod types;

pub use builder::{
    AfterMount, BeforeMount, Builder as AppBuilder, MountPoint, MountType, UndefinedAfterMount,
    UrlHandling,
};
pub use cfg::{AppCfg, AppInitCfg};
pub use cmd_manager::{CmdHandle, CmdManager};
pub use data::AppData;
pub use effects::Effect;
pub use get_element::GetElement;
pub use message_mapper::MessageMapper;
pub use orders::{Orders, OrdersContainer, OrdersProxy};
pub use render_timestamp_delta::RenderTimestampDelta;
pub use stream_manager::{StreamHandle, StreamManager};
pub use sub_manager::{Notification, SubHandle, SubManager};

pub struct UndefinedGMsg;

type OptDynInitCfg<Ms, Mdl, INodes, GMs> =
    Option<AppInitCfg<Ms, Mdl, INodes, GMs, dyn IntoAfterMount<Ms, Mdl, INodes, GMs>>>;

/// Determines if an update should cause the `VDom` to rerender or not.
pub enum ShouldRender {
    Render,
    ForceRenderNow,
    Skip,
}

pub struct App<Ms, Mdl, INodes, GMs = UndefinedGMsg>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    /// Temporary app configuration that is removed after app begins running.
    pub init_cfg: OptDynInitCfg<Ms, Mdl, INodes, GMs>,
    /// App configuration available for the entire application lifetime.
    pub cfg: Rc<AppCfg<Ms, Mdl, INodes, GMs>>,
    /// Mutable app state
    pub data: Rc<AppData<Ms, Mdl>>,
}

impl<Ms: 'static, Mdl: 'static, INodes: IntoNodes<Ms>, GMs> ::std::fmt::Debug
    for App<Ms, Mdl, INodes, GMs>
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "App")
    }
}

impl<Ms, Mdl, INodes: IntoNodes<Ms>, GMs> Clone for App<Ms, Mdl, INodes, GMs> {
    fn clone(&self) -> Self {
        Self {
            init_cfg: None,
            cfg: Rc::clone(&self.cfg),
            data: Rc::clone(&self.data),
        }
    }
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetitive sequences of parameters.
impl<Ms, Mdl, INodes: IntoNodes<Ms> + 'static, GMs: 'static> App<Ms, Mdl, INodes, GMs> {
    // @TODO: Relax input function restrictions - init: fn => FnOnce, update & view: FnOnce + Clone.
    // @TODO: Refactor while removing `Builder`.
    /// Create, mount and start the `App`. It's the standard way to create a Seed app.
    ///
    /// _NOTE:_ It tries to hydrate the root element content => you can use it also for prerendered website.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    ///     orders
    ///         .subscribe(Msg::UrlChanged)
    ///         .notify(subs::UrlChanged(url));
    ///
    ///     Model {
    ///         clicks: 0,
    ///     }
    /// }
    ///
    ///fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    ///   match msg {
    ///       Msg::Clicked => model.clicks += 1,
    ///   }
    ///}
    ///
    ///fn view(model: &Model) -> impl IntoNodes<Msg> {
    ///   button![
    ///       format!("Clicked: {}", model.clicks),
    ///       ev(Ev::Click, |_| Msg::Clicked),
    ///   ]
    ///}
    ///
    ///#[wasm_bindgen(start)]
    /// pub fn start() {
    ///     // Mount to the root element with id "app".
    ///     // You can pass also `web_sys::Element` or `web_sys::HtmlElement` as a root element.
    ///     // It's NOT recommended to mount into body or into elements which contain scripts.
    ///     App::start("app", init, update, view);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the root element cannot be found.
    ///
    pub fn start(
        root_element: impl GetElement,
        init: InitFn<Ms, Mdl, INodes, GMs>,
        update: UpdateFn<Ms, Mdl, INodes, GMs>,
        view: ViewFn<Mdl, INodes>,
    ) -> Self {
        // @TODO: Remove as soon as Webkit is fixed and older browsers are no longer in use.
        // https://github.com/seed-rs/seed/issues/241
        // https://bugs.webkit.org/show_bug.cgi?id=202881
        let _ = util::document().query_selector("html");

        // Allows panic messages to output to the browser console.error.
        console_error_panic_hook::set_once();

        let root_element = root_element.get_element().expect("get root element");

        let app_init_cfg = AppInitCfg {
            mount_type: MountType::Takeover,
            phantom: PhantomData,
            into_after_mount: Box::new(
                move |url: Url,
                      orders: &mut OrdersContainer<Ms, Mdl, INodes, GMs>|
                      -> AfterMount<Mdl> {
                    let model = init(url, orders);
                    AfterMount::new(model).url_handling(UrlHandling::None)
                },
            ) as Box<dyn IntoAfterMount<Ms, Mdl, INodes, GMs>>,
        };
        let app = Self::new(
            update,
            None,
            view,
            root_element,
            None,
            None,
            Some(app_init_cfg),
        );
        app.run()
    }

    /// Creates a new `AppBuilder` instance. It's the standard way to create a Seed app.
    ///
    /// Then you can call optional builder methods like `routes` or `sink`.
    /// And you have to call method `build_and_start` to build and run a new `App` instance.
    ///
    /// _NOTE:_ If your `Model` doesn't implement `Default`, you have to call builder method `after_mount`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg, GMsg>) {
    ///   match msg {
    ///       Msg::Clicked => model.clicks += 1,
    ///   }
    ///}
    ///
    ///fn view(model: &Model) -> impl IntoNodes<Msg> {
    ///   vec![
    ///       button![
    ///           format!("Clicked: {}", model.clicks),
    ///           simple_ev(Ev::Click, Msg::Clicked),
    ///       ],
    ///   ]
    ///}
    ///
    ///App::builder(update, view)
    /// ```
    pub fn builder(
        update: UpdateFn<Ms, Mdl, INodes, GMs>,
        view: ViewFn<Mdl, INodes>,
    ) -> AppBuilder<Ms, Mdl, INodes, GMs, UndefinedInitAPI> {
        // @TODO: Remove as soon as Webkit is fixed and older browsers are no longer in use.
        // https://github.com/David-OConnor/seed/issues/241
        // https://bugs.webkit.org/show_bug.cgi?id=202881
        let _ = util::document().query_selector("html");

        // Allows panic messages to output to the browser console.error.
        console_error_panic_hook::set_once();

        AppBuilder::new(update, view)
    }

    /// This runs whenever the state is changed, ie the user-written update function is called.
    /// It updates the state, and any DOM elements affected by this change.
    /// todo this is where we need to compare against differences and only update nodes affected
    /// by the state change.
    ///
    /// We re-create the whole virtual dom each time (Is there a way around this? Probably not without
    /// knowing what vars the model holds ahead of time), but only edit the rendered, `web_sys` dom
    /// for things that have been changed.
    /// We re-render the virtual DOM on every change, but (attempt to) only change
    /// the actual DOM, via `web_sys`, when we need.
    /// The model stored in inner is the old model; `updated_model` is a newly-calculated one.
    pub fn update(&self, message: Ms) {
        let mut queue: VecDeque<Effect<Ms, GMs>> = VecDeque::new();
        queue.push_front(message.into());
        self.process_effect_queue(queue);
    }

    pub fn notify<SubMs: 'static + Any + Clone>(&self, message: SubMs) {
        let mut queue: VecDeque<Effect<Ms, GMs>> = VecDeque::new();
        queue.push_front(Effect::Notification(Notification::new(message)));
        self.process_effect_queue(queue);
    }

    pub fn notify_with_notification(&self, notification: Notification) {
        let mut queue: VecDeque<Effect<Ms, GMs>> = VecDeque::new();
        queue.push_front(Effect::Notification(notification));
        self.process_effect_queue(queue);
    }

    pub fn sink(&self, g_msg: GMs) {
        let mut queue: VecDeque<Effect<Ms, GMs>> = VecDeque::new();
        queue.push_front(Effect::GMsg(g_msg));
        self.process_effect_queue(queue);
    }

    pub fn process_effect_queue(&self, mut queue: VecDeque<Effect<Ms, GMs>>) {
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
                Effect::Notification(notification) => {
                    let mut new_effects = self.process_queue_notification(&notification);
                    queue.append(&mut new_effects);
                }
            }
        }
    }

    pub fn patch_window_event_handlers(&self) {
        if let Some(window_events) = self.cfg.window_events {
            let new_event_handlers = (window_events)(self.data.model.borrow().as_ref().unwrap());
            let new_manager = EventHandlerManager::with_event_handlers(new_event_handlers);

            let mut old_manager = self.data.window_event_handler_manager.replace(new_manager);
            let mut new_manager = self.data.window_event_handler_manager.borrow_mut();

            new_manager.attach_listeners(util::window(), Some(&mut old_manager), &self.mailbox());
        }
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

    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        update: UpdateFn<Ms, Mdl, INodes, GMs>,
        sink: Option<SinkFn<Ms, Mdl, INodes, GMs>>,
        view: ViewFn<Mdl, INodes>,
        mount_point: Element,
        routes: Option<RoutesFn<Ms>>,
        window_events: Option<WindowEventsFn<Ms, Mdl>>,
        init_cfg: OptDynInitCfg<Ms, Mdl, INodes, GMs>,
    ) -> Self {
        let window = util::window();
        let document = window.document().expect("Can't find the window's document");

        Self {
            init_cfg,
            cfg: Rc::new(AppCfg {
                document,
                mount_point,
                update,
                sink,
                view,
                window_events,
            }),
            data: Rc::new(AppData {
                model: RefCell::new(None),
                // This is filled for the first time in run()
                main_el_vdom: RefCell::new(None),
                popstate_closure: RefCell::new(None),
                hashchange_closure: RefCell::new(None),
                routes: RefCell::new(routes),
                window_event_handler_manager: RefCell::new(EventHandlerManager::new()),
                sub_manager: RefCell::new(SubManager::new()),
                msg_listeners: RefCell::new(Vec::new()),
                scheduled_render_handle: RefCell::new(None),
                after_next_render_callbacks: RefCell::new(Vec::new()),
                render_timestamp: Cell::new(None),
            }),
        }
    }

    /// Bootstrap the dom with the vdom by taking over all children of the mount point and
    /// replacing them with the vdom if requested. Will otherwise ignore the original children of
    /// the mount point.
    fn bootstrap_vdom(&self, mount_type: MountType) -> El<Ms> {
        // "new" name is for consistency with `update` function.
        // this section parent is a placeholder, so we can iterate over children
        // in a way consistent with patching code.
        let mut new = El::empty(Tag::Placeholder);

        // Map the DOM's elements onto the virtual DOM if requested to takeover.
        if mount_type == MountType::Takeover {
            // Construct a vdom from the root element. Subsequently strip the workspace so that we
            // can recreate it later - this is a kind of simple way to avoid missing nodes (but
            // not entirely correct).
            // TODO: 1) Please refer to [issue #277](https://github.com/seed-rs/seed/issues/277)
            let mut dom_nodes: El<Ms> = (&self.cfg.mount_point).into();
            #[cfg(debug_assertions)]
            dom_nodes.warn_about_script_tags();

            dom_nodes.strip_ws_nodes_from_self_and_children();

            // Replace the root dom with a placeholder tag and move the children from the root element
            // to the newly created root. Uses `Placeholder` to mimic update logic.
            new.children = dom_nodes.children;
        }

        // Recreate the needed nodes. Only do this if requested to takeover the mount point since
        // it should only be needed here.
        if mount_type == MountType::Takeover {
            // TODO: Please refer to [issue #277](https://github.com/seed-rs/seed/issues/277)
            virtual_dom_bridge::assign_ws_nodes_to_el(&util::document(), &mut new);

            // Remove all old elements. We'll swap them out with the newly created elements later.
            // This maneuver will effectively allow us to remove everything in the mount and thus
            // takeover the mount point.
            while let Some(child) = self.cfg.mount_point.first_child() {
                self.cfg
                    .mount_point
                    .remove_child(&child)
                    .expect("No problem removing node from parent.");
            }

            // Attach all top-level elements to the mount point if present. This means that we have
            // effectively taken full control of everything within the mounting element.
            for child in &mut new.children {
                match child {
                    Node::Element(child_el) => {
                        virtual_dom_bridge::attach_el_and_children(
                            child_el,
                            &self.cfg.mount_point,
                            &self.mailbox(),
                        );
                    }
                    Node::Text(top_child_text) => {
                        virtual_dom_bridge::attach_text_node(top_child_text, &self.cfg.mount_point);
                    }
                    Node::Empty => (),
                }
            }
        }

        new
    }

    fn process_queue_notification(&self, notification: &Notification) -> VecDeque<Effect<Ms, GMs>> {
        self.data
            .sub_manager
            .borrow()
            .notify(notification)
            .into_iter()
            .map(Effect::Msg)
            .collect()
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

        self.patch_window_event_handlers();

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

        self.patch_window_event_handlers();

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

    fn schedule_render(&self) {
        let mut scheduled_render_handle = self.data.scheduled_render_handle.borrow_mut();

        if scheduled_render_handle.is_none() {
            let cb = Closure::new(enclose!((self => s) move |_| {
                s.data.scheduled_render_handle.borrow_mut().take();
                s.rerender_vdom();
            }));

            *scheduled_render_handle = Some(util::request_animation_frame(cb));
        }
    }

    fn cancel_scheduled_render(&self) {
        // Cancel animation frame request by dropping it.
        self.data.scheduled_render_handle.borrow_mut().take();
    }

    fn rerender_vdom(&self) {
        let new_render_timestamp = window().performance().expect("get `Performance`").now();

        // Create a new vdom: The top element, and all its children. Does not yet
        // have associated web_sys elements.
        let mut new = El::empty(Tag::Placeholder);
        new.children = (self.cfg.view)(self.data.model.borrow().as_ref().unwrap()).into_nodes();

        let old = self
            .data
            .main_el_vdom
            .borrow_mut()
            .take()
            .expect("missing main_el_vdom");

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

        // Execute `after_next_render_callbacks`.

        let old_render_timestamp = self
            .data
            .render_timestamp
            .replace(Some(new_render_timestamp));

        let timestamp_delta = old_render_timestamp.map(|old_render_timestamp| {
            RenderTimestampDelta::new(new_render_timestamp - old_render_timestamp)
        });

        self.process_effect_queue(
            self.data
                .after_next_render_callbacks
                .replace(Vec::new())
                .into_iter()
                .filter_map(|callback| callback(timestamp_delta).map(Effect::Msg))
                .collect(),
        );
    }

    fn mailbox(&self) -> Mailbox<Ms> {
        Mailbox::new(enclose!((self => s) move |option_message| {
            if let Some(message) = option_message {
                s.update(message);
            } else {
                s.rerender_vdom();
            }
        }))
    }

    #[deprecated(
        since = "0.5.0",
        note = "Use `builder` with `AppBuilder::{after_mount, before_mount}` instead."
    )]
    pub fn build(
        init: impl FnOnce(Url, &mut OrdersContainer<Ms, Mdl, INodes, GMs>) -> Init<Mdl> + 'static,
        update: UpdateFn<Ms, Mdl, INodes, GMs>,
        view: ViewFn<Mdl, INodes>,
    ) -> InitAppBuilder<Ms, Mdl, INodes, GMs> {
        Self::builder(update, view).init(Box::new(init))
    }

    /// App initialization: Collect its fundamental components, setup, and perform
    /// an initial render.
    #[deprecated(
        since = "0.4.2",
        note = "Please use `AppBuilder.build_and_start` instead"
    )]
    pub fn run(mut self) -> Self {
        let AppInitCfg {
            mount_type,
            into_after_mount,
            ..
        } = self.init_cfg.take().expect(
            "`init_cfg` should be set in `App::new` which is called from `AppBuilder::build_and_start`",
        );

        // Bootstrap the virtual DOM.
        self.data
            .main_el_vdom
            .replace(Some(self.bootstrap_vdom(mount_type)));

        let mut orders = OrdersContainer::new(self.clone());
        let AfterMount {
            model,
            url_handling,
        } = into_after_mount.into_after_mount(url::current(), &mut orders);

        self.data.model.replace(Some(model));

        match url_handling {
            UrlHandling::PassToRoutes => {
                let url = url::current();

                self.notify(subs::UrlChanged(url.clone()));

                let routing_msg = self
                    .data
                    .routes
                    .borrow()
                    .as_ref()
                    .and_then(|routes| routes(url));
                if let Some(routing_msg) = routing_msg {
                    orders.effects.push_back(routing_msg.into());
                }
            }
            UrlHandling::None => (),
        };

        self.patch_window_event_handlers();

        // Update the state on page load, based
        // on the starting URL. Must be set up on the server as well.
        let routes = *self.data.routes.borrow();
        routing::setup_popstate_listener(
            enc!((self => s) move |msg| s.update(msg)),
            enc!((self => s) move |closure| {
                s.data.popstate_closure.replace(Some(closure));
            }),
            enc!((self => s) move |notification| s.notify_with_notification(notification)),
            routes,
        );
        routing::setup_hashchange_listener(
            enc!((self => s) move |msg| s.update(msg)),
            enc!((self => s) move |closure| {
                s.data.hashchange_closure.replace(Some(closure));
            }),
            enc!((self => s) move |notification| s.notify_with_notification(notification)),
            routes,
        );
        routing::setup_link_listener(
            enc!((self => s) move |msg| s.update(msg)),
            enc!((self => s) move |notification| s.notify_with_notification(notification)),
            routes,
        );

        self.process_effect_queue(orders.effects);
        // TODO: In the future, only run the following line if the above statement:
        //  - didn't force-rerender vdom
        //  - didn't schedule render
        //  - doesn't want to skip render

        self.rerender_vdom();

        self
    }
}

#[deprecated(since = "0.5.0", note = "Part of the old Init API.")]
type InitAppBuilder<Ms, Mdl, INodes, GMs> = AppBuilder<
    Ms,
    Mdl,
    INodes,
    GMs,
    MountPointInitInitAPI<UndefinedMountPoint, BuilderInitFn<Ms, Mdl, INodes, GMs>>,
>;
