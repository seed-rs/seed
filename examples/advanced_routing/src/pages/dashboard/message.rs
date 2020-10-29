use seed::{prelude::*, *};

pub fn init(_: Url, _: &mut Model, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[derive(Default, Clone)]
pub struct Model {
    pub messages: Vec<String>,
}

pub enum Msg {
    AddMessage(String),
}
pub fn update(msg: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddMessage(_) => {}
    }
}
pub fn view(_: &Model) -> Node<Msg> {
    div!["messages list"]
}
