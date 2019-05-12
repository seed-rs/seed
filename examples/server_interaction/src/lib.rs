//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! https://serde.rs/

#[macro_use]
extern crate seed;

use futures::Future;
use seed::prelude::*;
use seed::{Method, Request};
use serde::{Deserialize, Serialize};

// Model

#[derive(Clone, Serialize, Deserialize)]
struct Data {
    pub val: u32,
    pub text: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Commit {
    pub sha: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Serialize)]
struct Message {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Clone, Deserialize, Debug)]
struct ServerResponse {
    pub success: bool,
}

struct Model {
    data: Branch,
}

fn get_data() -> impl Future<Item = Msg, Error = Msg> {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";

    Request::new(url)
        .method(Method::Get)
        .fetch_json()
        .map(Msg::Replace)
        .map_err(Msg::OnFetchErr)
}

fn send() -> impl Future<Item = Msg, Error = Msg> {
    let url = "https://infinitea.herokuapp.com/api/contact";

    let message = Message {
        name: "Mark Watney".into(),
        email: "mark@crypt.kk".into(),
        message: "I wanna be like Iron Man".into(),
    };

    Request::new(url)
        .method(Method::Post)
        .header("Content-Type", "application/json")
        .body_json(&message)
        .fetch_json()
        .map(Msg::OnServerResponse)
        .map_err(Msg::OnFetchErr)
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            data: Branch {
                name: "Loading...".into(),
                commit: Commit {
                    sha: "Loading...".into(),
                },
            },
        }
    }
}


#[derive(Clone)]
enum Msg {
    Replace(Branch),
    GetData,
    Send,
    OnServerResponse(ServerResponse),
    OnFetchErr(JsValue),
}

fn update(msg: Msg, model: &mut Model) -> impl Updater<Msg> {
    match msg {
        Msg::Replace(data) => {
            model.data = data;
            Render.into()
        }

        Msg::GetData => Update::with_future_msg(get_data()).skip(),

        Msg::Send => Update::with_future_msg(send()).skip(),

        Msg::OnServerResponse(result) => {
            log!(format!("Response: {:#?}", result));
            Skip.into()
        }

        Msg::OnFetchErr(err) => {
            log!(format!("Fetch error: {:#?}", err));
            Skip.into()
        }
    }
}


fn view(model: &Model) -> Vec<El<Msg>> {
    vec![
        div![format!(
            "Repo info: name: {}, sha: {}",
            model.data.name, model.data.commit.sha
        ),],
        button![
            raw_ev(Ev::Click, move |_| Msg::Send),
            "Send an urgent message"
        ],
    ]
}

#[wasm_bindgen]
pub fn render() {
    let state = seed::App::build(Model::default(), update, view)
        .mount("main")
        .finish()
        .run();

    state.update(Msg::GetData);
}
