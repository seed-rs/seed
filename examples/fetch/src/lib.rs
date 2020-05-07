//! Example of the Fetch API.
//!
//! See `simple.rs` for the most basic usage.

use seed::{prelude::*, *};

mod post;
mod simple;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    simple: simple::Model,
    post: post::Model,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Simple(simple::Msg),
    Post(post::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Simple(msg) => simple::update(msg, &mut model.simple, &mut orders.proxy(Msg::Simple)),
        Msg::Post(msg) => post::update(msg, &mut model.post, &mut orders.proxy(Msg::Post)),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        div![simple::view(&model.simple).map_msg(Msg::Simple)],
        hr![],
        div![post::view(&model.post).map_msg(Msg::Post)],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
