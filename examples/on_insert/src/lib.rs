#![allow(clippy::needless_pass_by_value)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model { inputs: vec![1] }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    inputs: Vec<u32>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    AddBox,
    RemoveBox(u32),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddBox => model
            .inputs
            .push(model.inputs.iter().max().unwrap_or(&0) + 1),
        Msg::RemoveBox(id_to_remove) => model.inputs.retain(|id| id != &id_to_remove),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        style! {
            St::Width => vw(100),
            St::Height => vh(100),
            St::Display => "flex",
            St::FlexDirection => "column",
            St::JustifyContent => "center",
            St::AlignItems => "center",
        },
        button![ev(Ev::Click, |_| Msg::AddBox), "Add Input"],
        model.inputs.iter().map(|id| {
            let id = *id;
            div![
                input![on_insert(|el| el
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap()
                    .focus()
                    .unwrap())],
                button![ev(Ev::Click, move |_| Msg::RemoveBox(id)), "Remove"]
            ]
        })
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
