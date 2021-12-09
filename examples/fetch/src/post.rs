//! Fetch POST example.

use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    form: Form,
    message: Option<String>,
}

#[derive(serde::Serialize, Default)]
pub struct Form {
    name: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    NameChanged(String),
    Submit,
    Submited,
    SubmitFailed(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NameChanged(name) => model.form.name = name,
        Msg::Submit => {
            orders.skip(); // No need to rerender

            let token = "YWxhZGRpbjpvcGVuc2VzYW1l";
            // Created outside async block because of lifetime reasons
            // (we can't use reference to `model.form` in async function).
            let request = Request::new("/")
                .method(Method::Post)
                .header(Header::custom("Accept-Language", "en"))
                .header(Header::bearer(token))
                .json(&model.form)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");

                if response.status().is_ok() {
                    Msg::Submited
                } else {
                    Msg::SubmitFailed(response.status().text)
                }
            });
        }
        Msg::Submited => {
            model.form.name = "".into();
            model.message = Some("Submit succeeded".into());
        }
        Msg::SubmitFailed(reason) => {
            model.message = Some(reason);
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    form![
        ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::Submit
        }),
        label![
            "Name",
            input![
                attrs! {At::Value => model.form.name},
                input_ev(Ev::Input, Msg::NameChanged),
            ]
        ],
        button!["Submit"],
        model
            .message
            .as_ref()
            .map_or(empty![], |message| span![message]),
    ]
}
