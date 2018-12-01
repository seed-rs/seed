extern crate wasm_bindgen;
// extern crate web_sys;
// extern crate framework;

use wasm_bindgen::prelude::*;

use framework::dom_types::{Attrs, Style, El, Events, Event, Tag};

use std::boxed::Box;

// The ELM Architecture (TEA)


// MODEL
// todo remove pub ?
#[derive(Clone, Debug)]
pub struct Model {
    pub clicks: i8,
    pub descrip: String,
}

impl Default for Model {
    // Initialize here, as in TEA.
    fn default() -> Self {
        Self {
            clicks: 0,
            descrip: "(Placeholder)".into(),
        }
    }
}


// UPDATE

pub enum Msg {
    Increment,
    Decrement,
    ChangeDescrip(String),
}

fn update(msg: Msg, model: Model) -> Model {
//    let model2 = model.clone(); // todo deal with this.
    match msg {
        Msg::Increment => {
            Model {clicks: model.clicks + 1, ..model}
        },
        Msg::Decrement => {
            Model {clicks: model.clicks - 1, ..model}
        },
        Msg::ChangeDescrip(descrip) => {
            Model {descrip, ..model}
        }
    }
}



// VIEW

fn comp(model: &Model) -> El<Msg> {
    let mut words = El::new(Tag::H2);
    words.text = Some("Hello, you!".into());

    let mut result = El::new(Tag::H3);
    result.text = Some(model.clicks.to_string());

    let mut button = El::new(Tag::Button);
    button.text = Some("Click me!".into());
    button.events = Events::new(vec![(Event::Click, Msg::Increment)]);

    let mut descrip = El::new(Tag::H2);
    descrip.text = Some(model.descrip.clone());

    div![ attrs!{"class" => "good elements"}, vec![
        div![attrs!{"class" => "ok elements"},
             style!{"color" => "purple"; "border" => "2px solid #004422"},
             vec![
                words,
                result,
                button
             ], ""],

        descrip
    ], "" ]
}


// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let model = Model::default();
    // todo make immutable again
    framework::vdom::mount::<Msg, Model>(model, &update, &comp, "main")
}