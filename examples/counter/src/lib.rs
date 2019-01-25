//! A simple, cliché example demonstrating structure and syntax.

#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

#[derive(Clone)]
struct Model {
    count: i32,
    what_we_count: String,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into(),
        }
    }
}

// Update

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeWWC(String),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Update<Model> {
    match msg {
        Msg::Increment => Render(Model {
            count: model.count + 1,
            ..model
        }),
        Msg::Decrement => Render(Model {
            count: model.count - 1,
            ..model
        }),
        Msg::ChangeWWC(what_we_count) => Render(Model {
            what_we_count,
            ..model
        }),
    }
}

/// A mutable-style alternative:
//fn update(msg: Msg, model: Model) -> ModelUpdate<Model> {
//    let mut model = model;
//    match msg {
//        Msg::Increment => model.count += 1,
//        Msg::Decrement => model.count -= 1,
//        Msg::ChangeWWC(what_we_count) => model.what_we_count = what_we_count,
//    }
//    Render(model)
//}

// View

/// A simple component.
fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0...5 => "Not very many 🙁",
        6...9 => "I got my first real six-string 😐",
        10...11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃",
    };
    p![descrip]
}

/// The top-level component we pass to the virtual dom. Must accept the model as its
/// only argument, and output a single El.
fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    let plural = if model.count == 1 { "" } else { "s" };
    let text = format!("{} {}{} so far", model.count, model.what_we_count, plural);

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style! {
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

    div![
        outer_style,
        h1!["The Grand Total"],
        div![
            style! {
                // Example of conditional logic in a style.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                // When passing numerical values to style!, "px" is implied.
                "border" => "2px solid #004422"; "padding" => 20
            },
            // We can use normal Rust code and comments in the view.
            h3![text, did_update(|_| log!("This shows when we increment"))],
            button![simple_ev("click", Msg::Increment), "+"],
            button![simple_ev("click", Msg::Decrement), "-"],
            // Optionally-displaying an element, and lifecycle hooks
            if model.count >= 10 {
                h2![
                    style! {"padding" => 50},
                    "Nice!",
                    did_mount(|_| log!("This shows when clicks reach 10")),
                    will_unmount(|_| log!("This shows when clicks drop below 10")),
                ]
            } else {
                seed::empty()
            }
        ],
        success_level(model.count), // Incorporating a separate component
        h3!["What precisely is it we're counting?"],
        input![
            attrs! {"value" => model.what_we_count},
            input_ev("input", Msg::ChangeWWC)
        ],
    ]
}

#[wasm_bindgen]
pub fn render() {
    let app_builder = seed::App::build(Model::default(), update, view);

    app_builder.clone().mount("main").finish().run();
    // app_builder.clone().mount("app-2").finish().run();
}
