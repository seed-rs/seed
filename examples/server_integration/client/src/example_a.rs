use seed::{self, prelude::*, *};
use std::borrow::Cow;

pub const TITLE: &str = "Example A";
pub const DESCRIPTION: &str = "Write something into input and click on 'Send message'.
    Message will be send to server and then it wil be returned with ordinal number.
    Ordinal number is incremented by server with each request.";

fn get_request_url() -> impl Into<Cow<'static, str>> {
    "/api/send-message"
}

// ------ ------
//     Model
// ------ -----

#[derive(Default)]
pub struct Model {
    pub new_message: String,
    pub response_data: Option<shared::SendMessageResponseBody>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    NewMessageChanged(String),
    SendRequest,
    Fetched(fetch::Result<shared::SendMessageResponseBody>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NewMessageChanged(message) => {
            model.new_message = message;
        }
        Msg::SendRequest => {
            orders.skip().perform_cmd({
                let message = model.new_message.clone();
                async { Msg::Fetched(send_message(message).await) }
            });
        }

        Msg::Fetched(Ok(response_data)) => {
            model.response_data = Some(response_data);
        }

        Msg::Fetched(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }
    }
}

async fn send_message(new_message: String) -> fetch::Result<shared::SendMessageResponseBody> {
    Request::new(get_request_url())
        .method(Method::Post)
        .json(&shared::SendMessageRequestBody { text: new_message })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, intro: impl FnOnce(&str, &str) -> Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    nodes![
        intro(TITLE, DESCRIPTION),
        view_message(&model.response_data),
        input![
            input_ev(Ev::Input, Msg::NewMessageChanged),
            attrs! {
                At::Value => model.new_message,
                At::AutoFocus => AtValue::None,
            }
        ],
        button![ev(Ev::Click, |_| Msg::SendRequest), "Send message"],
    ]
}

fn view_message(message: &Option<shared::SendMessageResponseBody>) -> Node<Msg> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };
    div![format!(
        r#"{}. message: "{}""#,
        message.ordinal_number, message.text
    )]
}
