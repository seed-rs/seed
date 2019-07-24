//! A simple, cliché example demonstrating structure and syntax.

#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;

use seed::prelude::*;

// Model

struct Model {
    count: i32,
    what_we_count: String,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into(),
        }
    }
}

// Update

#[derive(Debug, Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeWWC(String),
}

/// The sole source of updating the model
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
        Msg::ChangeWWC(what_we_count) => model.what_we_count = what_we_count,
    }
}

// View

/// A simple component.
fn success_level(clicks: i32) -> Node<Msg> {
    let descrip = match clicks {
        0...5 => "Not very many 🙁",
        6...9 => "I got my first real six-string 😐",
        10...11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃",
    };
    p![descrip]
}

/// The top-level component we pass to the virtual dom. Must accept the model as its
/// only argument, and output has to implement trait `ElContainer`.
fn view(model: &Model) -> impl View<Msg> {
    let plural = if model.count == 1 { "" } else { "s" };
    let text = format!("{} {}{} so far", model.count, model.what_we_count, plural);

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style! {
            "display" => "flex",
            "flex-direction" => "column",
            "text-align" => "center",
    };

    div![
        outer_style,
        h1!["The Grand Total"],
        div![
            style! {
                // Example of conditional logic in a style.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                "border" => "2px solid #004422";
                "padding" => unit!(20, px);
            },
            // We can use normal Rust code and comments in the view.
            h3![text, did_update(|_| log!("This shows when we increment"))],
            button![simple_ev(Ev::Click, Msg::Increment), "+"],
            button![simple_ev(Ev::Click, Msg::Decrement), "-"],
            // Optionally-displaying an element, and lifecycle hooks
            if model.count >= 10 {
                h2![
                    style! {"padding" => px(50)},
                    "Nice!",
                    did_mount(|_| log!("This shows when clicks reach 10")),
                    will_unmount(|_| log!("This shows when clicks drop below 10")),
                ]
            } else {
                empty![]
            },
        ],
        success_level(model.count), // Incorporating a separate component
        h3!["What are we counting?"],
        input![
            attrs! {At::Value => model.what_we_count},
            input_ev(Ev::Input, Msg::ChangeWWC),
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();
}
