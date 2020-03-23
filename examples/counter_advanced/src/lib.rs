#![allow(clippy::non_ascii_literal, clippy::replace_consts)]

use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

struct Model {
    count: i32,
    what_we_count: String,
}

// Setup a default here - `Model::default` will be automatically called in the default `AfterMount`.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".to_owned(),
        }
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Increment,
    Decrement,
    WhatWeCountChanged(String),
}

/// The sole source of updating the model.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
        Msg::WhatWeCountChanged(what_we_count) => model.what_we_count = what_we_count,
    }
}

// ------ ------
//     View
// ------ ------

/// The top-level view we pass to the virtual dom.
///  - Must accept the model as its only argument.
///  - Output has to implement trait `IntoNodes` (e.g. `Node<Msg>` or `Vec<Node<Msg>`).
fn view(model: &Model) -> impl IntoNodes<Msg> {
    let plural = if model.count.abs() == 1 { "" } else { "s" };
    let text = format!("{} {}{} so far", model.count, model.what_we_count, plural);

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style! {
            St::Display => "flex",
            St::FlexDirection => "column",
            St::TextAlign => "center",
    };

    div![
        // We can use normal Rust code and comments in the view.
        outer_style,
        h1!["The Grand Total"],
        div![
            style! {
                // Example of conditional logic in a style.
                St::Color => if model.count > 4 {"purple"} else {"gray"};
                St::Border => "2px solid #004422";
                St::Padding => unit!(20, px);
            },
            h3![text],
            button![ev(Ev::Click, |_| Msg::Increment), "+"],
            button![ev(Ev::Click, |_| Msg::Decrement), "-"],
            // Optionally-displaying an element.
            if model.count >= 10 {
                h2![style! {St::Padding => px(50)}, "Nice!",]
            } else {
                empty![]
            },
        ],
        view_success_level(model.count), // Incorporating a separate reusable view.
        h3!["What are we counting?"],
        input![
            attrs! {At::Value => model.what_we_count},
            input_ev(Ev::Input, Msg::WhatWeCountChanged),
        ],
    ]
}

/// A simple reusable view.
fn view_success_level(clicks: i32) -> Node<Msg> {
    let description = match clicks {
        std::i32::MIN..=5 => "Not very many 🙁",
        6..=9 => "I got my first real six-string 😐",
        10..=11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃",
    };
    p![description]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
