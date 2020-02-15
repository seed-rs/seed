//! Example of the Fetch API.
//!
//! See simple.rs for the most basic usage

//// Some Clippy linter rules are ignored for the sake of simplicity.
//#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};

mod post;
mod simple;

#[derive(Default)]
struct Model {
    simple: simple::Model,
    post: post::Model,
}

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

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        div![simple::view(&model.simple).map_msg(Msg::Simple)],
        hr![],
        div![post::view(&model.post).map_msg(Msg::Post)],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
