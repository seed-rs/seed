use futures::Future;
use seed::fetch;
use seed::prelude::*;
use std::mem;

pub const TITLE: &str = "Example E";
pub const DESCRIPTION: &str =
    "Write something and click button 'Submit`. It sends form to the server and server returns 200 OK with 2 seconds delay.";

fn get_request_url() -> String {
    "/api/form".into()
}

// Model

pub enum Model {
    ReadyToSubmit(String),
    WaitingForResponse(String),
}

impl Default for Model {
    fn default() -> Self {
        Model::ReadyToSubmit("".into())
    }
}

impl Model {
    fn text(&self) -> &str {
        match self {
            Model::ReadyToSubmit(text) | Model::WaitingForResponse(text) => text,
        }
    }
    fn text_mut(&mut self) -> &mut String {
        match self {
            Model::ReadyToSubmit(text) | Model::WaitingForResponse(text) => text,
        }
    }
    fn set_text(&mut self, text: String) {
        match self {
            Model::ReadyToSubmit(old_text) | Model::WaitingForResponse(old_text) => {
                *old_text = text
            }
        }
    }
}

// Update

#[derive(Clone)]
pub enum Msg {
    TextChanged(String),
    FormSubmitted,
    ServerResponded(fetch::ResponseResult<()>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::TextChanged(text) => model.set_text(text),

        Msg::FormSubmitted => {
            let text = take(model.text_mut());
            orders.perform_cmd(send_request(&text));
            *model = Model::WaitingForResponse(text);
        }

        Msg::ServerResponded(Ok(_)) => *model = Model::ReadyToSubmit("".into()),

        Msg::ServerResponded(Err(_)) => *model = Model::ReadyToSubmit(take(model.text_mut())),
    }
}

#[allow(clippy::ptr_arg)]
fn send_request(text: &String) -> impl Future<Item = Msg, Error = Msg> {
    fetch::Request::new(get_request_url())
        .method(fetch::Method::Post)
        .send_json(text)
        .fetch(|fetch_object| Msg::ServerResponded(fetch_object.response()))
}

fn take<T: Default>(source: &mut T) -> T {
    mem::replace(source, T::default())
}

// View

pub fn view(model: &Model) -> impl View<Msg> {
    let btn_disabled = match model {
        Model::ReadyToSubmit(text) if !text.is_empty() => false,
        _ => true,
    };

    form![
        raw_ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::FormSubmitted
        }),
        input![
            input_ev(Ev::Input, Msg::TextChanged),
            attrs! {At::Value => model.text()}
        ],
        button![
            style! {
                "padding" => format!{"{} {}", px(2), px(12)},
                "background-color" => if btn_disabled { CSSValue::Ignored } else { "aquamarine".into() },
            },
            attrs! {At::Disabled => btn_disabled.as_at_value()},
            "Submit"
        ]
    ]
}
