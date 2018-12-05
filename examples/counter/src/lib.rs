//! A simple, cliché demonstrating the basics.

#[macro_use]
extern crate rebar;
use rebar::prelude::*;
use wasm_bindgen::prelude::*;

// Model

#[derive(Clone)]
struct Model {
    count: i32,
    what_we_count: &'static str
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click"
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeWWC()
}

// Sole source of updating the model; returns a whole new model.
fn update(msg: &Msg, model: &Model) -> Model {
    match msg {
        &Msg::Increment => {
            Model {count: model.count + 1, what_we_count: model.what_we_count}
        },
        &Msg::Decrement => {
            Model {count: model.count - 1, what_we_count: model.what_we_count}
        },
        &Msg::ChangeWWC() => {
//            Model {count: model.count, what_we_count: ev.target.value}
            Model {count: model.count, what_we_count: "Tester"}
        },
    }
}


// View

fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0 ... 3 => "Not very many 🙁",
        4 ... 7 => "An OK amount 😐",
        8 ... 999 => "Good job! 🙂",
        _ => "You broke it 🙃"
    };
    p![ descrip ]
}

// Top-level component we pass to the virtual dom. Must accept the model as its
// only argument, and output a single El.
fn main_comp(model: &Model) -> El<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

     div![ outer_style, vec![
        h1![ "The Grand Total" ],
        div![
            style!{
                "color" => if model.count > 4 {"purple"} else {"gray"};
                "border" => "2px solid #004422"
            },
            vec![
                h3![ format!("{} {}{} so far", model.count, model.what_we_count, plural) ],
                button![ events!{"click" => |_| Msg::Increment}, "+" ],
                button![ events!{"click" => |_| Msg::Decrement}, "-" ]
            ] ],
        success_level(model.count),

        h3![ "What precisely is it we're counting?" ],
//        input![ attrs!{"value" => model.what_we_count}, events!{
//            "change" => |ev: web_sys::Event| Msg::ChangeWWC(ev.target().value())
//        } ]
    ] ]
}


#[wasm_bindgen]
pub fn render() {
    rebar::vdom::run(Model::default(), update, main_comp, "main");
}