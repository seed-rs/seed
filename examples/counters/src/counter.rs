use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub const fn init() -> Model {
    0
}

// ------ ------
//     Model
// ------ ------

pub type Model = i32;

// ------ ------
//    Update
// ------ ------

#[derive(Clone, Copy)]
pub enum Msg {
    Increment,
    Decrement,
}

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::Increment => *model += 1,
        Msg::Decrement => *model -= 1,
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::Decrement), "-"],
        div![model],
        button![ev(Ev::Click, |_| Msg::Increment), "+"],
    ]
}
