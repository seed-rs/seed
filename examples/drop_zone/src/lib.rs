use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, DragEvent, Event, FileList};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        drop_zone_active: false,
        drop_zone_content: vec![div!["Drop files here"]],
        file_texts: Vec::new(),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    drop_zone_active: bool,
    drop_zone_content: Vec<Node<Msg>>,
    file_texts: Vec<String>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    DragEnter,
    DragOver,
    DragLeave,
    Drop(FileList),
    FileRead(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DragEnter => model.drop_zone_active = true,
        Msg::DragOver => (),
        Msg::DragLeave => model.drop_zone_active = false,
        Msg::Drop(file_list) => {
            model.drop_zone_active = false;
            model.file_texts.clear();

            // Note: `FileList` doesn't implement `Iterator`.
            let files = (0..file_list.length())
                .map(|index| file_list.get(index).expect("get file with given index"))
                .collect::<Vec<_>>();

            // Get file names.
            model.drop_zone_content = files.iter().map(|file| div![file.name()]).collect();

            // Read files (async).
            for file in files {
                orders.perform_cmd(async move {
                    let text =
                        // Convert `promise` to `Future`.
                        JsFuture::from(file.text())
                            .await
                            .expect("read file")
                            .as_string()
                            .expect("cast file text to String");
                    Msg::FileRead(text)
                });
            }
        }
        Msg::FileRead(text) => model.file_texts.push(text + "\n"),
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

fn view(model: &Model) -> Node<Msg> {
    div![
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
            ],
        ],
        if model.file_texts.is_empty() {
            div!["Uploaded texts appear here"]
        } else {
            pre![&model.file_texts]
        }
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
