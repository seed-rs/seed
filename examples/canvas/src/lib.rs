//! [Web-sys docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.CanvasRenderingContext2d.html)
//! [Web-sys example](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
//! [MDN](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)

use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    Model::default()
}

// ------ ------
//     Model
// ------ ------

struct Model {
    fill_color: Color,
    zoom: f64,
    canvas: ElRef<HtmlCanvasElement>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            fill_color: Color::default(),
            zoom: 1.,
            canvas: ElRef::<HtmlCanvasElement>::default(),
        }
    }
}

// ------ Color -------

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    A,
    B,
}

impl Color {
    const fn as_str(&self) -> &str {
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
//    Update
// ------ ------

enum Zoom {
    In,
    Out,
}

enum Msg {
    Rendered,
    ChangeColor,
    Zoom(Zoom),
}

#[allow(clippy::needless_pass_by_value)]
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            draw(&model.canvas, model.fill_color, model.zoom);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
        Msg::ChangeColor => {
            model.fill_color = if model.fill_color == Color::A {
                Color::B
            } else {
                Color::A
            };
        }
        Msg::Zoom(zoom) => {
            model.zoom += match zoom {
                Zoom::In => -0.1,
                Zoom::Out => 0.1,
            };
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, fill_color: Color, zoom: f64) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    // clear canvas
    ctx.begin_path();
    ctx.clear_rect(0., 0., 400., 200.);

    let width = 200. * zoom;
    let height = 100. * zoom;

    ctx.rect(0., 0., width, height);
    ctx.set_fill_style(&JsValue::from_str(fill_color.as_str()));
    ctx.fill();

    ctx.move_to(0., 0.);
    ctx.line_to(width, height);
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
                At::Width => px(400),
                At::Height => px(200),
            ],
            style![
                St::Border => "1px solid black",
            ],
            wheel_ev(Ev::Wheel, |event| {
                let delta_y = event.delta_y();
                (delta_y != 0.0).then(|| {
                    event.prevent_default();
                    Msg::Zoom(if delta_y < 0.0 { Zoom::In } else { Zoom::Out })
                })
            }),
        ],
        button!["Change color", ev(Ev::Click, |_| Msg::ChangeColor)],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
