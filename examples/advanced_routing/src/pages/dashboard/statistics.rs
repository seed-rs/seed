use seed::{prelude::*, *};

pub fn init(_: Url, _: &mut Model, orders: &mut impl Orders<Msg,>,) -> Model {
    orders.subscribe(Msg::UrlChanged,);

    Model::default()
}

#[derive(Default, Clone)]
pub struct Model {
    pub routes_history_count: u32,
}

pub enum Msg {
    UrlChanged(subs::UrlChanged,),
}
pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg,>,) {
    match msg {
        Msg::UrlChanged(_,) => {
            model.routes_history_count += 1;
        },
    }
}
pub fn view(model: &Model,) -> Node<Msg,> {
    div!["route visited => {} ", &model.routes_history_count]
}
