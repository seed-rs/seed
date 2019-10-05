#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self, DragEvent, Event, FileList};

// Model

struct Model {
    drop_zone_active: bool,
    drop_zone_content: Vec<Node<Msg>>,
}

// Init

fn init(_: Url, _: &mut impl Orders<Msg>) -> Init<Model> {
    Init::new(Model {
        drop_zone_active: false,
        drop_zone_content: vec![
            div!["Drop files here"],
        ]
    })
}

// Update

#[derive(Clone, Debug)]
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

            let mut files = Vec::new();
            for index in 0..file_list.length() {
                files.push(file_list.item(index).unwrap());
            }

            model.drop_zone_content = files
                .iter()
                .map(|file| div![file.name()])
                .collect();
        }
    }
}

// View

trait IntoDragEvent {
    fn into_drag_event(self) -> DragEvent;
}
impl IntoDragEvent for Event {
    fn into_drag_event(self) -> DragEvent {
        self.dyn_into::<web_sys::DragEvent>()
            .expect("cannot cast given event into DragEvent")
    }
}

// Note: It's macro so you can use it with all events
macro_rules! stop_and_prevent {
    { $event:expr } => {
        {
            $event.stop_propagation();
            $event.prevent_default();
        }
     };
}

fn view(model: &Model) -> impl View<Msg> {
    log!("DROP EXAMPLE", model.drop_zone_content);
    div![
        style![
            St::Height => px(200),
            St::Width => px(200),
            St::Margin => "auto",
            St::Background => if model.drop_zone_active { "lightgreen" } else { "lightgray" },
            St::FontFamily => "Sans-Serif",
            St::Display => "flex",
            St::FlexDirection => "column",
            St::JustifyContent => "center",
            St::AlignItems => "center",
            St::Border => [&px(2), "dashed", "black"].join(" ");
            St::BorderRadius => px(20),
        ],
        raw_ev(Ev::DragEnter, |event| {
            stop_and_prevent!(event);
            Msg::DragEnter
        }),
        raw_ev(Ev::DragOver, |event| {
            let drag_event = event.into_drag_event();
            stop_and_prevent!(drag_event);
            drag_event.data_transfer().unwrap().set_drop_effect("copy");
            Msg::DragOver
        }),
        raw_ev(Ev::DragLeave, |event| {
            stop_and_prevent!(event);
            Msg::DragLeave
        }),
        raw_ev(Ev::Drop, |event| {
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
            model.drop_zone_content.clone()
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    seed::App::build(init, update, view).finish().run();
}
