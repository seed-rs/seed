// NOTE: Don't try to create as many components as possible.
// Try to reuse already existing `Msg` and other entities to prevent unnecessary nesting and complexity.

use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    value: i32,
}

// ------ ------
//    Update
// ------ ------

#[derive(Copy, Clone)]
pub enum Msg {
    Increment,
    Decrement,
}

pub fn update<Ms: 'static>(
    msg: Msg,
    model: &mut Model,
    on_change: impl FnOnce(i32) -> Ms,
    orders: &mut impl Orders<Ms>,
) {
    match msg {
        Msg::Increment => model.value += 1,
        Msg::Decrement => model.value -= 1,
    }
    orders.send_msg(on_change(model.value));
}

// ------ ALTERNATIVE update ------

// pub enum OutMsg {
//     Changed
// }
//
// pub fn update<Ms: 'static>(msg: Msg, model: &mut Model) -> OutMsg {
//     match msg {
//         Msg::Increment => model.value += 1,
//         Msg::Decrement => model.value -= 1,
//     }
//     OutMsg::Changed
// }

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(
    model: &Model,
    on_click: impl FnOnce() -> Ms + Clone + 'static,
    to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
    div![
        ev(Ev::Click, |_| on_click()),
        button![
            ev(Ev::Click, {
                let to_msg = to_msg.clone();
                move |_| to_msg(Msg::Decrement)
            }),
            "-"
        ],
        div![model.value.to_string()],
        button![ev(Ev::Click, move |_| to_msg(Msg::Increment)), "+"],
    ]
}
