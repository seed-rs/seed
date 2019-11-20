use web_sys::Element;

use crate::{
    dom_types::View,
    orders::OrdersContainer,
    routing::Url, util,
    vdom::{
        alias::*,
        App,
        builder::{
            before_mount::{MountPoint, MountType},
            after_mount::{UrlHandling},
        },
    },
};


/// Used as a flexible wrapper for the init function.
pub struct Init<Mdl> {
    /// Initial model to be used when the app begins.
    pub model: Mdl,
    /// How to handle initial url routing. Defaults to [`UrlHandling::PassToRoutes`] in the
    /// constructors.
    pub url_handling: UrlHandling,
    /// How to handle elements already present in the mount. Defaults to [`MountType::Append`]
    /// in the constructors.
    pub mount_type: MountType,
}

impl<Mdl> Init<Mdl> {
    pub const fn new(model: Mdl) -> Self {
        Self {
            model,
            url_handling: UrlHandling::PassToRoutes,
            mount_type: MountType::Append,
        }
    }

    pub const fn new_with_url_handling(model: Mdl, url_handling: UrlHandling) -> Self {
        Self {
            model,
            url_handling,
            mount_type: MountType::Append,
        }
    }
}

impl<Mdl: Default> Default for Init<Mdl> {
    fn default() -> Self {
        Self {
            model: Mdl::default(),
            url_handling: UrlHandling::PassToRoutes,
            mount_type: MountType::Append,
        }
    }
}

pub type InitFn<Ms, Mdl, ElC, GMs> =
    Box<dyn FnOnce(Url, &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl>>;

pub trait IntoInit<Ms: 'static, Mdl, ElC: View<Ms>, GMs> {
    fn into_init(self, init_url: Url, ord: &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl>;
}

impl<Ms: 'static, Mdl, ElC: View<Ms>, GMs, F> IntoInit<Ms, Mdl, ElC, GMs>
    for F
    where
        F: FnOnce(Url, &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl>,
{
    fn into_init(self, init_url: Url, ord: &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl> {
        self(init_url, ord)
    }
}
