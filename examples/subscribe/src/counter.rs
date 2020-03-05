#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

#[derive(Copy, Clone)]
pub struct DoReset;

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    Model {
        value: 0,
        _sub_handle: orders.subscribe_with_handle(|_: DoReset| Msg::Reset),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    value: i32,
    _sub_handle: SubHandle,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    Increment,
    Decrement,
    Reset,
}

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::Increment => model.value += 1,
        Msg::Decrement => model.value -= 1,
        Msg::Reset => model.value = 0,
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        style! {St::TextAlign => "center"},
        button![ev(Ev::Click, |_| Msg::Decrement), "-"],
        div![model.value],
        button![ev(Ev::Click, |_| Msg::Increment), "+"],
    ]
}
