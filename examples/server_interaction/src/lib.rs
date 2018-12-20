//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! https://serde.rs/

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;


// Model


// Note that you can apply Serialize and Deserialize to your model directly,
// eg if you'd like to receive or pass all of its data.
// Why is clone required?b
#[derive(Serialize, Deserialize, Clone)]
struct Data {
    val: i32,
    text: String,
}

#[derive(Clone)]
struct Model {
    data: Data,
}


// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
//        let url = "https://seed-example.herokuapp.com/data";
        let url = "https://api.github.com/repos/rust-lang/rust/branches/master";
        let mut headers = HashMap::new();
//        headers.insert("Content-Type", "application/json");

        let data = seed::fetch::fetch(seed::fetch::Method::Get, url, None, Some(headers));


//        let closure = Closure::wrap(
//            Box::new(move |v| {
//                seed::log(v);
//            })
//        );



//        server_data.then(&closure);;

//        Msg::Replace(data);

//        seed::log(server_data.into());

        Self {
            data: Data { val: 0, text: String::new() }
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Replace(Data),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Replace(data) => Model {data},
    }
}


// View

fn view(model: Model) -> El<Msg> {
    div![ format!("Hello World. Val: {}, text: {}", model.data.val, model.data.text) ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);
}