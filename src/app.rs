use crate::browser::{
    service::routing,
    util::{self, window, ClosureNew},
    Url, DUMMY_BASE_URL,
};
use crate::virtual_dom::{patch, El, EventHandlerManager, IntoNodes, Mailbox, Tag};
use enclose::enclose;
use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::Rc,
};
use types::{UpdateFn, ViewFn};
use wasm_bindgen::closure::Closure;
use web_sys::Element;

pub mod cfg;
pub mod cmd_manager;
pub mod cmds;
pub mod data;
pub mod effects;
pub mod get_element;
pub mod message_mapper;
pub mod orders;
pub mod render_info;
pub mod stream_manager;
pub mod streams;
pub mod sub_manager;
pub mod subs;
pub mod types;

pub use cfg::{AppCfg, AppInitCfg};
pub use cmd_manager::{CmdHandle, CmdManager};
pub use data::AppData;
pub use effects::Effect;
pub use get_element::GetElement;
pub use message_mapper::MessageMapper;
pub use orders::{Orders, OrdersContainer, OrdersProxy};
pub use render_info::RenderInfo;
pub use stream_manager::{StreamHandle, StreamManager};
pub use sub_manager::{Notification, SubHandle, SubManager};

pub struct UndefinedGMsg;

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
    /// App configuration available for the entire application lifetime.
    pub cfg: Rc<AppCfg<Ms, Mdl, INodes, GMs>>,
    /// Mutable app state.
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
    #[allow(clippy::too_many_lines)]
    pub fn start(
        root_element: impl GetElement,
        init: impl FnOnce(Url, &mut OrdersContainer<Ms, Mdl, INodes, GMs>) -> Mdl + 'static,
        update: UpdateFn<Ms, Mdl, INodes, GMs>,
        view: ViewFn<Mdl, INodes>,
    ) -> Self {
        // This function looks to be a significant sticking point. It will possibly end
        // up being a relatively major effort to get it cleaned up.

        // @TODO: Remove as soon as Webkit is fixed and older browsers are no longer in use.
        // https://github.com/seed-rs/seed/issues/241
        // https://bugs.webkit.org/show_bug.cgi?id=202881
        let _ = util::document().query_selector("html");

        // Allows panic messages to output to the browser console.error.
        console_error_panic_hook::set_once();

        let base_path: Rc<Vec<String>> = Rc::new(
            util::document()
                .query_selector("base")
                .expect("query element with 'base' tag")
                .and_then(|element| element.get_attribute("href"))
                .and_then(|href| web_sys::Url::new_with_base(&href, DUMMY_BASE_URL).ok())
                .map(|url| {
                    url.pathname()
                        .trim_matches('/')
                        .split('/')
                        .map(ToOwned::to_owned)
                        .collect()
                })
                .unwrap_or_default(),
        );

        let root_element = root_element.get_element().expect("get root element");
        let app = Self::new(update, view, root_element, base_path.clone());

        // Bootstrap the virtual DOM.
        let mut orders = OrdersContainer::new(app.clone());
        let model = init(Url::current().skip_base_path(&base_path), &mut orders);

        app.data.model.replace(Some(model));
        app.data
            .main_el_vdom
            .replace(Some(El::empty(Tag::Placeholder)));

        // Update the state on page load, based
        // on the starting URL. Must be set up on the server as well.
        // let routes = *app.data.routes.borrow();
        routing::setup_popstate_listener(
            enclose!((app => s) move |closure| {
                s.data.popstate_closure.replace(Some(closure));
            }),
            enclose!((app => s) move |notification| s.notify_with_notification(notification)),
            Rc::clone(&app.cfg.base_path),
        );
        routing::setup_hashchange_listener(
            enclose!((app => s) move |closure| {
                s.data.hashchange_closure.replace(Some(closure));
            }),
            enclose!((app => s) move |notification| s.notify_with_notification(notification)),
            Rc::clone(&app.cfg.base_path),
        );
        routing::setup_link_listener(
            enclose!((app => s) move |notification| s.notify_with_notification(notification)),
        );

        orders.subscribe(enclose!((app => s) move |url_requested| {
            routing::url_request_handler(
                url_requested,
                Rc::clone(&s.cfg.base_path),
                move |notification| s.notify_with_notification(notification),
            )
        }));

        app.process_effect_queue(orders.effects);
        // TODO: In the future, only run the following line if the above statement:
        //  - didn't force-rerender vdom
        //  - didn't schedule render
        //  - doesn't want to skip render

        app.rerender_vdom();

        app
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
                Effect::GMsg(_) => self.schedule_render(),
                Effect::Notification(notification) => {
                    let mut new_effects = self.process_queue_notification(&notification);
                    queue.append(&mut new_effects);
                }
            }
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
        view: ViewFn<Mdl, INodes>,
        mount_point: Element,
        base_path: Rc<Vec<String>>,
    ) -> Self {
        let window = util::window();
        let document = window.document().expect("get window's document");

        Self {
            cfg: Rc::new(AppCfg {
                document,
                mount_point,
                update,
                view,
                base_path,
            }),
            data: Rc::new(AppData {
                model: RefCell::new(None),
                // This is filled for the first time in run()
                main_el_vdom: RefCell::new(None),
                popstate_closure: RefCell::new(None),
                hashchange_closure: RefCell::new(None),
                window_event_handler_manager: RefCell::new(EventHandlerManager::new()),
                sub_manager: RefCell::new(SubManager::new()),
                msg_listeners: RefCell::new(Vec::new()),
                scheduled_render_handle: RefCell::new(None),
                after_next_render_callbacks: RefCell::new(Vec::new()),
                render_info: Cell::new(None),
            }),
        }
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
        // TODO: schedule_render already gets the window, possibly doubling up
        // this call. DOM calls can be expensive, so it's probably worth sorting
        // that out.
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

        let render_info = match self.data.render_info.take() {
            Some(old_render_info) => RenderInfo {
                timestamp: new_render_timestamp,
                timestamp_delta: Some(new_render_timestamp - old_render_info.timestamp),
            },
            None => RenderInfo {
                timestamp: new_render_timestamp,
                timestamp_delta: None,
            },
        };
        self.data.render_info.set(Some(render_info));

        self.process_effect_queue(
            self.data
                .after_next_render_callbacks
                .replace(Vec::new())
                .into_iter()
                .filter_map(|callback| callback(render_info).map(Effect::Msg))
                .collect(),
        );
    }

    pub fn mailbox(&self) -> Mailbox<Ms> {
        Mailbox::new(enclose!((self => s) move |option_message| {
            if let Some(message) = option_message {
                s.update(message);
            } else {
                s.rerender_vdom();
            }
        }))
    }
}
