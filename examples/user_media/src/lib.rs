use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlMediaElement, MediaStream, MediaStreamConstraints};

// Model

#[derive(Default)]
struct Model {
    video: ElRef<HtmlMediaElement>,
}

// AfterMount

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(user_media());
    AfterMount::new(Model::default())
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

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UserMedia(Ok(media_stream)) => {
            model
                .video
                .get()
                .expect("get video element")
                .set_src_object(Some(&media_stream));
        }
        Msg::UserMedia(Err(error)) => {
            log!(error);
        }
    }
}

// View

fn view(model: &Model) -> impl View<Msg> {
    video![
        el_ref(&model.video),
        attrs! {
            At::Width => 320,
            At::Height => 240,
            At::AutoPlay => AtValue::None,
        }
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
