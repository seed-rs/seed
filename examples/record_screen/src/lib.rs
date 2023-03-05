use gloo_console::log;
use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;
use web_sys::{DisplayMediaStreamConstraints, HtmlMediaElement, MediaStream};

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
    video: ElRef<HtmlMediaElement>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    RecordScreen,
    DisplayMedia(Result<MediaStream, JsValue>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RecordScreen => {
            orders.perform_cmd(display_media());
        }
        Msg::DisplayMedia(Ok(media_stream)) => {
            model
                .video
                .get()
                .expect("get video element")
                .set_src_object(Some(&media_stream));
        }
        Msg::DisplayMedia(Err(error)) => {
            log!(format!("{error:?}"));
        }
    }
}

async fn display_media() -> Msg {
    let mut constraints = DisplayMediaStreamConstraints::new();
    constraints.video(&JsValue::from(true));

    let media_stream_promise = window()
        .navigator()
        .media_devices()
        .unwrap()
        .get_display_media_with_constraints(&constraints)
        .unwrap();

    Msg::DisplayMedia(
        JsFuture::from(media_stream_promise)
            .await
            .map(MediaStream::from),
    )
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        button![
            style! {
                St::Display => "block",
            },
            "Record Screen",
            ev(Ev::Click, |_| Msg::RecordScreen)
        ],
        video![
            el_ref(&model.video),
            attrs! {
                At::Width => 1024,
                At::AutoPlay => AtValue::None,
            }
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
