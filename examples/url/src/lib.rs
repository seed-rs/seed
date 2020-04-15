use seed::{prelude::*, *};
use std::rc::Rc;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        base_path: orders.clone_base_path(),
        base_url: url.to_base_url(),
        initial_url: url,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    base_path: Rc<Vec<String>>,
    initial_url: Url,
    base_url: Url,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    ol![
        li![
            format!("Base path: \"{}\"", &model.base_path.join("/")),
        ],
        li![
            format!("Initial Url: \"{}\"", &model.initial_url),
        ],
        li![
            format!("Base Url: \"{}\"", &model.base_url),
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
