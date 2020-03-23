use super::{types::*, App, AppInitCfg, OrdersContainer};
use crate::browser::{url, Url};
use crate::virtual_dom::IntoNodes;
use std::marker::PhantomData;

pub mod after_mount;
pub mod before_mount;
pub mod init;

pub use after_mount::{AfterMount, IntoAfterMount, UndefinedAfterMount, UrlHandling};
pub use before_mount::{BeforeMount, MountPoint, MountType, UndefinedMountPoint};
pub use init::{IntoInit, UndefinedInitAPI, UndefinedIntoInit};

#[deprecated(
    since = "0.5.0",
    note = "Used for compatibility with old Init API. Use `BeforeAfterInitAPI` together with `BeforeMount` and `AfterMount` instead."
)]
pub struct MountPointInitInitAPI<MP, II> {
    mount_point: MP,
    into_init: II,
}
// TODO Remove when removing the other `InitAPI`s.
pub struct BeforeAfterInitAPI<IAM> {
    before_mount_handler: Box<dyn FnOnce(Url) -> BeforeMount>,
    into_after_mount: IAM,
}
// TODO Remove when removing the other `InitAPI`s.
impl Default for BeforeAfterInitAPI<UndefinedAfterMount> {
    fn default() -> Self {
        BeforeAfterInitAPI {
            before_mount_handler: Box::new(|_| BeforeMount::default()),
            into_after_mount: UndefinedAfterMount,
        }
    }
}

// TODO Remove when removing the other `InitAPI`s.
pub trait InitAPI<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs> {
    type Builder;
    fn build(builder: Self::Builder) -> App<Ms, Mdl, INodes, GMs>;
}

// TODO Remove when removing the other `InitAPI`s.
pub trait InitAPIData {
    type IntoAfterMount;
    #[deprecated(
        since = "0.5.0",
        note = "Used for compatibility with old Init API. Use `IntoBeforeMount` and `IntoAfterMount` instead."
    )]
    type IntoInit;
    #[deprecated(
        since = "0.5.0",
        note = "Used for compatibility with old Init API. Use `IntoBeforeMount` and `IntoAfterMount` instead."
    )]
    type MountPoint;

    fn before_mount(
        self,
        before_mount_handler: Box<dyn FnOnce(Url) -> BeforeMount>,
    ) -> BeforeAfterInitAPI<Self::IntoAfterMount>;
    fn after_mount<
        Ms: 'static,
        Mdl,
        INodes: IntoNodes<Ms>,
        GMs,
        NewIAM: IntoAfterMount<Ms, Mdl, INodes, GMs>,
    >(
        self,
        into_after_mount: NewIAM,
    ) -> BeforeAfterInitAPI<NewIAM>;

    #[deprecated(
        since = "0.5.0",
        note = "Used for compatibility with old Init API. Use `before_mount` and `after_mount` instead."
    )]
    fn init<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs, NewII: IntoInit<Ms, Mdl, INodes, GMs>>(
        self,
        into_init: NewII,
    ) -> MountPointInitInitAPI<Self::MountPoint, NewII>;
    #[deprecated(
        since = "0.5.0",
        note = "Used for compatibility with old Init API. Use `before_mount` and `after_mount` instead."
    )]
    fn mount<NewMP: MountPoint>(
        self,
        mount_point: NewMP,
    ) -> MountPointInitInitAPI<NewMP, Self::IntoInit>;
}

// TODO Remove when removing the other `InitAPI`s.
#[deprecated(
    since = "0.5.0",
    note = "Used for compatibility with old Init API. Use `BeforeAfterInitAPI` together with `BeforeMount` and `AfterMount` instead."
)]
impl<
        Ms: 'static,
        Mdl: 'static,
        INodes: 'static + IntoNodes<Ms>,
        GMs: 'static,
        MP: MountPoint,
        II: IntoInit<Ms, Mdl, INodes, GMs>,
    > InitAPI<Ms, Mdl, INodes, GMs> for MountPointInitInitAPI<MP, II>
{
    type Builder = Builder<Ms, Mdl, INodes, GMs, Self>;
    fn build(builder: Self::Builder) -> App<Ms, Mdl, INodes, GMs> {
        let MountPointInitInitAPI {
            into_init,
            mount_point,
        } = builder.init_api;

        let mut app = App::new(
            builder.update,
            builder.sink,
            builder.view,
            mount_point.element_getter()(),
            builder.routes,
            builder.window_events,
            None,
        );

        let mut initial_orders = OrdersContainer::new(app.clone());
        let init = into_init.into_init(url::current(), &mut initial_orders);

        app.init_cfg.replace(AppInitCfg {
            mount_type: init.mount_type,
            into_after_mount: Box::new((init, initial_orders)),
            phantom: PhantomData,
        });

        app
    }
}
// TODO Remove when removing the other `InitAPI`s.
impl<
        Ms: 'static,
        Mdl: 'static,
        INodes: 'static + IntoNodes<Ms>,
        GMs: 'static,
        IAM: 'static + IntoAfterMount<Ms, Mdl, INodes, GMs>,
    > InitAPI<Ms, Mdl, INodes, GMs> for BeforeAfterInitAPI<IAM>
{
    type Builder = Builder<Ms, Mdl, INodes, GMs, Self>;
    fn build(builder: Self::Builder) -> App<Ms, Mdl, INodes, GMs> {
        let BeforeAfterInitAPI {
            before_mount_handler,
            into_after_mount,
        } = builder.init_api;

        let BeforeMount {
            mount_point_getter,
            mount_type,
        } = before_mount_handler(url::current());

        App::new(
            builder.update,
            builder.sink,
            builder.view,
            mount_point_getter(),
            builder.routes,
            builder.window_events,
            Some(AppInitCfg {
                mount_type,
                into_after_mount: Box::new(into_after_mount),
                phantom: PhantomData,
            }),
        )
    }
}
// TODO Remove when removing the other `InitAPI`s.
impl<Ms: 'static, Mdl: 'static + Default, INodes: 'static + IntoNodes<Ms>, GMs: 'static>
    InitAPI<Ms, Mdl, INodes, GMs> for UndefinedInitAPI
{
    type Builder = Builder<Ms, Mdl, INodes, GMs, Self>;
    fn build(builder: Self::Builder) -> App<Ms, Mdl, INodes, GMs> {
        BeforeAfterInitAPI::build(Builder {
            update: builder.update,
            view: builder.view,

            routes: builder.routes,
            window_events: builder.window_events,
            sink: builder.sink,

            init_api: BeforeAfterInitAPI::default(),
        })
    }
}

#[deprecated(
    since = "0.5.0",
    note = "Used for compatibility with old Init API. Use `BeforeAfterInitAPI` together with `BeforeMount` and `AfterMount` instead."
)]
impl<MP, II> InitAPIData for MountPointInitInitAPI<MP, II> {
    type IntoAfterMount = UndefinedAfterMount;
    type IntoInit = II;
    type MountPoint = MP;

    fn before_mount(
        self,
        before_mount_handler: Box<dyn FnOnce(Url) -> BeforeMount>,
    ) -> BeforeAfterInitAPI<Self::IntoAfterMount> {
        BeforeAfterInitAPI {
            before_mount_handler,
            into_after_mount: UndefinedAfterMount,
        }
    }
    fn after_mount<
        Ms: 'static,
        Mdl,
        INodes: IntoNodes<Ms>,
        GMs,
        NewIAM: IntoAfterMount<Ms, Mdl, INodes, GMs>,
    >(
        self,
        into_after_mount: NewIAM,
    ) -> BeforeAfterInitAPI<NewIAM> {
        BeforeAfterInitAPI {
            into_after_mount,
            before_mount_handler: Box::new(|_| BeforeMount::default()),
        }
    }

    fn init<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs, NewII: IntoInit<Ms, Mdl, INodes, GMs>>(
        self,
        into_init: NewII,
    ) -> MountPointInitInitAPI<Self::MountPoint, NewII> {
        MountPointInitInitAPI {
            into_init,
            mount_point: self.mount_point,
        }
    }
    fn mount<NewMP: MountPoint>(
        self,
        mount_point: NewMP,
    ) -> MountPointInitInitAPI<NewMP, Self::IntoInit> {
        MountPointInitInitAPI {
            mount_point,
            into_init: self.into_init,
        }
    }
}
// TODO Remove when removing the other `InitAPI`s.
impl<IAM> InitAPIData for BeforeAfterInitAPI<IAM> {
    type IntoAfterMount = IAM;
    type IntoInit = UndefinedIntoInit;
    type MountPoint = UndefinedMountPoint;

    fn before_mount(
        self,
        before_mount_handler: Box<dyn FnOnce(Url) -> BeforeMount>,
    ) -> BeforeAfterInitAPI<Self::IntoAfterMount> {
        BeforeAfterInitAPI {
            before_mount_handler,
            into_after_mount: self.into_after_mount,
        }
    }
    fn after_mount<
        Ms: 'static,
        Mdl,
        INodes: IntoNodes<Ms>,
        GMs,
        NewIAM: IntoAfterMount<Ms, Mdl, INodes, GMs>,
    >(
        self,
        into_after_mount: NewIAM,
    ) -> BeforeAfterInitAPI<NewIAM> {
        BeforeAfterInitAPI {
            into_after_mount,
            before_mount_handler: self.before_mount_handler,
        }
    }

    fn init<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs, NewII: IntoInit<Ms, Mdl, INodes, GMs>>(
        self,
        into_init: NewII,
    ) -> MountPointInitInitAPI<Self::MountPoint, NewII> {
        MountPointInitInitAPI {
            into_init,
            mount_point: UndefinedMountPoint,
        }
    }
    fn mount<NewMP: MountPoint>(
        self,
        mount_point: NewMP,
    ) -> MountPointInitInitAPI<NewMP, Self::IntoInit> {
        MountPointInitInitAPI {
            mount_point,
            into_init: UndefinedIntoInit,
        }
    }
}
// TODO Remove when removing the other `InitAPI`s.
impl InitAPIData for UndefinedInitAPI {
    type IntoAfterMount = UndefinedAfterMount;
    type IntoInit = UndefinedIntoInit;
    type MountPoint = UndefinedMountPoint;

    fn before_mount(
        self,
        before_mount_handler: Box<dyn FnOnce(Url) -> BeforeMount>,
    ) -> BeforeAfterInitAPI<Self::IntoAfterMount> {
        BeforeAfterInitAPI {
            before_mount_handler,
            into_after_mount: UndefinedAfterMount,
        }
    }
    fn after_mount<
        Ms: 'static,
        Mdl,
        INodes: IntoNodes<Ms>,
        GMs,
        NewIAM: IntoAfterMount<Ms, Mdl, INodes, GMs>,
    >(
        self,
        into_after_mount: NewIAM,
    ) -> BeforeAfterInitAPI<NewIAM> {
        BeforeAfterInitAPI {
            into_after_mount,
            before_mount_handler: Box::new(|_| BeforeMount::default()),
        }
    }

    fn init<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs, NewII: IntoInit<Ms, Mdl, INodes, GMs>>(
        self,
        into_init: NewII,
    ) -> MountPointInitInitAPI<Self::MountPoint, NewII> {
        MountPointInitInitAPI {
            into_init,
            mount_point: UndefinedMountPoint,
        }
    }
    fn mount<NewMP: MountPoint>(
        self,
        mount_point: NewMP,
    ) -> MountPointInitInitAPI<NewMP, Self::IntoInit> {
        MountPointInitInitAPI {
            mount_point,
            into_init: UndefinedIntoInit,
        }
    }
}

/// Used to create and store initial app configuration, ie items passed by the app creator.
pub struct Builder<Ms: 'static, Mdl: 'static, INodes: IntoNodes<Ms>, GMs, InitAPIType> {
    update: UpdateFn<Ms, Mdl, INodes, GMs>,
    view: ViewFn<Mdl, INodes>,

    routes: Option<RoutesFn<Ms>>,
    window_events: Option<WindowEventsFn<Ms, Mdl>>,
    sink: Option<SinkFn<Ms, Mdl, INodes, GMs>>,

    // TODO: Remove when removing legacy init fields.
    init_api: InitAPIType,
}

impl<Ms, Mdl, INodes: IntoNodes<Ms> + 'static, GMs: 'static>
    Builder<Ms, Mdl, INodes, GMs, UndefinedInitAPI>
{
    /// Constructs the Builder.
    pub(super) fn new(update: UpdateFn<Ms, Mdl, INodes, GMs>, view: ViewFn<Mdl, INodes>) -> Self {
        Builder {
            update,
            view,

            routes: None,
            window_events: None,
            sink: None,

            init_api: UndefinedInitAPI,
        }
    }
}

impl<
        Ms,
        Mdl,
        INodes: IntoNodes<Ms> + 'static,
        GMs: 'static,
        IAM: 'static,
        MP,
        II,
        InitAPIType: InitAPIData<IntoInit = II, MountPoint = MP, IntoAfterMount = IAM>,
    > Builder<Ms, Mdl, INodes, GMs, InitAPIType>
{
    #[deprecated(
        since = "0.5.0",
        note = "Used for compatibility with old Init API. Use `before_mount` and `after_mount` instead."
    )]
    pub fn init<NewII: IntoInit<Ms, Mdl, INodes, GMs>>(
        self,
        new_init: NewII,
    ) -> Builder<Ms, Mdl, INodes, GMs, MountPointInitInitAPI<MP, NewII>> {
        Builder {
            update: self.update,
            view: self.view,

            routes: self.routes,
            window_events: self.window_events,
            sink: self.sink,

            init_api: self.init_api.init(new_init),
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
    #[deprecated(
        since = "0.5.0",
        note = "Used for compatibility with old Init API. Use `before_mount` and `after_mount` instead."
    )]
    pub fn mount<NewMP: MountPoint>(
        self,
        new_mount_point: NewMP,
    ) -> Builder<Ms, Mdl, INodes, GMs, MountPointInitInitAPI<NewMP, II>> {
        Builder {
            update: self.update,
            view: self.view,

            routes: self.routes,
            window_events: self.window_events,
            sink: self.sink,

            init_api: self.init_api.mount(new_mount_point),
        }
    }

    /// Select HTML element where the app will be mounted and how it'll be mounted.
    ///
    /// See `BeforeMount::mount_point` and `BeforeMount::mount_type` docs for more info.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn before_mount(_url: Url) -> BeforeMount {
    ///    BeforeMount::new()
    ///        .mount_point("main")
    ///        .mount_type(MountType::Takeover)
    ///}
    /// ```
    pub fn before_mount(
        self,
        before_mount: impl FnOnce(Url) -> BeforeMount + 'static,
    ) -> Builder<Ms, Mdl, INodes, GMs, BeforeAfterInitAPI<IAM>> {
        Builder {
            update: self.update,
            view: self.view,

            routes: self.routes,
            window_events: self.window_events,
            sink: self.sink,

            init_api: self.init_api.before_mount(Box::new(before_mount)),
        }
    }

    /// You can create your `Model` and handle initial URL in this method.
    ///
    /// See `AfterMount::url_handling` for more info about initial URL handling.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn after_mount(_url: Url, _orders: &mut impl Orders<Msg, GMsg>) -> AfterMount<Model> {
    ///    let model = Model { clicks: 0 };
    ///    AfterMount::new(model).url_handling(UrlHandling::None)
    ///}
    /// ```
    pub fn after_mount<AM: 'static + IntoAfterMount<Ms, Mdl, INodes, GMs>>(
        self,
        after_mount: AM,
    ) -> Builder<Ms, Mdl, INodes, GMs, BeforeAfterInitAPI<AM>> {
        Builder {
            update: self.update,
            view: self.view,

            routes: self.routes,
            window_events: self.window_events,
            sink: self.sink,

            init_api: self.init_api.after_mount(after_mount),
        }
    }

    /// Registers a function which maps URLs to messages.
    ///
    /// When you return `None`, Seed doesn't call your `update` function
    /// and also doesn't push the new route or prevent page refresh.
    /// It's useful if the user clicked on a link and Seed shouldn't intercept it,
    /// because it's e.g. a download link.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn routes(url: Url) -> Option<Msg> {
    ///    Some(Msg::UrlChanged(url))
    ///}
    /// ```
    pub fn routes(mut self, routes: RoutesFn<Ms>) -> Self {
        self.routes = Some(routes);
        self
    }

    /// Registers a function which decides how window events will be handled.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
    ///    vec![keyboard_ev(Ev::KeyDown, Msg::KeyPressed)]
    ///}
    /// ```
    pub fn window_events(mut self, window_events: WindowEventsFn<Ms, Mdl>) -> Self {
        self.window_events = Some(window_events);
        self
    }

    /// Registers a sink function.
    ///
    /// The sink function is a function which can update the model based
    /// on global messages. Consider to use a sink function when a
    /// submodule needs to trigger changes in other modules.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn sink(g_msg: GMsg, _model: &mut Model, _orders: &mut impl Orders<Msg, GMsg>) {
    ///    match g_msg {
    ///        GMsg::SayHello => log!("Hello!"),
    ///    }
    ///}
    /// ```
    pub fn sink(mut self, sink: SinkFn<Ms, Mdl, INodes, GMs>) -> Self {
        self.sink = Some(sink);
        self
    }
}

impl<
        Ms: 'static,
        Mdl,
        INodes: IntoNodes<Ms> + 'static,
        GMs: 'static,
        InitAPIType: InitAPI<Ms, Mdl, INodes, GMs, Builder = Self>,
    > Builder<Ms, Mdl, INodes, GMs, InitAPIType>
{
    /// Build, mount and start the app.
    pub fn build_and_start(self) -> App<Ms, Mdl, INodes, GMs> {
        InitAPIType::build(self).run()
    }
}

impl<
        Ms: 'static,
        Mdl,
        INodes: IntoNodes<Ms> + 'static,
        GMs: 'static,
        MP: MountPoint,
        II: IntoInit<Ms, Mdl, INodes, GMs>,
    > Builder<Ms, Mdl, INodes, GMs, MountPointInitInitAPI<MP, II>>
{
    /// Turn this [`Builder`] into an [`App`] which is ready to run.
    ///
    /// [`Builder`]: struct.Builder.html
    /// [`App`]: struct.App.html
    #[deprecated(since = "0.4.2", note = "Please use `.build_and_start` instead")]
    pub fn finish(self) -> App<Ms, Mdl, INodes, GMs> {
        MountPointInitInitAPI::build(self)
    }
}
