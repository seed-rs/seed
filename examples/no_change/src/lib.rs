use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter: 0,
        redraw_text_field: true,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    counter: i32,
    redraw_text_field: bool,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Increment,
    Decrement,
    ToggleRedrawTextField,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
        Msg::ToggleRedrawTextField => {
            log!("fsef");
            model.redraw_text_field = not(model.redraw_text_field);
        },
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::Decrement), "-"],
        div![model.counter],
        button![ev(Ev::Click, |_| Msg::Increment), "+"],
        fieldset![
            input![
                attrs!{
                    At::Value => model.counter,
                    At::Disabled => true.as_at_value(),
                }
            ],
            div![
                ev(Ev::Click, |_| Msg::ToggleRedrawTextField),
                input![
                    attrs!{
                        At::Type => "checkbox"
                        At::Checked => model.redraw_text_field.as_at_value(),
                    },
                ],
                label!["Redraw the text field on each render"],
            ]
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
