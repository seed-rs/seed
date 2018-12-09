#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


// Model


// Note that you can apply Serialize and Deserialize to your model directly,
// eg if you'd like to receive or pass all of its data.
#[derive(Serialize, Deserialize)]
struct Data {

}

#[derive(Clone)]
struct Model {
    pub val: i32,
    data: Data,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
            data: Data {

            }
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: &Msg, model: &Model) -> Model {
    match msg {
        Msg::Increment => {
            Model {val: model.val + 1}
        },
    }
}


// View

fn view(model: &Model) -> El<Msg> {
    div![ "Hello World" ]
}

#[wasm_bindgen]
pub fn render() {
    rebar::run(Model::default(), update, view, "main");
}