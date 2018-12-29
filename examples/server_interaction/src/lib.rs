//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! https://serde.rs/

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;

use serde_json;
use serde::{Serialize, Deserialize};

// Model

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
    //    data: Data,
    data: Branch,
}


fn temp_fetch() {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";

    let callback = |json: JsValue| {
            // Use serde to parse the JSON into a struct.
            let branch_info: Branch = json.into_serde().unwrap();

            let model = Model {
                data: branch_info
            };

            seed::run(model, update, view, "main", None);
        };

    seed::get(url, None, None, Box::new(callback));
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {

        temp_fetch();

        let placeholder = Self {
            data: Branch{ name: "Test".into(), commit: Commit{sha: "123".into()} }
        };

        placeholder
    }
}


// Update

#[derive(Clone)]
enum Msg {
    //    Replace(Data),
    Replace(Branch),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Replace(data) => {
            log!(format!("{:?}", &data));
            Model {data}
        },
    }
}


// View

fn view(model: Model) -> El<Msg> {
    div![ format!("Hello World. name: {}, sha: {}", model.data.name, model.data.commit.sha),
//        simple_ev("click", Msg::Replace(new_data))
        did_mount(|_| {
//            Msg::Replace()
        }),

     ]
}

#[wasm_bindgen]
pub fn render() {

    temp_fetch();
//    seed::run(Model::default(), update, view, "main", None);
}