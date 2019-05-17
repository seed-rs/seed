#[macro_use]
extern crate seed;

use futures::prelude::*;
use gloo_timers::future::TimeoutFuture;
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

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
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

fn view(model: &Model) -> impl ElContainer<Msg> {
    div![
        style![
            "display" => "flex";
            "justify-content" => "center";
            "align-items" => "center";
            "font-size" => "5vmin";
            "font-family" => "sans-serif";
            "height" => "50vmin";
        ],
        if model.greet_clicked {
            h1![model.title]
        } else {
            div![
                style![
                    "background-color" => "lightgreen";
                    "padding" => "3vmin";
                    "border-radius" => "3vmin";
                    "cursor" => "pointer";
                    "box-shadow" => "0 0.5vmin 0.5vmin green";
                ],
                simple_ev(Ev::Click, Msg::Greet),
                "Greet!"
            ]
        }
    ]
}

#[wasm_bindgen]
pub fn start() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}
