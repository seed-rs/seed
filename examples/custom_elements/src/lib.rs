use seed::{prelude::*, *};

mod checkbox_tristate;

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
    pub checkbox_state: checkbox_tristate::State,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone, Copy)]
enum Msg {
    RotateCheckboxState,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::RotateCheckboxState => model.checkbox_state = model.checkbox_state.next(),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    span![
        style! {
            St::Cursor => "pointer",
            St::UserSelect => "none",
        },
        checkbox_tristate::view(model.checkbox_state),
        ev(Ev::Click, |_| Msg::RotateCheckboxState),
        "checkbox-tristate",
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
