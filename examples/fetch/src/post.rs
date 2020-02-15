//! Fetch POST example.

use seed::{fetch::*, prelude::*, *};

#[derive(serde::Serialize, Default)]
pub struct Form {
    name: String,
}

#[derive(Default)]
pub struct Model {
    form: Form,
}

pub enum Msg {
    NameChanged(String),
    Submit,
    Submited,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NameChanged(name) => model.form.name = name,
        Msg::Submit => {
            orders.skip(); // No need to rerender

            // Created outside async block for lifetime reasons.
            let request = Request::new("/")
                .method(Method::Post)
                .json(&model.form)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");
                assert!(response.status().is_ok());
                Msg::Submited
            });
        }
        Msg::Submited => {
            model.form.name = "".into();
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    form![
        ev(Ev::Submit, |_| Msg::Submit),
        label![
            "Name",
            input![
                attrs! {At::Value => model.form.name},
                input_ev(Ev::Input, Msg::NameChanged),
            ]
        ],
        button![
            "Submit",
            ev(Ev::Click, |event| {
                event.prevent_default();
                Msg::Submit
            })
        ]
    ]
}
