#[macro_use]
extern crate seed;
use seed::prelude::*;

use shared::Data;


// Model

#[derive(Clone)]
struct Model {
    pub data: Data,
}

impl Default for Model {
    fn default() -> Self {
        Self {
           data: Data {val: 0, text: "".into()}
        }
    }
}

fn get_data(state: seed::App<Msg, Model>) {
    let url = "http://localhost:8001/data";
    let callback = move |json: JsValue| {
        let data: Data = json.into_serde().unwrap();
        state.update(Msg::Replace(data));
    };

    seed::get(url, None, Box::new(callback));
}


// Update

#[derive(Clone)]
enum Msg {
    GetData(seed::App<Msg, Model>),
    Replace(Data),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::GetData(state) => {
            get_data(state);
            model
        },
        Msg::Replace(data) => Model {data}
    }
}


// View

fn view(state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    div![
        h1![ format!("Val: {} Text: {}", model.data.val, model.data.text) ],
        button![ 
            raw_ev("click", move |_| Msg::GetData(state.clone())), 
            "Update data"
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None, None);
}