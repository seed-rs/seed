use super::{builder::IntoAfterMount, types::*, MountType};
use crate::virtual_dom::IntoNodes;
use std::marker::PhantomData;

#[allow(clippy::module_name_repetitions)]
pub struct AppInitCfg<Ms, Mdl, INodes, GMs, IAM: ?Sized>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
    IAM: IntoAfterMount<Ms, Mdl, INodes, GMs>,
{
    pub mount_type: MountType,
    pub into_after_mount: Box<IAM>,
    pub phantom: PhantomData<(Ms, Mdl, INodes, GMs)>,
}

#[allow(clippy::module_name_repetitions)]
pub struct AppCfg<Ms, Mdl, INodes, GMs>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    pub document: web_sys::Document,
    pub mount_point: web_sys::Element,
    pub update: UpdateFn<Ms, Mdl, INodes, GMs>,
    pub sink: Option<SinkFn<Ms, Mdl, INodes, GMs>>,
    pub view: ViewFn<Mdl, INodes>,
    pub window_events: Option<WindowEventsFn<Ms, Mdl>>,
}
