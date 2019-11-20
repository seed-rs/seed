use crate::{dom_types::View, orders::OrdersContainer, routing::Url};

/// Used for handling initial routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UrlHandling {
    PassToRoutes,
    None,
    // todo: Expand later, as-required
}

impl Default for UrlHandling {
    fn default() -> Self {
        Self::PassToRoutes
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AfterMount<Mdl> {
    pub model: Mdl,
    pub url_handling: UrlHandling,
}

impl<Mdl> AfterMount<Mdl> {
    pub fn new(model: Mdl) -> Self {
        Self {
            model,
            url_handling: UrlHandling::default(),
        }
    }

    // TODO: Change to const fn when possible.
    // TODO: Relevant issue: https://github.com/rust-lang/rust/issues/60964
    #[allow(clippy::missing_const_for_fn)]
    pub fn model<NewMdl>(self, model: NewMdl) -> AfterMount<NewMdl> {
        AfterMount {
            model,
            url_handling: self.url_handling,
        }
    }

    pub const fn url_handling(mut self, url_handling: UrlHandling) -> Self {
        self.url_handling = url_handling;
        self
    }
}

pub trait Into<Ms: 'static, Mdl, ElC: View<Ms>, GMs> {
    fn into_after_mount(
        self: Box<Self>,
        init_url: Url,
        orders: &mut OrdersContainer<Ms, Mdl, ElC, GMs>,
    ) -> AfterMount<Mdl>;
}

impl<Ms: 'static, Mdl, ElC: View<Ms>, GMs, F> Into<Ms, Mdl, ElC, GMs> for F
where
    F: FnOnce(Url, &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> AfterMount<Mdl>,
{
    fn into_after_mount(
        self: Box<Self>,
        init_url: Url,
        orders: &mut OrdersContainer<Ms, Mdl, ElC, GMs>,
    ) -> AfterMount<Mdl> {
        self(init_url, orders)
    }
}

impl<Ms: 'static, Mdl: Default, ElC: View<Ms>, GMs> Into<Ms, Mdl, ElC, GMs> for () {
    fn into_after_mount(
        self: Box<Self>,
        _: Url,
        _: &mut OrdersContainer<Ms, Mdl, ElC, GMs>,
    ) -> AfterMount<Mdl> {
        AfterMount::default()
    }
}
