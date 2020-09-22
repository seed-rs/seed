#![allow(clippy::needless_pass_by_value)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter: 0,
        redraw_text_field: true,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    counter: i32,
    redraw_text_field: bool,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Increment,
    Decrement,
    ToggleRedrawTextField,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
        Msg::ToggleRedrawTextField => model.redraw_text_field = not(model.redraw_text_field),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::Decrement), "-"],
        div![model.counter],
        button![ev(Ev::Click, |_| Msg::Increment), "+"],
        fieldset![
            style! {St::MarginTop => px(10), St::Width => rem(20)},
            if model.redraw_text_field {
                input![attrs! {
                    At::Value => model.counter,
                    At::Disabled => true.as_at_value(),
                }]
            } else {
                Node::NoChange
            },
            div![
                ev(Ev::Click, |_| Msg::ToggleRedrawTextField),
                input![attrs! {
                    At::Type => "checkbox"
                    At::Checked => model.redraw_text_field.as_at_value(),
                },],
                label!["Redraw the text field on each render"],
            ]
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
