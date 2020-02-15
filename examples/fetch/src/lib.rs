//! Example of the Fetch API.
//!
//! See simple.rs for the most basic usage

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};

mod simple;

#[derive(Default)]
struct Model {
    simple: simple::Model,
}

enum Msg {
    Simple(simple::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Simple(msg) => simple::update(msg, &mut model.simple, &mut orders.proxy(Msg::Simple)),
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![div![simple::view(&model.simple).map_msg(Msg::Simple)],]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
