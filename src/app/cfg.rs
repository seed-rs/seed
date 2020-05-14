use super::types::{SinkFn, UpdateFn, ViewFn, WindowEventsFn};
use crate::virtual_dom::IntoNodes;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(clippy::module_name_repetitions)]
pub struct AppInitCfg<Ms, Mdl, INodes, GMs>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    pub takeover: bool,
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
    pub base_path: Rc<Vec<String>>,
}
