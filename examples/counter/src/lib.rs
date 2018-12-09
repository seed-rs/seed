//! A simple, cliché example demonstrating the basics.

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone, Debug)]
struct Model {
    count: i32,
    what_we_count: String
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into()
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
//    ChangeWWC(web_sys::Event),
    ChangeWWC(String),
    KeyTest(web_sys::Event),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: &Msg, model: &Model) -> Model {
    match msg {
        Msg::Increment => {
            Model {count: model.count + 1, what_we_count: model.what_we_count.clone()}
        },
        Msg::Decrement => {
            Model {count: model.count - 1, what_we_count: model.what_we_count.clone()}
        },
        Msg::ChangeWWC(text) => {
            Model {count: model.count, what_we_count: text.clone()}
        }

//        Msg::ChangeWWC(ev) => {
//            let text = match ev.target() {
//                Some(et) => {
//                    (et.unchecked_ref() as &web_sys::HtmlInputElement).value()
//                },
//                None => String::from("Error"),
//            };
//            Model {count: model.count, what_we_count: text}
//        },
        Msg::KeyTest(ev) => {
//
//            let text = match ev.target() {
//                Some(et) => {
//                    seed::log("KEY down");
//                    (et.unchecked_ref() as &web_sys::HtmlInputElement).value()
//                },
//                None => String::from("Error"),
//            };
//            seed::log(&text);
            Model {count: model.count, what_we_count: "TEMP".into()}
        },
    }
}


// View

/// A simple component.
fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0 ... 3 => "Not very many 🙁",
        4 ... 7 => "An OK amount 😐",
        8 ... 999 => "Good job! 🙂",
        _ => "You broke it 🙃"
    };
    p![ descrip ]
}

/// The top-level component we pass to the virtual dom. Must accept a ref to the model as its
/// only argument, and output a single El.
fn main_comp(model: &Model) -> El<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    // Attrs, Style, and Events may be defined separately, and passed into
    // element macros.
    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center";
            "margin" => "auto"
    };

//     div![ outer_style, &model.count.to_string(), vec![
     div![ outer_style, vec![
        h1![ "The Grand Total" ],
        div![
            style!{
                // Example of conditional logic in a style.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                // When passing numerical values to style!, "px" is implied.
                // If you want a different unit, use a str.
                "border" => "2px solid #004422"; "padding" => 20
            },
            vec![
                // We can insert normal Rust code in the view, without speical syntax.
                h3![ format!("{} {}{} so far", model.count, model.what_we_count, plural) ],
                button![ events!{"click" => |_| Msg::Increment}, "+" ],
                button![ events!{"click" => |_| Msg::Decrement}, "-" ],

                // An example of optionally-displaying an element: Note that with
                // ternary logic like this, we must pass an element on both branches.
                // If you wish to avoid the extra el, you can construct the children Vec
                // separately, and optionally append the El.
                if model.count >= 10 { h2![ style!{"padding" => 50}, "Nice!" ] } else { span![] }

            ] ],
        success_level(model.count),

        h3![ "What precisely is it we're counting?" ],
        input![ attrs!{"value" => model.what_we_count}, events!{
            "input" => |ev| Msg::ChangeWWC(ev)
//            "keydown" => |ev: web_sys::Event| Msg::KeyTest(ev)
        } ]
    ] ]
}


#[wasm_bindgen]
pub fn render() {
    seed::vdom::run(Model::default(), update, main_comp, "main");
}