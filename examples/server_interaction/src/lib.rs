//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! https://serde.rs/

#[macro_use]
extern crate seed;
use seed::prelude::*;
use seed::{spawn_local, Method, Request};
use serde::{Deserialize, Serialize};

use futures::Future;

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

#[derive(Deserialize, Debug)]
struct ServerResponse {
    pub success: bool,
}

#[derive(Clone)]
struct Model {
    data: Branch,
}

fn get_data(state: seed::App<Msg, Model>) -> impl Future<Item = (), Error = JsValue> {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";

    Request::new(url)
        .method(Method::Get)
        .fetch_json()
        .map(move |json| {
            state.update(Msg::Replace(json));
        })
}

fn send() -> impl Future<Item = (), Error = JsValue> {
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
        .map(|result: ServerResponse| {
            log!(format!("Response: {:?}", result));
        })
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            data: Branch {
                name: "Test".into(),
                commit: Commit { sha: "123".into() },
            },
        }
    }
}

// Update

#[derive(Clone)]
enum Msg {
    Replace(Branch),
    GetData(seed::App<Msg, Model>),
    Send,
}

fn update(msg: Msg, model: Model) -> Update<Model> {
    match msg {
        Msg::Replace(data) => Render(Model { data }),
        // Msg::GetData is unused in this example, but could be used when
        // updating state from an event.
        Msg::GetData(state) => {
            spawn_local(get_data(state));
            Render(model)
        }
        Msg::Send => {
            spawn_local(send());
            Render(model)
        }
    }
}

// View

fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![
        div![
            format!(
                "Repo info: name: {}, sha: {}",
                model.data.name, model.data.commit.sha
            ),
            did_mount(move |_| spawn_local(get_data(state.clone())))
        ],
        button![
            raw_ev("click", move |_| Msg::Send),
            "Send an urgent message"
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .mount("main")
        .finish()
        .run();
}
