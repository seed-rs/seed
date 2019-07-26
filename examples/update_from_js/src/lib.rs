#[macro_use]
extern crate seed;
use seed::prelude::*;
use serde::Deserialize;

// Model

struct Model {
    time_from_js: Option<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self { time_from_js: None }
    }
}

// Update

// We trigger update only from JS land
#[allow(dead_code)]
// `Deserialize` is required for receiving messages from JS land
// (see `trigger_update_handler`)
#[derive(Clone, Deserialize)]
enum Msg {
    ClockEnabled,
    Tick(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClockEnabled => log!("Clock enabled"),
        Msg::Tick(time) => model.time_from_js = Some(time),
    }
}

// View

fn view(model: &Model) -> Node<Msg> {
    h1![
        style![
            "text-align" => "center"
            "margin-top" => unit!(40, vmin)
            "font-size" => unit!(10, vmin)
            "font-family" => "monospace"
        ],
        model.time_from_js.clone().unwrap_or_default()
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_,_| Model::default(), update, view)
        // `trigger_update_handler` processes JS event
        // and forwards it to `update` function.
        .window_events(|_| vec![trigger_update_handler()])
        .finish()
        .run();

    // call JS function `enableClock` from `clock.js`
    enableClock();
}

#[wasm_bindgen]
extern "C" {
    fn enableClock();
}
