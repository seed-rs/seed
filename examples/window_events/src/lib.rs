use seed::{prelude::*, *};
use std::fmt;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    watching: bool,
    point: Point,
    key_code: u32,
}

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "X: {}, Y: {}", self.x, self.y)
    }
}

// ------ ------
// Window Events
// ------ ------

fn window_events(model: &Model) -> Vec<EventHandler<Msg>> {
    if !model.watching {
        return Vec::new();
    }
    vec![
        mouse_ev(Ev::MouseMove, Msg::MouseMoved),
        keyboard_ev(Ev::KeyDown, Msg::KeyPressed),
    ]
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ToggleWatching,
    MouseMoved(web_sys::MouseEvent),
    KeyPressed(web_sys::KeyboardEvent),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleWatching => model.watching = !model.watching,
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
        h2![model.point.to_string()],
        h2![format!("Last key pressed: {}", model.key_code)],
        button![
            ev(Ev::Click, |_| Msg::ToggleWatching),
            if model.watching {
                "Stop watching"
            } else {
                "Start watching"
            }
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .window_events(window_events)
        .build_and_start();
}
