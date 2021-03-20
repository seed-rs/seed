use seed::{prelude::*, *};

mod checkbox_tristate;
mod code_block;
mod feather_icon;
mod math_tex;

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

enum Msg {
    RotateCheckboxState,
}

#[allow(clippy::needless_pass_by_value)]
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::RotateCheckboxState => model.checkbox_state = model.checkbox_state.next(),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        div![
            "checkbox-tristate",
            checkbox_tristate::view("checkbox-tristate", "Label", model.checkbox_state, || {
                Msg::RotateCheckboxState
            }),
        ],
        hr![],
        div![
            "code-block",
            code_block::view("rust", "let number: Option<u32> = Some(10_200);"),
        ],
        hr![],
        div![
            "feather-icon",
            feather_icon::view("shopping-cart", None, None),
        ],
        hr![],
        div![
            "math-tex",
            math_tex::view(r"\mathbb{1} = \sum_i \lvert i \rangle \langle i \rvert"),
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
