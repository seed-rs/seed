//! [Web-sys docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html)
//! [Web-sys example](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
//! [MDN](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)

#[macro_use]
extern crate seed;
use seed::prelude::*;

const CANVAS_ID: &str = "canvas";

// Model

struct Model {}

// Update

#[derive(Copy, Clone)]
enum Msg {
    Draw,
}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Draw => fill(),
    }
}

// View

fn view(_model: &Model) -> impl View<Msg> {
    div![
        style! {St::Display => "flex"},
        canvas![
            attrs![
                At::Id => CANVAS_ID,
                At::Width => px(200),
                At::Height => px(100),
            ],
            style![
                St::Border => "1px solid black",
            ],
        ],
        button!["Change color", simple_ev(Ev::Click, Msg::Draw)],
    ]
}

fn draw() {
    let canvas = seed::canvas(CANVAS_ID).unwrap();
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.move_to(0., 0.);
    ctx.line_to(200., 100.);
    ctx.stroke();
}

fn fill() {
    let canvas = seed::canvas(CANVAS_ID).unwrap();
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.rect(0., 0., 200., 100.);
    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill();
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Init::new(Model {}), update, view).build_and_start();
    draw();
}
