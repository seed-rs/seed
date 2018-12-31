//! A simple, cliché example demonstrating structure and syntax.

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone)]
struct Model {
    watching: bool,
    coords: (u32, u32),
}

impl Model {
    fn coords_string(&self) -> String {
        format!("X: {}, Y: {}", self.coords.0, self.coords.1)
    }
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            watching: false,
            coords: (0, 0),
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    ToggleWatching,
    UpdateCoords(web_sys::Event),

}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::ToggleWatching => Model {watching: !model.watching, ..model},
        Msg::UpdateCoords(window) => {


//            Model {coords: (x, y), ..model}
            model
        },
    }
}


// View

/// The top-level component we pass to the virtual dom. Must accept the model as its
/// only argument, and output a single El.
fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    // This view func demonstrates use of custom element tags.
    let mut custom_el = El::empty(Tag::Custom("mytag".into()));
    custom_el.set_text("Words");

    let mut attributes = attrs!{};
    attributes.add_multiple("class", vec!["a-modicum-of", "hardly-any"]);

    custom![ Tag::Custom("superdiv".into()),
        h2![ attributes, model.coords_string() ],

        // class! and id! convenience macros, if no other attributes are required.
        span![ class!["calculus", "chemistry", "literature"] ],
        span![ seed::id("unique-element") ],

        custom_el,
        button![ simple_ev("click", Msg::ToggleWatching), "Toggle watching" ]
    ]
}

fn window_events(model: Model) -> Vec<seed::Listener<Msg>> {
    let mut result = Vec::new();
    if model.watching {
        result.push(raw_ev("mousemove", |ev| Msg::UpdateCoords(ev)));
    }
    result
}


#[wasm_bindgen]
pub fn render() {
    // The final parameter is an optional routing map.
    seed::run(Model::default(), update, view, "main", None, Some(window_events));
}