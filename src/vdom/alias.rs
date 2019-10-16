use crate::{events, orders::OrdersContainer, routing};

pub type UpdateFn<Ms, Mdl, ElC, GMs> = fn(Ms, &mut Mdl, &mut OrdersContainer<Ms, Mdl, ElC, GMs>);
pub type SinkFn<Ms, Mdl, ElC, GMs> = fn(GMs, &mut Mdl, &mut OrdersContainer<Ms, Mdl, ElC, GMs>);
pub type ViewFn<Mdl, ElC> = fn(&Mdl) -> ElC;
pub type RoutesFn<Ms> = fn(routing::Url) -> Option<Ms>;
pub type WindowEvents<Ms, Mdl> = fn(&Mdl) -> Vec<events::Listener<Ms>>;
pub type MsgListeners<Ms> = Vec<Box<dyn Fn(&Ms)>>;
