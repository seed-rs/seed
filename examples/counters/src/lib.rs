use seed::{prelude::*, *};

mod counter;

type CounterId = usize;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counters: (0..3).map(|_| counter::init()).collect(),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    counters: Vec<counter::Model>,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone, Copy)]
enum Msg {
    Counter(counter::Msg, CounterId),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Counter(msg, id) => counter::update(msg, &mut model.counters[id]),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        style! { St::Display => "flex" },
        model.counters.iter().enumerate().map(|(id, model)| {
            counter::view(*model).map_msg(move |counter_msg| Msg::Counter(counter_msg, id))
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
