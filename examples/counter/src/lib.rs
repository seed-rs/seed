//! A simple, cliché example demonstrating structure and syntax.
//! Inspired by [Elm example](https://guide.elm-lang.org/architecture/buttons.html).

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

type Model = i32;

// ------ ------
//    Update
// ------ ------

enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
        Msg::Decrement => *model -= 1,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::Decrement), "-"],
        div![model],
        button![ev(Ev::Click, |_| Msg::Increment), "+"],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
