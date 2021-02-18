use seed::{prelude::*, *};
use web_sys::{self, HtmlDivElement};
// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        drop_zone: ElRef::new(),
        drag_me_1: ElRef::new(),
        drag_me_2: ElRef::new(),
        who_is_getting_dragged: None,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    drop_zone: ElRef<HtmlDivElement>,
    drag_me_1: ElRef<HtmlDivElement>,
    drag_me_2: ElRef<HtmlDivElement>,
    who_is_getting_dragged: Option<u8>,
}

type MouseEntered = bool;
type Id = u8;
// ------ ------
//    Update
// ------ ------

enum Msg {
    Dragging(Id),
    DragEnded(Id),
    MouseChange(Id, MouseEntered),
    DragOver,
    Drop,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Drop => {
            if let Some(id) = model.who_is_getting_dragged {
                let div_drag = get_div_from_id(id, &model);

                let zone: HtmlDivElement =
                    model.drop_zone.get().expect("should get the div drop zone");
                let res = zone.append_child(&div_drag);
                if res.is_ok() {
                    log!("it worked");
                } else {
                    log!("it did failed")
                }
            }
            model.who_is_getting_dragged = None;
        }
        Msg::MouseChange(id, enter) => {
            let div_drag = get_div_from_id(id, &model);
            let text = if enter {
                "Drag me and let's go!"
            } else {
                "Put mouse on me"
            };
            let draggable = enter;
            div_drag.set_inner_text(text);
            div_drag.set_draggable(draggable);
        }
        Msg::Dragging(id) => {
            let div_drag = get_div_from_id(id, &model);
            div_drag.set_inner_text("OUUUUUUUUUUUUUIII");
            model.who_is_getting_dragged = Some(id);
        }
        Msg::DragEnded(id) => {
            let div_drag = get_div_from_id(id, &model);

            if let Some(container) = div_drag.parent_element() {
                if container.id() != "drop_zone" {
                    div_drag.set_inner_text("NOO :(");
                } else {
                    div_drag.set_inner_text("Now I am happy");
                }
            }
            model.who_is_getting_dragged = None
        }
        Msg::DragOver => {}
    }
}

fn get_div_from_id(id: Id, model: &Model) -> HtmlDivElement {
    match id {
        1 => model.drag_me_1.get().expect("should get div 1"),
        2 => model.drag_me_2.get().expect("should get div 2"),
        _ => {
            panic!("wrong id");
        }
    }
}
// ------ ------
//     View
// ------ ------

// Note: It's macro so you can use it with all events.
macro_rules! stop_and_prevent {
    { $event:expr } => {
        {
            $event.stop_propagation();
            $event.prevent_default();
        }
     };
}

fn view(model: &Model) -> Node<Msg> {
    div![
        generate_draggable_div(2, "I am number1", &model.drag_me_2, "green"),
        generate_draggable_div(1, "I am number1", &model.drag_me_1, "purple"),
        div![
            id!("drop_zone"),
            el_ref(&model.drop_zone),
            "drop stuff there",
            style![
                St::Height => px(150),
                St::Width => px(150),
                St::Margin => "auto",
                St::FontFamily => "sans-serif",
                St::Display => "flex",
                St::FlexDirection => "column",
                St::TextAlign=> "center",
                St::JustifyContent => "center",
                St::AlignItems => "center",
                St::Border => [&px(2), "dotted", "green"].join(" ");
                St::BorderRadius => px(20),
            ],
            ev(Ev::Drop, |_| { Msg::Drop }),
            drag_ev(Ev::DragOver, |event| {
                stop_and_prevent!(event);
                event.data_transfer().unwrap().set_drop_effect("move");
                Msg::DragOver
            })
        ]
    ]
}

fn generate_draggable_div(
    id: u8,
    default_msg: &str,
    div: &ElRef<HtmlDivElement>,
    color: &str,
) -> Node<Msg> {
    div![
        default_msg,
        id!(id),
        el_ref(div),
        style![
            St::Height => px(50),
            St::Width => px(50),
            St::FontFamily => "sans-serif",
            St::FontSize=>"9px",
            St::Display => "flex",
            St::Cursor => if can_be_dragged(div) {"grab"} else {"default"},
            St::FlexDirection => "column",
            St::JustifyContent => "center",
            St::AlignItems => "center",
            St::Border => [&px(2), "solid", color].join(" ");
            St::BorderRadius => px(20),
        ],
        ev(Ev::MouseOver, move |_| { Msg::MouseChange(id, true) }),
        ev(Ev::MouseLeave, move |_| { Msg::MouseChange(id, false) }),
        ev(Ev::DragStart, move |_| { Msg::Dragging(id) }),
        ev(Ev::DragEnd, move |_| { Msg::DragEnded(id) })
    ]
}

fn can_be_dragged(div: &ElRef<HtmlDivElement>) -> bool {
    div.get()
        .map(|div_drag| div_drag.draggable())
        .unwrap_or_default()
}
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
