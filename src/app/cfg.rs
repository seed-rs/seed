use super::types::{UpdateFn, ViewFn};
use crate::virtual_dom::IntoNodes;
use std::rc::Rc;

#[allow(clippy::module_name_repetitions)]
pub struct AppCfg<Ms, Mdl, INodes>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    pub(crate) document: web_sys::Document,
    // @TODO: Look into removing mount_point
    pub(crate) mount_point: web_sys::Element,
    pub(crate) update: UpdateFn<Ms, Mdl, INodes>,
    pub(crate) view: ViewFn<Mdl, INodes>,
    pub(crate) base_path: Rc<Vec<String>>,
}
