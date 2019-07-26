//! A simple, cliché example demonstrating structure and syntax.

#[macro_use]
extern crate seed;
use seed::{events::Listener, prelude::*};

// Model

#[derive(Clone, Default)]
struct Model {
    watching: bool,
    coords: (i32, i32),
    last_keycode: u32,
}

impl Model {
    fn coords_string(&self) -> String {
        format!("X: {}, Y: {}", self.coords.0, self.coords.1)
    }
}

// Update

#[derive(Clone)]
enum Msg {
    ToggleWatching,
    UpdateCoords(web_sys::MouseEvent),
    KeyPressed(web_sys::KeyboardEvent),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleWatching => model.watching = !model.watching,
        Msg::UpdateCoords(ev) => model.coords = (ev.screen_x(), ev.screen_y()),
        Msg::KeyPressed(ev) => model.last_keycode = ev.key_code(),
    }
}

// View

/// This func demonstrates use of custom element tags, and the class! and
/// id! convenience macros
fn misc_demo() -> Node<Msg> {
    let custom_el = El::empty(Tag::Custom("mytag".into())).add_text(""); // Demo of add_text.

    let mut attributes = attrs! {};
    attributes.add_multiple(At::Class, &["a-modicum-of", "hardly-any"]);

    custom![
        Tag::Custom("superdiv".into()),
        p![attributes],
        // class! and id! convenience macros, if no other attributes are required.
        span![class!["calculus", "chemistry", "literature"]],
        span![id!("unique-element")],
        custom_el,
    ]
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        h2![model.coords_string()],
        h2![format!("Last key pressed: {}", model.last_keycode)],
        button![
            simple_ev(Ev::Click, Msg::ToggleWatching),
            if model.watching {
                "Stop watching"
            } else {
                "Start watching"
            }
        ],
        misc_demo(),
    ]
}

fn window_events(model: &Model) -> Vec<Listener<Msg>> {
    let mut result = Vec::new();
    if model.watching {
        result.push(mouse_ev(Ev::MouseMove, Msg::UpdateCoords));
        result.push(keyboard_ev(Ev::KeyDown, Msg::KeyPressed));
    }
    result
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .window_events(window_events)
        .finish()
        .run();
}
