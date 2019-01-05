//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! https://serde.rs/

#[macro_use]
extern crate seed;
use seed::prelude::*;

use serde::{Serialize, Deserialize};
use serde_json;


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
//        data: Data,
    data: Branch,
}

// todo
use wasm_bindgen_futures;
use futures::{future, Future};


fn get_data(state: seed::App<Msg, Model>) {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";
//    let url = "https://seed-example.herokuapp.com/data";
    let callback = move |json: JsValue| {
        let data: Branch = json.into_serde().unwrap();
        state.update(Msg::Replace(data));
    };

    seed::get_json(url, None, Box::new(callback));
}

fn send() {
//    let url = "http://127.0.0.1:8001/api/contact";
    let url = "https://infinitea.herokuapp.com/api/contact";

    let message = Message {
        name: "Mark Watney".into(),
        email: "mark@crypt.kk".into(),
        message: "I wanna be like Iron Man".into(),
    };

    let mut opts = seed::RequestOpts::new();
    opts.headers.insert("Content-Type".into(), "application/json".into());

    let callback = |json: JsValue| {
        let result: ServerResponse = json.into_serde().unwrap();
        log!(format!("Response: {:?}", result));
    };

    seed::post_json(url, message, Some(opts), Box::new(callback));
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            data: Branch{ name: "Test".into(), commit: Commit{sha: "123".into()} }
//            data: Data {val: 0, text: "".into()}
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
//        Replace(Data),
    Replace(Branch),
    GetData(seed::App<Msg, Model>),
    Send,
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Replace(data) => Model {data},
        Msg::GetData(app) => {
            get_data(app);
            model
        },
        Msg::Send => {
            send();
            model
        }
    }
}


// View

fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    let state2 = state.clone();
    div![
        div![ format!("Repo info: name: {}, sha: {}", model.data.name, model.data.commit.sha),
            did_mount(move |_| get_data(state.clone()))
        ],

        button![ raw_ev("click", move |_| Msg::Send), "Send an urgent message"]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None, None);

}