//! A simple, cliché example demonstrating structure and syntax.

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone)]
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
fn view(model: &Model) -> El<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

     div![ outer_style, vec![
        h1![ "The Grand Total" ],
        div![
            style!{
                // Example of conditional logic in a style.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                // When passing numerical values to style!, "px" is implied.
                "border" => "2px solid #004422"; "padding" => 20
            },
            vec![
                // We can use normal Rust code in the view.
                h3![ format!("{} {}{} so far", model.count, model.what_we_count, plural) ],
                button![ vec![ seed::simple_event("click", Msg::Increment) ], "-" ],
                button![ vec![ seed::simple_event("click", Msg::Decrement) ], "-" ],

                // Optionally-displaying an element
                if model.count >= 10 { h2![ style!{"padding" => 50}, "Nice!" ] } else { seed::empty() }

            ] ],
        success_level(model.count),  // Incorporating a separate component

        h3![ "What precisely is it we're counting?" ],
        input![ attrs!{"value" => model.what_we_count},
                vec![ seed::input_event("input", |text: String| Msg::ChangeWWC(text)) ]
        ]

    ] ]
}


#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main");
}