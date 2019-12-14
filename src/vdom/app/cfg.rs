use crate::dom_types::View;
use crate::vdom::{
    alias::*,
    builder::{after_mount::IntoAfterMount, before_mount::MountType},
};
use std::marker::PhantomData;

#[allow(clippy::module_name_repetitions)]
pub struct AppInitCfg<Ms, Mdl, ElC, GMs, IAM: ?Sized>
where
    Ms: 'static,
    Mdl: 'static,
    ElC: View<Ms>,
    IAM: IntoAfterMount<Ms, Mdl, ElC, GMs>,
{
    pub mount_type: MountType,
    pub into_after_mount: Box<IAM>,
    pub phantom: PhantomData<(Ms, Mdl, ElC, GMs)>,
}

#[allow(clippy::module_name_repetitions)]
pub struct AppCfg<Ms, Mdl, ElC, GMs>
where
    Ms: 'static,
    Mdl: 'static,
    ElC: View<Ms>,
{
    pub document: web_sys::Document,
    pub mount_point: web_sys::Element,
    pub update: UpdateFn<Ms, Mdl, ElC, GMs>,
    pub sink: Option<SinkFn<Ms, Mdl, ElC, GMs>>,
    pub view: ViewFn<Mdl, ElC>,
    pub window_events: Option<WindowEventsFn<Ms, Mdl>>,
}
