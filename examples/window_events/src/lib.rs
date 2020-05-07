use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    event_streams: Vec<StreamHandle>,
    point: Point,
    key_code: u32,
}

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ToggleWatching,
    MouseMoved(web_sys::MouseEvent),
    KeyPressed(web_sys::KeyboardEvent),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleWatching => {
            if model.event_streams.is_empty() {
                model.event_streams = vec![
                    orders.stream_with_handle(streams::window_event(Ev::MouseMove, |event| {
                        Msg::MouseMoved(event.unchecked_into())
                    })),
                    orders.stream_with_handle(streams::window_event(Ev::KeyDown, |event| {
                        Msg::KeyPressed(event.unchecked_into())
                    })),
                ];
            } else {
                model.event_streams.clear();
            }
        },
        Msg::MouseMoved(ev) => {
            model.point = Point {
                x: ev.client_x(),
                y: ev.client_y(),
            }
        }
        Msg::KeyPressed(ev) => model.key_code = ev.key_code(),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        h2![format!("X: {}, Y: {}", model.point.x, model.point.y)],
        h2![format!("Last key pressed: {}", model.key_code)],
        button![
            ev(Ev::Click, |_| Msg::ToggleWatching),
            if model.event_streams.is_empty() {
                "Start watching"
            } else {
                "Stop watching"
            }
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
