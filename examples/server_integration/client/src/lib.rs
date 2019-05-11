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

fn get_data() -> impl Future<Item=Msg, Error=Msg> {
    Request::new("/data")
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

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::Replace(data) => model.data = data,

        Msg::GetData => {
            orders
                .skip()
                .perform_cmd(get_data());
        }

        Msg::OnFetchErr(err) => {
            log!(format!("Fetch error: {:?}", err));
            orders.skip();
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
pub fn start() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}
