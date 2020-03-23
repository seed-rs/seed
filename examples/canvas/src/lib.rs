//! [Web-sys docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html)
//! [Web-sys example](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
//! [MDN](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)

use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    fill_color: Color,
    canvas: ElRef<HtmlCanvasElement>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    A,
    B,
}

impl Color {
    fn as_str(&self) -> &str {
        match self {
            Self::A => "white",
            Self::B => "green",
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::A
    }
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.after_next_render(|_| Msg::Rendered);
    AfterMount::default()
}

// ------ ------
//    Update
// ------ ------

#[derive(Copy, Clone)]
enum Msg {
    Rendered,
    ChangeColor,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            draw(&model.canvas, model.fill_color);
            // We want to call `.skip` to prevent infinite loop.
            // (Infinite loops are useful for animations.)
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
        Msg::ChangeColor => {
            model.fill_color = if model.fill_color == Color::A {
                Color::B
            } else {
                Color::A
            };
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, fill_color: Color) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.rect(0., 0., 200., 100.);
    ctx.set_fill_style(&JsValue::from_str(fill_color.as_str()));
    ctx.fill();

    ctx.move_to(0., 0.);
    ctx.line_to(200., 100.);
    ctx.stroke();
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        style! {St::Display => "flex"},
        canvas![
            el_ref(&model.canvas),
            attrs![
                At::Width => px(200),
                At::Height => px(100),
            ],
            style![
                St::Border => "1px solid black",
            ],
        ],
        button!["Change color", ev(Ev::Click, |_| Msg::ChangeColor)],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
