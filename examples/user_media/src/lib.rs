use seed::{document, window};
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlMediaElement, MediaStream, MediaStreamConstraints};

// Model

struct Model {}

// AfterMount

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(user_media());
    AfterMount::new(Model {})
}

async fn user_media() -> Result<Msg, Msg> {
    let mut constraints = MediaStreamConstraints::new();
    constraints.video(&JsValue::from(true));

    let media_stream_promise = window()
        .navigator()
        .media_devices()
        .unwrap()
        .get_user_media_with_constraints(&constraints)
        .unwrap();

    Ok(Msg::UserMedia(
        JsFuture::from(media_stream_promise)
            .await
            .map(MediaStream::from),
    ))
}

// Update

#[derive(Clone, Debug)]
enum Msg {
    UserMedia(Result<MediaStream, JsValue>),
}

fn update(msg: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UserMedia(Ok(media_stream)) => {
            let video_el = document()
                .query_selector("video")
                .unwrap()
                .unwrap()
                .dyn_into::<HtmlMediaElement>()
                .unwrap();

            video_el.set_src_object(Some(&media_stream));
        }
        Msg::UserMedia(Err(error)) => {
            log!(error);
        }
    }
}

// View

fn view(_: &Model) -> impl View<Msg> {
    video![attrs! {
        At::Width => 320,
        At::Height => 240,
        At::AutoPlay => AtValue::None,
    }]
}

#[wasm_bindgen(start)]
pub fn start() {
    seed::App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
