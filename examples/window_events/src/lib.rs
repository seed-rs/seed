//! A simple, cliché example demonstrating structure and syntax.

#[macro_use]
extern crate seed;
use seed::prelude::*;


// Model

#[derive(Clone)]
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

impl Default for Model {
    fn default() -> Self {
        Self {
            watching: false,
            coords: (0, 0),
            last_keycode: 0,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    ToggleWatching,
    UpdateCoords(web_sys::MouseEvent),
    KeyPressed(web_sys::KeyboardEvent),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::ToggleWatching => Model {watching: !model.watching, ..model},
        Msg::UpdateCoords(ev) => Model {coords: (ev.screen_x(), ev.screen_y()), ..model},
        Msg::KeyPressed(ev) => Model {last_keycode: ev.key_code(), ..model}
    }
}


// View

/// This func demonstrates use of custom element tags, and the class! and
/// id! convenience macros
fn misc_demo() -> El<Msg> {
    let mut custom_el = El::empty(Tag::Custom("mytag".into()));
    custom_el.set_text("");  // Demo of set_text.

    let mut attributes = attrs!{};
    attributes.add_multiple("class", vec!["a-modicum-of", "hardly-any"]);

    custom![ Tag::Custom("superdiv".into()),
        p![ attributes ],

        // class! and id! convenience macros, if no other attributes are required.
        span![ class!["calculus", "chemistry", "literature"] ],
        span![ id!("unique-element") ],

        custom_el,
    ]
}

fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![
        h2![ model.coords_string() ],
        h2![ format!("Last key pressed: {}", model.last_keycode) ],
        button![
            simple_ev("click", Msg::ToggleWatching),
            if model.watching {"Stop watching"} else {"Start watching"}
        ],
        misc_demo()
    ]
}

fn window_events(model: Model) -> Vec<seed::Listener<Msg>> {
    let mut result = Vec::new();
    if model.watching {
        result.push(mouse_ev("mousemove", |ev| Msg::UpdateCoords(ev)));
        result.push(keyboard_ev("keydown", |ev| Msg::KeyPressed(ev)));
    }
    result
}


#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None, Some(window_events));
}