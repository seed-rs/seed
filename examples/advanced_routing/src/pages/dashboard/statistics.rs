use seed::{prelude::*, *};

pub fn init(_: Url, _: &mut Model, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[derive(Default)]
pub struct Model {
    pub routes_history_count: u32,
}

pub enum Msg {
    AddMessage(String),
}
pub fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}
pub fn view(model: &Model) -> Node<Msg> {
    div!["route visited => {}", &model.routes_history_count]
}
