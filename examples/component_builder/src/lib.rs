use seed::{prelude::*, *};

mod button;
use button::Button;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

type Model = i32;

// ------ ------
//    Update
// ------ ------

#[derive(Copy, Clone)]
enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
        Msg::Decrement => *model -= 1,
    }
}

// ------ ------
//     View
// ------ ------

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> Node<Msg> {
    div![
        style! { St::Display => "flex" },
        Button::new("-")
            .disabled(true)
            .add_on_click(|_| Msg::Decrement),
        Button::new("-")
            .secondary()
            .large()
            .outline()
            .add_on_click(|_| Msg::Decrement),
        Button::new("-").add_on_click(|_| Msg::Decrement),
        div![model],
        Button::new("+").add_on_click(|_| Msg::Increment),
        Button::new("+")
            .secondary()
            .large()
            .outline()
            .add_on_click(|_| Msg::Increment),
        Button::new("+")
            .disabled(true)
            .add_on_click(|_| Msg::Increment),
        Button::new("seed-rs.org").a("https://seed-rs.org"),
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
