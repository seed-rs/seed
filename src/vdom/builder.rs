use web_sys::Element;

use crate::{
    dom_types::View,
    orders::OrdersContainer,
    routing, util,
    vdom::{
        alias::*,
        App,
    },
};

pub mod init;
pub use init::{Init, InitFn};
pub mod before_mount;
pub use before_mount::{MountPoint, MountType};
pub mod after_mount;
pub use after_mount::UrlHandling;

/// Used to create and store initial app configuration, ie items passed by the app creator
pub struct Builder<Ms: 'static, Mdl: 'static, ElC: View<Ms>, GMs> {
    init: InitFn<Ms, Mdl, ElC, GMs>,
    update: UpdateFn<Ms, Mdl, ElC, GMs>,
    sink: Option<SinkFn<Ms, Mdl, ElC, GMs>>,
    view: ViewFn<Mdl, ElC>,
    mount_point: Option<Element>,
    routes: Option<RoutesFn<Ms>>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
}

impl<Ms, Mdl, ElC: View<Ms> + 'static, GMs: 'static> Builder<Ms, Mdl, ElC, GMs> {
    /// Constructs the Builder.
    pub(super) fn new(
        init: InitFn<Ms, Mdl, ElC, GMs>,
        update: UpdateFn<Ms, Mdl, ElC, GMs>,
        view: ViewFn<Mdl, ElC>,
    ) -> Self {
        Self {
            init,
            update,
            sink: None,
            view,
            mount_point: None,
            routes: None,
            window_events: None,
        }
    }

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
        // @TODO: Remove as soon as Webkit is fixed and older browsers are no longer in use.
        // https://github.com/seed-rs/seed/issues/241
        // https://bugs.webkit.org/show_bug.cgi?id=202881
        let _ = util::document().query_selector("html");

        self.mount_point = Some(mount_point.element());
        self
    }

    /// Registers a function which maps URLs to messages.
    pub fn routes(mut self, routes: RoutesFn<Ms>) -> Self {
        self.routes = Some(routes);
        self
    }

    /// Registers a function which decides how window events will be handled.
    pub fn window_events(mut self, evts: WindowEvents<Ms, Mdl>) -> Self {
        self.window_events = Some(evts);
        self
    }

    /// Registers a sink function.
    ///
    /// The sink function is a function which can update the model based
    /// on global messages. Consider to use a sink function when a
    /// submodule needs to trigger changes in other modules.
    pub fn sink(mut self, sink: SinkFn<Ms, Mdl, ElC, GMs>) -> Self {
        self.sink = Some(sink);
        self
    }

    /// Turn this [`Builder`] into an [`App`] which is ready to run.
    ///
    /// [`Builder`]: struct.Builder.html
    /// [`App`]: struct.App.html
    #[deprecated(since = "0.4.2", note = "Please use `.build_and_start` instead")]
    pub fn finish(mut self) -> App<Ms, Mdl, ElC, GMs> {
        if self.mount_point.is_none() {
            self = self.mount("app")
        }

        let app = App::new(
            self.update,
            self.sink,
            self.view,
            self.mount_point.unwrap(),
            self.routes,
            self.window_events,
        );

        let mut initial_orders = OrdersContainer::new(app.clone());
        let mut init = (self.init)(routing::current_url(), &mut initial_orders);

        match init.url_handling {
            UrlHandling::PassToRoutes => {
                let url = routing::current_url();
                if let Some(r) = self.routes {
                    if let Some(u) = r(url) {
                        (self.update)(u, &mut init.model, &mut initial_orders);
                    }
                }
            }
            UrlHandling::None => (),
        };

        app.cfg.initial_orders.replace(Some(initial_orders));
        app.cfg.mount_type.replace(Some(init.mount_type));
        app.data.model.replace(Some(init.model));

        app
    }

    /// Build and run the app.
    pub fn build_and_start(self) -> App<Ms, Mdl, ElC, GMs> {
        let app = self.finish();
        app.run()
    }
}
