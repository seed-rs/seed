#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;

use futures::prelude::*;
// todo: crate:: here is temporary, until gloo_timers is published.
use seed::gloo_timers::future::TimeoutFuture;
use seed::prelude::*;

// Model

#[derive(Default)]
struct Model {
    title: String,
    greet_clicked: bool,
}

// Update

#[derive(Clone)]
enum Msg {
    Greet,
    WriteHello,
    WriteName(String),
    WriteExclamationMarks,
    WriteEmoticon(String),
    TimeoutError,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Greet => {
            model.greet_clicked = true;
            orders
                .skip()
                .send_msg(Msg::WriteHello)
                .send_msg(Msg::WriteName("World".into()))
                .perform_cmd(write_exclamation_marks_after_delay())
                .perform_cmd(write_emoticon_after_delay("🙂".into()));
        }
        Msg::WriteHello => model.title.push_str("Hello "),
        Msg::WriteName(name) => model.title.push_str(&name),
        Msg::WriteExclamationMarks => model.title.push_str("!!! "),
        Msg::WriteEmoticon(emoticon) => model.title.push_str(&emoticon),
        Msg::TimeoutError => {
            error!("Timeout failed!");
            orders.skip();
        }
    }
}

fn write_exclamation_marks_after_delay() -> impl Future<Item = Msg, Error = Msg> {
    TimeoutFuture::new(1_000)
        .map(|_| Msg::WriteExclamationMarks)
        .map_err(|_| Msg::TimeoutError)
}

fn write_emoticon_after_delay(emoticon: String) -> impl Future<Item = Msg, Error = Msg> {
    TimeoutFuture::new(2_000)
        .map(|_| Msg::WriteEmoticon(emoticon))
        .map_err(|_| Msg::TimeoutError)
}

// View

fn view(model: &Model) -> impl View<Msg> {
    div![
        style![
            "display" => "flex",
            "justify-content" => "center",
            "align-items" => "center",
            "font-size" => vmin(5),
            "font-family" => "sans-serif",
            "height" => vmin(50),
        ],
        if model.greet_clicked {
            h1![model.title]
        } else {
            div![
                style![
                    "background-color" => "lightgreen",
                    "padding" => vmin(3),
                    "border-radius" => vmin(3),
                    "cursor" => "pointer",
                    "box-shadow" => [vmin(0), vmin(0.5), vmin(0.5), "green".into()].join(" "),
                ],
                simple_ev(Ev::Click, Msg::Greet),
                "Greet!"
            ]
        }
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();
}
