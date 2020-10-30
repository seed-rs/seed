use seed::{prelude::*, *};
pub mod message;
pub mod statistics;
pub mod tasks;

pub use router::{Init, View};
use seed_routing::*;

#[derive(Debug, PartialEq, Clone, RoutingModules)]
pub enum Routes {
    Message,
    Tasks {
        query: IndexMap<String, String,>,
        children: tasks::Routes,
    },
    Statistics,
    #[default_route]
    #[view = "=> root"]
    #[as_path = ""]
    Root,
}
pub fn init(_: Url, model: &mut Model, nested: &Routes, orders: &mut impl Orders<Msg,>,) -> Model {
    nested.init(model, orders,);
    model.clone()
}

#[derive(Default, Clone)]
pub struct Model {
    pub name: String,
    pub message: message::Model,
    pub statistics: statistics::Model,
    pub tasks: tasks::Model,
}

pub enum Msg {
    ChangeName,
    Message(message::Msg,),
    Statistics(statistics::Msg,),
    Tasks(tasks::Msg,),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg,>,) {
    match msg {
        Msg::ChangeName => {},
        Msg::Message(message,) => message::update(
            message,
            &mut model.message,
            &mut orders.proxy(Msg::Message,),
        ),
        Msg::Statistics(statistics,) => statistics::update(
            statistics,
            &mut model.statistics,
            &mut orders.proxy(Msg::Statistics,),
        ),
        Msg::Tasks(task,) => tasks::update(task, &mut model.tasks, &mut orders.proxy(Msg::Tasks,),),
    }
}
pub fn view(dashboard_routes: &Routes, model: &Model,) -> Node<Msg,> {
    dashboard_routes.view(model,)
}

pub fn root(_: &Model,) -> Node<Msg,> {
    div!["root for dashboard"]
}
