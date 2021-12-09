#![allow(clippy::module_name_repetitions)]

use crate::browser::dom::virtual_dom_bridge;
use crate::browser::{
    service::routing,
    util::{self, window, ClosureNew},
    Url, DUMMY_BASE_URL,
};
use crate::virtual_dom::{patch, El, EventHandlerManager, IntoNodes, Mailbox, Node, Tag};
use enclose::{enc, enclose};
use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::VecDeque,
    fmt,
    rc::Rc,
};
use sub_manager::SubManager;
use wasm_bindgen::closure::Closure;

pub mod cfg;
pub mod cmd_manager;
pub mod cmds;
pub mod data;
mod effect;
pub mod get_element;
pub mod message_mapper;
pub mod orders;
pub mod render_info;
pub mod stream_manager;
pub mod streams;
pub mod sub_manager;
pub mod subs;

pub use cfg::AppCfg;
pub use cmd_manager::CmdHandle;
pub(crate) use data::AppData;
use effect::Effect;
pub use get_element::GetElement;
pub use message_mapper::MessageMapper;
pub use orders::{Orders, OrdersContainer, OrdersProxy};
pub use render_info::RenderInfo;
pub use stream_manager::StreamHandle;
pub use sub_manager::{Notification, SubHandle};

/// Determines if an update should cause the `VDom` to rerender or not.
pub enum ShouldRender {
    Render,
    ForceRenderNow,
    Skip,
}

pub struct App<Ms, Mdl, INodes>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    /// App configuration.
    cfg: Rc<AppCfg<Ms, Mdl, INodes>>,
    /// Mutable app state.
    data: Rc<AppData<Ms, Mdl>>,
}

impl<Ms, Mdl, INodes> fmt::Debug for App<Ms, Mdl, INodes>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> fmt::Result {
        write!(f, "App")
    }
}

impl<Ms, Mdl, INodes> Clone for App<Ms, Mdl, INodes>
where
    INodes: IntoNodes<Ms>,
{
    fn clone(&self) -> Self {
        Self {
            cfg: Rc::clone(&self.cfg),
            data: Rc::clone(&self.data),
        }
    }
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetitive sequences of parameters.
impl<Ms, Mdl, INodes> App<Ms, Mdl, INodes>
where
    INodes: IntoNodes<Ms> + 'static,
{
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
    // pub type UpdateFn<Ms, Mdl, INodes> = fn(Ms, &mut Mdl, &mut OrdersContainer<Ms, Mdl, INodes>);
    pub fn start(
        root_element: impl GetElement,
        init: impl FnOnce(Url, &mut OrdersContainer<Ms, Mdl, INodes>) -> Mdl + 'static,
        update: impl FnOnce(Ms, &mut Mdl, &mut OrdersContainer<Ms, Mdl, INodes>) + Clone + 'static,
        view: impl FnOnce(&Mdl) -> INodes + Clone + 'static,
    ) -> Self {
        // @TODO: Remove as soon as Webkit is fixed and older browsers are no longer in use.
        // https://github.com/seed-rs/seed/issues/241
        // https://bugs.webkit.org/show_bug.cgi?id=202881
        std::mem::drop(util::document().query_selector("html"));

        // Allows panic messages to output to the browser console.error.
        #[cfg(feature = "panic-hook")]
        console_error_panic_hook::set_once();

        let base_path: Rc<[String]> = Rc::from(
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
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
                .as_slice(),
        );

        let app = Self {
            cfg: Rc::new(AppCfg {
                document: util::window().document().expect("get window's document"),
                mount_point: root_element.get_element().expect("get root element"),
                update: Box::new(move |msg, model, orders| update.clone()(msg, model, orders)),
                view: Box::new(move |model| view.clone()(model)),
                base_path,
            }),
            data: Rc::new(AppData {
                model: RefCell::new(None),
                root_el: RefCell::new(None),
                popstate_closure: RefCell::new(None),
                hashchange_closure: RefCell::new(None),
                window_event_handler_manager: RefCell::new(EventHandlerManager::new()),
                sub_manager: RefCell::new(SubManager::new()),
                msg_listeners: RefCell::new(Vec::new()),
                scheduled_render_handle: RefCell::new(None),
                after_next_render_callbacks: RefCell::new(Vec::new()),
                render_info: Cell::new(None),
            }),
        };

        app.data.root_el.replace(Some(app.bootstrap_vdom()));

        let mut orders = OrdersContainer::new(app.clone());

        let new_model = init(
            Url::current().skip_base_path(&Rc::clone(&app.cfg.base_path)),
            &mut orders,
        );
        app.data.model.replace(Some(new_model));

        routing::setup_popstate_listener(
            enc!((app => s) move |closure| {
                s.data.popstate_closure.replace(Some(closure));
            }),
            enc!((app => s) move |notification| s.notify_with_notification(notification)),
            Rc::clone(&app.cfg.base_path),
        );
        routing::setup_link_listener(
            enc!((app => s) move |notification| s.notify_with_notification(notification)),
        );

        orders.subscribe(enc!((app => s) move |url_requested| {
            routing::url_request_handler(
                url_requested,
                Rc::clone(&s.cfg.base_path),
                move |notification| s.notify_with_notification(notification),
            );
        }));

        app.process_effect_queue(orders.effects);
        app.rerender_vdom();
        app
    }

    /// Invoke your `update` function with provided message.
    pub fn update(&self, message: Ms) {
        self.update_with_option(Some(message));
    }

    /// Invoke your `update` function with provided message.
    ///
    /// If the message is `None`, then your `update` won't be invoked,
    /// but rerender will be still scheduled.
    pub fn update_with_option(&self, message: Option<Ms>) {
        let mut queue: VecDeque<Effect<Ms>> = VecDeque::new();
        queue.push_front(Effect::Msg(message));
        self.process_effect_queue(queue);
    }

    pub fn notify<SubMs: 'static + Any + Clone>(&self, message: SubMs) {
        let mut queue: VecDeque<Effect<Ms>> = VecDeque::new();
        queue.push_front(Effect::Notification(Notification::new(message)));
        self.process_effect_queue(queue);
    }

    pub fn notify_with_notification(&self, notification: Notification) {
        let mut queue: VecDeque<Effect<Ms>> = VecDeque::new();
        queue.push_front(Effect::Notification(notification));
        self.process_effect_queue(queue);
    }

    pub(crate) fn process_effect_queue(&self, mut queue: VecDeque<Effect<Ms>>) {
        if std::thread::panicking() {
            return;
        }

        while let Some(effect) = queue.pop_front() {
            match effect {
                Effect::Msg(msg) => {
                    let mut new_effects = self.process_queue_message(msg);
                    queue.append(&mut new_effects);
                }
                Effect::Notification(notification) => {
                    let mut new_effects = self.process_queue_notification(&notification);
                    queue.append(&mut new_effects);
                }
                Effect::TriggeredHandler(handler) => {
                    let mut new_effects = self.process_queue_message(handler());
                    queue.append(&mut new_effects);
                }
            }
        }
    }

    /// Bootstrap the dom at startup with the vdom by taking over all children of the mount point and
    /// replacing them with the vdom.
    fn bootstrap_vdom(&self) -> El<Ms> {
        // "new" name is for consistency with `update` function.
        // this section parent is a placeholder, so we can iterate over children
        // in a way consistent with patching code.
        let mut new = El::empty(Tag::Placeholder);

        // Map the DOM's elements onto the virtual DOM.
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

        // Recreate the needed nodes.
        // TODO: Please refer to [issue #277](https://github.com/seed-rs/seed/issues/277)
        // TODO: Look into how the 0.7 API changes removing of MountType takeover check interact
        // with the uses of `rerender_vdom` and `bootstrap_vdom`
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
                Node::Empty | Node::NoChange => (),
            }
        }

        new
    }

    fn rerender_vdom(&self) {
        if std::thread::panicking() {
            return;
        }

        let new_render_timestamp = window().performance().expect("get `Performance`").now();

        // Create a new vdom: The top element, and all its children. Does not yet
        // have associated web_sys elements.
        let mut new = El::empty(Tag::Placeholder);
        new.children = (self.cfg.view)(self.data.model.borrow().as_ref().unwrap()).into_nodes();

        let old = self
            .data
            .root_el
            .borrow_mut()
            .take()
            .expect("missing root element");

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
        self.data.root_el.borrow_mut().replace(new);

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
                .map(|callback| Effect::TriggeredHandler(Box::new(move || callback(render_info))))
                .collect(),
        );
    }

    fn process_queue_notification(&self, notification: &Notification) -> VecDeque<Effect<Ms>> {
        self.data
            .sub_manager
            .borrow()
            .notify(notification)
            .into_iter()
            .map(Effect::TriggeredHandler)
            .collect()
    }

    fn process_queue_message(&self, message: Option<Ms>) -> VecDeque<Effect<Ms>> {
        let mut orders = OrdersContainer::new(self.clone());

        if let Some(message) = message {
            for l in self.data.msg_listeners.borrow().iter() {
                (l)(&message);
            }

            (self.cfg.update)(
                message,
                self.data.model.borrow_mut().as_mut().unwrap(),
                &mut orders,
            );
        }

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

    pub fn mailbox(&self) -> Mailbox<Ms> {
        Mailbox::new(enclose!((self => s) move |option_message| {
            s.update_with_option(option_message);
        }))
    }
}
