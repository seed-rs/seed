#![allow(clippy::enum_variant_names, clippy::large_enum_variant)]

use seed::{prelude::*, *};

mod example_a;
mod example_b;
mod example_c;
mod example_d;
mod example_e;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    example_a: example_a::Model,
    example_b: example_b::Model,
    example_c: example_c::Model,
    example_d: example_d::Model,
    example_e: example_e::Model,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ExampleA(example_a::Msg),
    ExampleB(example_b::Msg),
    ExampleC(example_c::Msg),
    ExampleD(example_d::Msg),
    ExampleE(example_e::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ExampleA(msg) => {
            example_a::update(msg, &mut model.example_a, &mut orders.proxy(Msg::ExampleA));
        }
        Msg::ExampleB(msg) => {
            example_b::update(msg, &mut model.example_b, &mut orders.proxy(Msg::ExampleB));
        }
        Msg::ExampleC(msg) => {
            example_c::update(msg, &mut model.example_c, &mut orders.proxy(Msg::ExampleC));
        }
        Msg::ExampleD(msg) => {
            example_d::update(msg, &mut model.example_d, &mut orders.proxy(Msg::ExampleD));
        }
        Msg::ExampleE(msg) => {
            example_e::update(msg, &mut model.example_e, &mut orders.proxy(Msg::ExampleE));
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        style! {
            St::FontFamily => "sans-serif";
            St::MaxWidth => px(460);
            St::Margin => "auto";
        },
        example_a::view(&model.example_a, view_intro).map_msg(Msg::ExampleA),
        example_b::view(&model.example_b, view_intro).map_msg(Msg::ExampleB),
        example_c::view(&model.example_c, view_intro).map_msg(Msg::ExampleC),
        example_d::view(&model.example_d, view_intro).map_msg(Msg::ExampleD),
        example_e::view(&model.example_e, view_intro).map_msg(Msg::ExampleE),
    ]
}

fn view_intro<Ms>(title: &str, description: &str) -> Vec<Node<Ms>> {
    vec![
        hr![],
        h2![title],
        div![style! {St::MarginBottom => px(15)}, description],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
