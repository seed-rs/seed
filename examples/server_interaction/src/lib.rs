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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: u32,
    pub name: String,
    pub category: u32,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngList {
    pub count: u32,
    pub results: Vec<Ingredient>,

}

#[derive(Clone)]
struct Model {
//        data: Data,
    data: Branch,
}


fn temp_fetch(app: seed::App<Msg, Model>) {
    let url = "https://api.github.com/repos/david-oconnor/seed/branches/master";
//    let url = "https://infinitea.herokuapp.com/api/ingredients";
//    let url = "http://127.0.0.1:8001/api/ingredients";
//    let url = "http://127.0.0.1:8001/api/contact";
//    let url = "localhost:8001/api/contact";
//    let url = "https://seed-example.herokuapp.com/data";

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

    let callback = move |json: JsValue| {
        // Use serde to parse the JSON into a struct.
        let data: Branch = json.into_serde().unwrap();
//        let data2: IngList = json.into_serde().unwrap();
        log!(format!("{:?}", data));
        app.update_dom(Msg::Replace(data));
    };

    seed::get(url, None, Box::new(callback));
//    seed::post(url, Some(opts), Box::new(callback));
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {

//        temp_fetch();

        let placeholder = Self {
            data: Branch{ name: "Test".into(), commit: Commit{sha: "123".into()} }
//            data: Data {val: 0, text: "".into()}
        };

        placeholder
    }
}


// Update

#[derive(Clone)]
enum Msg {
//        Replace(Data),
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

fn view(app: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![ format!("Hello World. name: {}, sha: {}", model.data.name, model.data.commit.sha),
//    div![ format!("Hello World. name: {}, sha: {}", model.data.val, model.data.text),
//        simple_ev("click", Msg::Replace(new_data))
        did_mount(move |_| {
        temp_fetch(app.clone());
//            Msg::Replace()

        }),

     ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);

}