//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! https://serde.rs/

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;

use serde::{Serialize, Deserialize};

// Model

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub val: u32,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Clone)]
struct Model {
//        data: Data,
    data: Branch,
}


fn test_get(app: seed::App<Msg, Model>) {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";
//    let url = "https://seed-example.herokuapp.com/data";

    let callback = move |json: JsValue| {
        let data: Branch = json.into_serde().unwrap();
        app.update_dom(Msg::Replace(data));
    };
    seed::get(url, None, Box::new(callback));
}

fn test_post() {
//    let url = "http://127.0.0.1:8001/api/contact";
    let url = "https://infinitea.herokuapp.com/api/contact";

    let opts = seed::RequestOpts {
        payload: Some(
            hashmap_string!{
                "name" => "david",
                "email" => "it@worked.gov",
                "message" => "Great site!",
            }
        ),
        headers: Some(  // todo try without headers
            hashmap_string!{
                "Content-Type" => "application/json",
            }
        ),
        credentials: None,
    };

    let callback = move |json: JsValue| {};

    seed::post(url, Some(opts), Box::new(callback));
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
    GetData(seed::App<Msg, Model>)
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Replace(data) => {
            log!(format!("{:?}", &data));
            Model {data}
        },
        Msg::GetData(app) => {
            test_get(app);
            model
        }
    }
}


// View

fn view(app: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![
        div![ format!("Hello World. name: {}, sha: {}", model.data.name, model.data.commit.sha) ],
        button![ raw_ev("click", move |_| Msg::GetData(app.clone())), "Update state from the internet"]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);

}