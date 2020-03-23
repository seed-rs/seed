use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
use web_sys::{self, DragEvent, Event, FileList};

// ------ ------
//     Model
// ------ ------

struct Model {
    drop_zone_active: bool,
    drop_zone_content: Vec<Node<Msg>>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            drop_zone_active: false,
            drop_zone_content: vec![div!["Drop files here"]],
        }
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    DragEnter,
    DragOver,
    DragLeave,
    Drop(FileList),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::DragEnter => model.drop_zone_active = true,
        Msg::DragOver => (),
        Msg::DragLeave => model.drop_zone_active = false,
        Msg::Drop(file_list) => {
            model.drop_zone_active = false;

            // FileList is not an iterator, so instead we iterate over (0..len(FileList)) range.
            // As `.item(index)` returns an `Option` we need to unwrap it.
            model.drop_zone_content = (0..file_list.length())
                .map(|index| file_list.item(index).unwrap())
                .map(|file| div![file.name()])
                .collect();
        }
    }
}

// ------ ------
//     View
// ------ ------

trait IntoDragEvent {
    fn into_drag_event(self) -> DragEvent;
}

impl IntoDragEvent for Event {
    fn into_drag_event(self) -> DragEvent {
        self.dyn_into::<web_sys::DragEvent>()
            .expect("cannot cast given event into DragEvent")
    }
}

// Note: It's macro so you can use it with all events.
macro_rules! stop_and_prevent {
    { $event:expr } => {
        {
            $event.stop_propagation();
            $event.prevent_default();
        }
     };
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        style![
            St::Height => px(200),
            St::Width => px(200),
            St::Margin => "auto",
            St::Background => if model.drop_zone_active { "lightgreen" } else { "lightgray" },
            St::FontFamily => "sans-serif",
            St::Display => "flex",
            St::FlexDirection => "column",
            St::JustifyContent => "center",
            St::AlignItems => "center",
            St::Border => [&px(2), "dashed", "black"].join(" ");
            St::BorderRadius => px(20),
        ],
        ev(Ev::DragEnter, |event| {
            stop_and_prevent!(event);
            Msg::DragEnter
        }),
        ev(Ev::DragOver, |event| {
            let drag_event = event.into_drag_event();
            stop_and_prevent!(drag_event);
            drag_event.data_transfer().unwrap().set_drop_effect("copy");
            Msg::DragOver
        }),
        ev(Ev::DragLeave, |event| {
            stop_and_prevent!(event);
            Msg::DragLeave
        }),
        ev(Ev::Drop, |event| {
            let drag_event = event.into_drag_event();
            stop_and_prevent!(drag_event);
            let file_list = drag_event.data_transfer().unwrap().files().unwrap();
            Msg::Drop(file_list)
        }),
        div![
            style! {
                // we don't want to fire `DragLeave` when we are dragging over drop-zone children
                St::PointerEvents => "none",
            },
            model.drop_zone_content.clone(),
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view).build_and_start();
}
