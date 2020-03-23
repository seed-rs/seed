use super::super::OrdersContainer;
use crate::browser::Url;
use crate::virtual_dom::IntoNodes;

#[allow(clippy::module_name_repetitions)]
pub struct UndefinedAfterMount;

// ------ UrlHandling ------

/// Used for handling initial routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UrlHandling {
    PassToRoutes,
    None,
}

impl Default for UrlHandling {
    fn default() -> Self {
        Self::PassToRoutes
    }
}

// ------ AfterMount ------

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AfterMount<Mdl> {
    /// Initial model to be used when the app begins.
    pub(crate) model: Mdl,
    /// How to handle initial url routing. Defaults to [`UrlHandling::PassToRoutes`] in the
    /// constructors.
    pub(crate) url_handling: UrlHandling,
}

impl<Mdl> AfterMount<Mdl> {
    /// Creates a new `AfterMount` instance. You can also use `AfterMount::default`
    /// if your `Model` implements `Default`.
    pub fn new(model: Mdl) -> Self {
        Self {
            model,
            url_handling: UrlHandling::default(),
        }
    }

    /// - `UrlHandling::PassToRoutes` - your function `routes` will be called with initial URL. _[Default]_
    /// - `UrlHandling::None` - URL won't be handled by Seed.
    pub const fn url_handling(mut self, url_handling: UrlHandling) -> Self {
        self.url_handling = url_handling;
        self
    }
}

// ------ IntoAfterMount ------

#[allow(clippy::module_name_repetitions)]
pub trait IntoAfterMount<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs> {
    fn into_after_mount(
        self: Box<Self>,
        init_url: Url,
        orders: &mut OrdersContainer<Ms, Mdl, INodes, GMs>,
    ) -> AfterMount<Mdl>;
}

impl<Ms: 'static, Mdl, INodes: IntoNodes<Ms>, GMs, F> IntoAfterMount<Ms, Mdl, INodes, GMs> for F
where
    F: FnOnce(Url, &mut OrdersContainer<Ms, Mdl, INodes, GMs>) -> AfterMount<Mdl>,
{
    fn into_after_mount(
        self: Box<Self>,
        init_url: Url,
        orders: &mut OrdersContainer<Ms, Mdl, INodes, GMs>,
    ) -> AfterMount<Mdl> {
        self(init_url, orders)
    }
}

impl<Ms: 'static, Mdl: Default, INodes: IntoNodes<Ms>, GMs> IntoAfterMount<Ms, Mdl, INodes, GMs>
    for UndefinedAfterMount
{
    fn into_after_mount(
        self: Box<Self>,
        _: Url,
        _: &mut OrdersContainer<Ms, Mdl, INodes, GMs>,
    ) -> AfterMount<Mdl> {
        AfterMount::default()
    }
}
