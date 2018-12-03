use wasm_bindgen::prelude::*;

// This prelude is the equivalent of the following imports:
// use rebar::dom_types::{El, Style, Attrs, Tag, Event, Events, UpdateEl};
// use rebar::vdom::run;
use crate::prelude::*;

// Todo trait etc that prevents the user from having to enter <Msg> with each El?

// Model

#[derive(Clone, Debug)] // todo
struct Model {
    pub clicks: i32,
    pub what_we_count: String,
}

// Set the starting state here, for initialization in the render() function.
impl Default for Model {
    fn default() -> Self {
        Self {
            clicks: 0,
            what_we_count: "click".into(),
        }
    }
}

// Update

pub enum Msg {  // todo temp pub
    Increment,
    Decrement,
    ChangeDescrip(String),
}

fn update(msg: &Msg, model: &Model) -> Model {
    let model2 = model.clone(); // todo deal with this.
    match msg {
        &Msg::Increment => {
            Model {clicks: model.clicks + 1, ..model2}
        },
        &Msg::Decrement => {
            Model {clicks: model.clicks - 1, ..model2}
        },
        &Msg::ChangeDescrip(ref descrip) => {
            Model { what_we_count: descrip.to_string(), ..model2}
        }
    }
}

// View

// An example component; it's just a Rust function that returns an element.
fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0 ... 3 => "Not very many 🙁",
        4 ... 7 => "An OK amount 😐",
        8 ... 999 => "Good job! 🙂",
        _ => "You broke it 🙃"
    };
    p![ descrip ]  // We could add children here if we wanted
}

// Top-level component we pass to the virtual dom. Must accept the model as its only argument.
fn comp(model: &Model) -> El<Msg> {
    // Attributes, styles, events, text, and children can be created inside the
    // element macros, or separately.
    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

    let mut button1 = button![ events!{"click" => Msg::Increment}, "Click me" ];
//    button1.add_ev("click", |_| Msg::Increment);
    let mut button2 = button![ events!{"contextmenu" => Msg::Decrement}, "Don't click me" ];
//    button2.add_ev("dblclick", |_| Msg::Decrement);

    div![outer_style, vec![
        div![
            attrs!{"class" => "ok elements"},
            style!{
                "color" => if model.clicks > 4 {"purple"} else {"gray"};
                "border" => "2px solid #004422"
            },
            // This is normal Rust code, so you can insert comments whever you like.
            vec![
                h1![ "Counting" ],
                h3![ format!("{} {}(s) so far", model.clicks + 1, model.what_we_count) ],
                button1,
                button2

            ] ],
        success_level(model.clicks),
    ] ]
}

#[wasm_bindgen]
pub fn render() {
    run(Model::default(), update, comp, "main");
}