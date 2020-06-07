use super::OrdersContainer;
use crate::virtual_dom::IntoNodes;
use std::rc::Rc;

#[allow(clippy::module_name_repetitions, clippy::type_complexity)]
pub struct AppCfg<Ms, Mdl, INodes>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    pub(crate) document: web_sys::Document,
    pub(crate) mount_point: web_sys::Element,
    pub(crate) update: Box<dyn Fn(Ms, &mut Mdl, &mut OrdersContainer<Ms, Mdl, INodes>)>,
    pub(crate) view: Box<dyn Fn(&Mdl) -> INodes>,
    pub(crate) base_path: Rc<Vec<String>>,
}
