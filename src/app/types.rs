use super::OrdersContainer;
use crate::browser::Url;
use crate::virtual_dom::EventHandler;

pub type InitFn<Ms, Mdl, INodes, GMs> = fn(Url, &mut OrdersContainer<Ms, Mdl, INodes, GMs>) -> Mdl;
pub type UpdateFn<Ms, Mdl, INodes, GMs> =
    fn(Ms, &mut Mdl, &mut OrdersContainer<Ms, Mdl, INodes, GMs>);
pub type SinkFn<Ms, Mdl, INodes, GMs> =
    fn(GMs, &mut Mdl, &mut OrdersContainer<Ms, Mdl, INodes, GMs>);
pub type ViewFn<Mdl, INodes> = fn(&Mdl) -> INodes;
pub type RoutesFn<Ms> = fn(Url) -> Option<Ms>;
pub type WindowEventsFn<Ms, Mdl> = fn(&Mdl) -> Vec<EventHandler<Ms>>;
pub type MsgListeners<Ms> = Vec<Box<dyn Fn(&Ms)>>;
