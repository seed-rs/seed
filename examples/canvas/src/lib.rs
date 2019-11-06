//! [Web-sys docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html)
//! [Web-sys example](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
//! [MDN](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)

#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

struct Model {}

// Update

#[derive(Clone)]
enum Msg {}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

// View

fn view(_model: &Model) -> impl View<Msg> {
    vec![
        h1!["Example canvas"],
        canvas![
            attrs![
                At::Id => "canvas",
                At::Width => px(200),
                At::Height => px(100),
            ],
            style![
                St::Border => "1px solid #000000",
            ],
        ],
    ]
}

fn draw() {
    let canvas = seed::canvas("canvas");
    let ctx = seed::canvas_context(&canvas, "2d");

    ctx.move_to(0., 0.);
    ctx.line_to(200., 100.);
    ctx.stroke();
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Init::new(Model {}), update, view).build_and_start();
    draw();
}
