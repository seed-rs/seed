//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! https://serde.rs/

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;

use serde_json;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;




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


use futures::{future, Future};
use wasm_bindgen_futures;
use wasm_bindgen_futures::future_to_promise;
// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        let url = "https://api.github.com/repos/rust-lang/rust/branches/master";

        let placeholder = Self {
            data: Branch{ name: "Test".into(), commit: Commit{sha: "123".into()} }
        };

        let cb = Box::new(|serialized: String| {
            let data: Branch = serde_json::from_str(&serialized).unwrap();
            log!(format!("{:?}", &data));
//            let p2 = Self {
//            data: Data { val: 0, text: String::new() }
//                data: Branch{ name: "Test".into(), commit: Commit{sha: "123".into()} }
//            data: data
//            };

//            update(Msg::Replace(data), p2);
        });
        seed::fetch::get(url, None, Some(headers), cb);



//            .and_then(|json| {
////            // Use serde to parse the JSON into a struct.
//            let data: Branch = json.into_serde().unwrap();
//            log!(format!("{:?}", data));
////
//////            log!(format!("{:?}", data));
////            future::ok("test")
//            future::ok(JsValue::from_serde(&data).unwrap())
////
//        });
//        let p = future_to_promise(r);

//        Msg::Replace(data);


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
        Msg::Replace(data) => Model {data},
    }
}


// View

fn view(model: Model) -> El<Msg> {
//    div![ format!("Hello World. Val: {}, text: {}", model.data.val, model.data.text) ]
    div![ format!("Hello World. name: {}, sha: {}", model.data.name, model.data.commit.sha) ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);
}