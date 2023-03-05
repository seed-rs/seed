#![allow(clippy::needless_pass_by_value)]

use gloo_console::log;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let svg_container = ElRef::new();
    let msg_sender = orders.msg_sender();

    orders.after_next_render({
        let svg_container = svg_container.clone();
        move |_| {
            let element = svg_container.get().expect("get svg_container element");
            let callback = move |width, height| msg_sender(Some(Msg::Resized(width, height)));

            let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn(f64, f64)>);
            let closure_as_js_value = closure.as_ref().clone();
            closure.forget();

            observeElementSize(&element, &closure_as_js_value);
        }
    });

    Model {
        svg_container,
        svg_container_size: None,
    }
}

// ------ ------
//    Extern
// ------ ------

#[wasm_bindgen]
extern "C" {
    fn observeElementSize(element: &web_sys::Element, callback: &JsValue);
}

// ------ ------
//     Model
// ------ ------

struct Model {
    svg_container: ElRef<web_sys::Element>,
    svg_container_size: Option<(f64, f64)>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Resized(f64, f64),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Resized(width, height) => model.svg_container_size = Some((width, height)),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        style! {
            St::Width => vw(100),
            St::Height => vh(100),
            St::Display => "flex",
            St::JustifyContent => "center",
            St::AlignItems => "center",
        },
        svg![
            el_ref(&model.svg_container),
            style! {
                St::Border => "solid orange 4px",
            },
            attrs! {
                At::Width => percent(60),
                At::Height => percent(60),
            },
            ev(Ev::Resize, |_| log!("resize!")),
            rect![attrs! {
                At::Width => percent(100),
                At::Height => percent(100),
                At::Fill => "whitesmoke",
            },],
            model.svg_container_size.map(|(width, height)| {
                text![
                    style! {
                        St::FontSize => px(30),
                    },
                    attrs! {
                        At::X => 20,
                        At::Y => 40,
                    },
                    format!("{:.1} x {:.1}", width, height),
                ]
            })
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
