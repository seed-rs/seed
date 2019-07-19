use futures::Future;
use seed::fetch;
use seed::prelude::*;
use serde::Serialize;
use std::mem;

pub const TITLE: &str = "Example E";
pub const DESCRIPTION: &str =
    "Write something and click button 'Submit`. See console.log for more info. \
     It sends form to the server and server returns 200 OK with 2 seconds delay.";

fn get_request_url() -> String {
    "/api/form".into()
}

// Model

#[derive(Serialize, Default)]
pub struct Form {
    text: String,
    checked: bool,
}

pub enum Model {
    ReadyToSubmit(Form),
    WaitingForResponse(Form),
}

impl Default for Model {
    fn default() -> Self {
        Model::ReadyToSubmit(Form::default())
    }
}

impl Model {
    fn form(&self) -> &Form {
        match self {
            Model::ReadyToSubmit(form) | Model::WaitingForResponse(form) => form,
        }
    }
    fn form_mut(&mut self) -> &mut Form {
        match self {
            Model::ReadyToSubmit(form) | Model::WaitingForResponse(form) => form,
        }
    }
}

// Update

#[derive(Clone)]
pub enum Msg {
    TextChanged(String),
    CheckedChanged,
    FormSubmitted(String),
    ServerResponded(fetch::ResponseResult<()>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::TextChanged(text) => model.form_mut().text = text,
        Msg::CheckedChanged => toggle(&mut model.form_mut().checked),
        Msg::FormSubmitted(id) => {
            let form = take(model.form_mut());
            orders.perform_cmd(send_request(&form));
            *model = Model::WaitingForResponse(form);
            log!("Form with id", id, "submitted.");
        }
        Msg::ServerResponded(Ok(_)) => {
            *model = Model::ReadyToSubmit(Form::default());
            log!("Form processed successfully.");
        }
        Msg::ServerResponded(Err(fail_reason)) => {
            *model = Model::ReadyToSubmit(take(model.form_mut()));
            error!("Request failed!", fail_reason);
        }
    }
}

fn send_request(form: &Form) -> impl Future<Item = Msg, Error = Msg> {
    fetch::Request::new(get_request_url())
        .method(fetch::Method::Post)
        .send_json(form)
        .fetch(|fetch_object| Msg::ServerResponded(fetch_object.response()))
}

fn take<T: Default>(source: &mut T) -> T {
    mem::replace(source, T::default())
}

fn toggle(value: &mut bool) {
    *value = !*value
}

// View

pub fn view(model: &Model) -> impl View<Msg> {
    let btn_disabled = match model {
        Model::ReadyToSubmit(form) if !form.text.is_empty() => false,
        _ => true,
    };

    let form_id = "A_FORM".to_string();
    form![
        raw_ev(Ev::Submit, move |event| {
            event.prevent_default();
            Msg::FormSubmitted(form_id)
        }),
        input![
            input_ev(Ev::Input, Msg::TextChanged),
            attrs! {At::Value => model.form().text}
        ],
        input![
            simple_ev(Ev::Click, Msg::CheckedChanged),
            attrs! {
                At::Type => "checkbox",
                At::Checked => model.form().checked.as_at_value(),
            }
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
