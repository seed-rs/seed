use seed::{prelude::*, *};
use std::cell::RefCell;

thread_local!(static TITLE: RefCell<String> = RefCell::new("I'm TITLE!".to_owned()));

#[wasm_bindgen]
pub fn set_title(title: String) {
    TITLE.with(|title_cell| title_cell.replace(title));
}

fn title() -> String {
    TITLE.with(|title_cell| title_cell.borrow().clone())
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model
}

// ------ ------
//     Model
// ------ ------

struct Model;

// ------ ------
//    Update
// ------ ------

#[derive(Clone, Copy)]
enum Msg {
    Rerender,
}

fn update(msg: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rerender => log!("Rerendered"),
    }
}

// ------ ------
//     View
// ------ ------

fn view(_: &Model) -> Vec<Node<Msg>> {
    vec![
        h1![title()],
        button!["Rerender", ev(Ev::Click, |_| Msg::Rerender)],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
