#[macro_use]
extern crate seed;
use seed::prelude::*;
use seed::{Method, Request};

use futures::Future;

use shared::Data;

// Model

#[derive(Default)]
struct Model {
    pub data: Data,
}

fn get_data() -> impl Future<Item = Msg, Error = Msg> {
    let url = "http://localhost:8001/data";

    Request::new(url)
        .method(Method::Get)
        .fetch_json()
        .map(Msg::Replace)
        .map_err(Msg::OnFetchErr)
}

// Update

#[derive(Clone)]
enum Msg {
    GetData,
    Replace(Data),
    OnFetchErr(JsValue),
}

fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Replace(data) => {
            model.data = data;
            Render.into()
        }

        Msg::GetData => Update::with_future_msg(get_data()).skip(),

        Msg::OnFetchErr(err) => {
            log!(format!("Fetch error: {:?}", err));
            Skip.into()
        }
    }
}

// View

fn view(model: &Model) -> Vec<El<Msg>> {
    vec![
        h1![format!("Val: {} Text: {}", model.data.val, model.data.text)],
        button![
            raw_ev("click", move |_| Msg::GetData),
            "Update data"
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}
