use super::OrdersContainer;

// @TODO: Move content to more appropriate files and delete this file.

pub type UpdateFn<Ms, Mdl, INodes> = fn(Ms, &mut Mdl, &mut OrdersContainer<Ms, Mdl, INodes>);
pub type ViewFn<Mdl, INodes> = fn(&Mdl) -> INodes;
pub type MsgListeners<Ms> = Vec<Box<dyn Fn(&Ms)>>;
