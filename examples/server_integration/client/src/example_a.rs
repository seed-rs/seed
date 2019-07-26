use futures::Future;
use seed::fetch;
use seed::prelude::*;
use std::borrow::Cow;

use shared;

pub const TITLE: &str = "Example A";
pub const DESCRIPTION: &str = "Write something into input and click on 'Send message'.
    Message will be send to server and then it wil be returned with ordinal number.
    Ordinal number is incremented by server with each request.";

fn get_request_url() -> impl Into<Cow<'static, str>> {
    "/api/send-message"
}

// Model

#[derive(Default)]
pub struct Model {
    pub new_message: String,
    pub response_data: Option<shared::SendMessageResponseBody>,
}

// Update

#[derive(Clone)]
pub enum Msg {
    NewMessageChanged(String),
    SendRequest,
    Fetched(fetch::ResponseDataResult<shared::SendMessageResponseBody>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NewMessageChanged(message) => {
            model.new_message = message;
        }
        Msg::SendRequest => {
            orders
                .skip()
                .perform_cmd(send_request(model.new_message.clone()));
        }

        Msg::Fetched(Ok(response_data)) => {
            model.response_data = Some(response_data);
        }

        Msg::Fetched(Err(fail_reason)) => {
            log!("Example_A error:", fail_reason);
            orders.skip();
        }
    }
}

fn send_request(new_message: String) -> impl Future<Item = Msg, Error = Msg> {
    fetch::Request::new(get_request_url())
        .method(fetch::Method::Post)
        .send_json(&shared::SendMessageRequestBody { text: new_message })
        .fetch_json_data(Msg::Fetched)
}

// View

pub fn view(model: &Model) -> impl View<Msg> {
    let message = match &model.response_data {
        None => empty![],
        Some(shared::SendMessageResponseBody {
            ordinal_number,
            text,
        }) => div![format!(r#"{}. message: "{}""#, ordinal_number, text)],
    };

    vec![
        message,
        input![
            input_ev(Ev::Input, Msg::NewMessageChanged),
            attrs! {
                At::Value => model.new_message,
                At::AutoFocus => AtValue::None,
            }
        ],
        button![simple_ev(Ev::Click, Msg::SendRequest), "Send message"],
    ]
}
