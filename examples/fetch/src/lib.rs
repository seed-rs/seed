//! A simple, cliché example demonstrating structure and syntax.
//! Inspired by [Elm example](https://guide.elm-lang.org/architecture/buttons.html).

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};
use seed::browser::fetch::*;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    foo: Option<Foo>,
}

// ------ ------
//    Update
// ------ ------

#[derive(serde::Deserialize)]
struct Foo {
    bar: usize,
}

enum Msg {
    SendRequest,
    Fetched(Foo),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            orders.skip().perform_cmd(fetch_foo());
        },
        Msg::Fetched(foo) => {
            model.foo = Some(foo);
        },
    }
}

async fn fetch_foo() -> Msg {
    let response = fetch("/foo.json").await.expect("Request failed");
    let foo: Foo = response.json().await.expect("Deserialization failed");
    Msg::Fetched(foo)
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::SendRequest), "Fetch"],
        if let Some(foo) = &model.foo {
            div![format!("Bar: {}", foo.bar)]
        } else {
            empty![]
        }
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
