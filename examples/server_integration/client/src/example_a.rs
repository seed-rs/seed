use seed::prelude::*;
use seed::fetch;
use futures::Future;

use shared;

pub const TITLE: &str = "Example A";
pub const DESCRIPTION: &str =
    "Write something into input and click on 'Send message'.
    Message will be send to server and then it wil be returned with ordinal number.
    Ordinal number is incremented by server with each request.";

fn get_request_url() -> String {
    "/api/send-message".into()
}

// Model

#[derive(Default)]
pub struct Model {
    pub new_message: String,
    pub response_result: Option<fetch::ResponseResult<shared::SendMessageResponseBody>>,
}

// Update

#[derive(Clone)]
pub enum Msg {
    NewMessageChanged(String),
    SendRequest,
    Fetched(fetch::FetchObject<shared::SendMessageResponseBody>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::NewMessageChanged(message) => {
            model.new_message = message;
        }
        Msg::SendRequest => {
            orders
                .skip()
                .perform_cmd(send_request(model.new_message.clone()));
        }

        Msg::Fetched(fetch_object) => {
            model.response_result = Some(fetch_object.response());
        }
    }
}

fn send_request(new_message: String) -> impl Future<Item=Msg, Error=Msg> {
    fetch::Request::new(get_request_url())
        .method(fetch::Method::Post)
        .send_json(&shared::SendMessageRequestBody { text: new_message })
        .fetch_json(Msg::Fetched)
}

// View

pub fn view(model: &Model) -> impl ElContainer<Msg> {
    let message = match &model.response_result {
        None => empty![],
        Some(response_result) => {
            match response_result {
                Err(fail_reason) => {
                    log!("Example_A error:", fail_reason);
                    empty![]
                }
                Ok(response) => {
                    div![
                        format!(r#"{}. message: "{}""#, response.data.ordinal_number, response.data.text)
                    ]
                }
            }
        }
    };

    vec![
        message,
        input![
            input_ev(Ev::Input, Msg::NewMessageChanged),
            attrs!{At::Value => model.new_message}
        ],
        button![
            simple_ev(Ev::Click, Msg::SendRequest),
            "Send message"
        ],
    ]
}