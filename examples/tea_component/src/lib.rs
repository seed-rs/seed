// NOTE: Don't try to create as many components as possible.
// Try to reuse already existing `Msg` and other entities to prevent unnecessary nesting and complexity.

use seed::{prelude::*, *};

mod counter;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    counter_a: counter::Model,
    counter_b: counter::Model,
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone)]
enum Msg {
    CounterA(counter::Msg),
    CounterB(counter::Msg),
    CounterClicked(char),
    CounterChanged(i32),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CounterA(msg) => {
            counter::update(msg, &mut model.counter_a, Msg::CounterChanged, orders)
        }
        Msg::CounterB(msg) => {
            counter::update(msg, &mut model.counter_b, Msg::CounterChanged, orders)
        }
        Msg::CounterClicked(counter_id) => log!("CounterClicked", counter_id),
        Msg::CounterChanged(value) => log!("CounterChanged", value),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        style! { St::Display => "flex"},
        counter::view(&model.counter_a, || Msg::CounterClicked('A'), Msg::CounterA),
        counter::view(&model.counter_b, || Msg::CounterClicked('B'), Msg::CounterB),
        "See Console log",
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
