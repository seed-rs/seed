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

enum Msg {
    Increment(i32),
    Decrement(i32),
}

#[allow(clippy::needless_pass_by_value)]
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment(d) => *model += d,
        Msg::Decrement(d) => *model -= d,
    }
}

// ------ ------
//     View
// ------ ------

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> Node<Msg> {
    div![
        style! {
            St::Display => "flex"
            St::AlignItems => "center",
        },
        comp![Button { label: "-100" },
            disabled => true,
            on_click => || Msg::Decrement(100),
        ],
        comp![Button { label: "-10" }, on_click => || Msg::Decrement(10)],
        comp![Button { label: "-1" },
            outlined => true,
            on_click => || Msg::Decrement(1),
        ],
        div![style! { St::Margin => "0 1em" }, model],
        comp![Button { label: "+1" },
            outlined => true,
            on_click => || Msg::Increment(1),
        ],
        comp![Button { label: "+10" }, on_click => || Msg::Increment(10)],
        comp![Button { label: "+100" },
              disabled => true,
              on_click => || Msg::Increment(100),
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
