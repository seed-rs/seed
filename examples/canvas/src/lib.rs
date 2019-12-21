//! [Web-sys docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html)
//! [Web-sys example](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
//! [MDN](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)

use seed::{prelude::*, *};

type Color = &'static str;

const COLOR_A: Color = "white";
const COLOR_B: Color = "green";

const CANVAS_ID: &str = "canvas";

// Model

struct Model {
    fill_color: Color,
}

// AfterMount

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.after_next_render(|_| Msg::Rendered);
    AfterMount::new(Model {
        fill_color: COLOR_A,
    })
}

// Update

#[derive(Copy, Clone)]
enum Msg {
    Rendered,
    ChangeColor,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            draw(model.fill_color);
            // We want to call `.skip` to prevent infinite loop.
            // (Infinite loops are useful for animations.)
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
        Msg::ChangeColor => {
            model.fill_color = if model.fill_color == COLOR_A {
                COLOR_B
            } else {
                COLOR_A
            };
        }
    }
}

fn draw(fill_color: Color) {
    let canvas = seed::canvas(CANVAS_ID).unwrap();
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.rect(0., 0., 200., 100.);
    ctx.set_fill_style(&JsValue::from_str(fill_color));
    ctx.fill();

    ctx.move_to(0., 0.);
    ctx.line_to(200., 100.);
    ctx.stroke();
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
        button!["Change color", simple_ev(Ev::Click, Msg::ChangeColor)],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
