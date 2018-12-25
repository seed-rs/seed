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
    ChangeWWC(String),

    Focus(&'static str),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model {count: model.count + 1, ..model},
        Msg::Decrement => Model {count: model.count - 1, ..model},
        Msg::ChangeWWC(what_we_count) => Model {what_we_count, ..model },

        Msg::Focus(el_id) => {
            let input = seed::document().get_element_by_id(el_id).unwrap();
            let input = seed::to_html_el(&input);
            input.focus().unwrap();
            model
        }
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

/// The top-level component we pass to the virtual dom. Must accept the model as its
/// only argument, and output a single El.
fn view(model: Model) -> El<Msg> {
    let plural = if model.count == 1 {""} else {"s"};
    let text = format!("{} {}{} so far", model.count, model.what_we_count, plural);

//    let autofocus = |el: &web_sys::Element| {
//                log!("Mounted the thang");
//                let html_el = seed::to_html_el(&el);
//                html_el.focus().unwrap();
//    };

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

    div![ outer_style,
        h1![ "The Grand Total" ],
        div![
            style!{
                // Example of conditional logic in a style.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                // When passing numerical values to style!, "px" is implied.
                "border" => "2px solid #004422"; "padding" => 20
            },
                // We can use normal Rust code and comments in the view.
                h3![ text, did_update(|el| log!("This shows when we increment")) ],
                button![ simple_ev("click", Msg::Increment), "+" ],
                button![ simple_ev("click", Msg::Decrement), "-" ],

                // Optionally-displaying an element, and lifecycle hooks
                if model.count >= 10 {
                    h2![ style!{"padding" => 50}, "Nice!",
                         did_mount(|el| log!("This shows when clicks reach 10")),
                         will_unmount(|el| log!("This shows when clicks drop below 10")),
                    ]
                } else { seed::empty() }

            ],
        success_level(model.count),  // Incorporating a separate component

        h3![ "What precisely is it we're counting?" ],
        input![ attrs!{"value" => model.what_we_count}, input_ev("input", Msg::ChangeWWC) ],
    ]
}


#[wasm_bindgen]
pub fn render() {
    // The final parameter is an optional routing map.
    seed::run(Model::default(), update, view, "main", None);
}