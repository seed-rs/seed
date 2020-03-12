use super::OrdersContainer;
use crate::browser::Url;
use crate::virtual_dom::EventHandler;

pub type InitFn<Ms, Mdl, ElC, GMs> = fn(Url, &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Mdl;
pub type UpdateFn<Ms, Mdl, ElC, GMs> = fn(Ms, &mut Mdl, &mut OrdersContainer<Ms, Mdl, ElC, GMs>);
pub type SinkFn<Ms, Mdl, ElC, GMs> = fn(GMs, &mut Mdl, &mut OrdersContainer<Ms, Mdl, ElC, GMs>);
pub type ViewFn<Mdl, ElC> = fn(&Mdl) -> ElC;
pub type RoutesFn<Ms> = fn(Url) -> Option<Ms>;
pub type WindowEventsFn<Ms, Mdl> = fn(&Mdl) -> Vec<EventHandler<Ms>>;
pub type MsgListeners<Ms> = Vec<Box<dyn Fn(&Ms)>>;
