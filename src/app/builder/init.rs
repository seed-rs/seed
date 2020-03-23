use super::{super::OrdersContainer, AfterMount, IntoAfterMount, MountType, UrlHandling};
use crate::browser::Url;
use crate::virtual_dom::IntoNodes;

pub struct UndefinedInitAPI;
#[allow(clippy::module_name_repetitions)]
pub struct UndefinedIntoInit;

/// Used as a flexible wrapper for the init function.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    since = "0.5.0",
    note = "Part of old Init API. Use a combination of `BeforeMount` and `AfterMount` instead."
)]
pub struct Init<Mdl> {
    /// Initial model to be used when the app begins.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub model: Mdl,
    /// How to handle initial url routing. Defaults to [`UrlHandling::PassToRoutes`] in the
    /// constructors.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub url_handling: UrlHandling,
    /// How to handle elements already present in the mount. Defaults to [`MountType::Append`]
    /// in the constructors.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `BeforeMount` instead."
    )]
    pub mount_type: MountType,
}

impl<Mdl> Init<Mdl> {
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub const fn new(model: Mdl) -> Self {
        Self {
            model,
            url_handling: UrlHandling::PassToRoutes,
            mount_type: MountType::Append,
        }
    }

    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub const fn new_with_url_handling(model: Mdl, url_handling: UrlHandling) -> Self {
        Self {
            model,
            url_handling,
            mount_type: MountType::Append,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[deprecated(
    since = "0.5.0",
    note = "Part of old Init API. Use `AfterMount` instead."
)]
pub type InitFn<Ms, Mdl, INodes, GMs> =
    Box<dyn FnOnce(Url, &mut OrdersContainer<Ms, Mdl, INodes, GMs>) -> Init<Mdl>>;

#[allow(clippy::module_name_repetitions)]
#[deprecated(
    since = "0.5.0",
    note = "Part of old Init API. Use `IntoAfterMount` and `IntoBeforeMount` instead."
)]
pub trait IntoInit<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs> {
    fn into_init(self, init_url: Url, ord: &mut OrdersContainer<Ms, Mdl, INodes, GMs>)
        -> Init<Mdl>;
}

impl<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs, F> IntoInit<Ms, Mdl, INodes, GMs> for F
where
    F: FnOnce(Url, &mut OrdersContainer<Ms, Mdl, INodes, GMs>) -> Init<Mdl>,
{
    fn into_init(
        self,
        init_url: Url,
        ord: &mut OrdersContainer<Ms, Mdl, INodes, GMs>,
    ) -> Init<Mdl> {
        self(init_url, ord)
    }
}

impl<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs> IntoAfterMount<Ms, Mdl, INodes, GMs>
    for (Init<Mdl>, OrdersContainer<Ms, Mdl, INodes, GMs>)
{
    fn into_after_mount(
        self: Box<Self>,
        _: Url,
        ord: &mut OrdersContainer<Ms, Mdl, INodes, GMs>,
    ) -> AfterMount<Mdl> {
        let (init, old_ord) = *self;
        ord.merge(old_ord);
        AfterMount {
            model: init.model,
            url_handling: init.url_handling,
        }
    }
}
