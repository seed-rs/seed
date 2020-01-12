use seed::browser::service::fetch;
use seed::{prelude::*, *};
use std::borrow::Cow;

pub const TITLE: &str = "Example C";
pub const DESCRIPTION: &str =
    "Click button 'Send request` to send request to endpoint with configurable delay.
    Click again to abort request.";

fn get_request_url() -> impl Into<Cow<'static, str>> {
    let response_delay_ms: u32 = 2000;
    format!("/api/delayed-response/{}", response_delay_ms)
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    pub response_data_result: Option<fetch::ResponseDataResult<String>>,
    pub request_controller: Option<fetch::RequestController>,
    pub status: Status,
}

pub enum Status {
    ReadyToSendRequest,
    WaitingForResponse,
    RequestAborted,
}

impl Default for Status {
    fn default() -> Self {
        Self::ReadyToSendRequest
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    SendRequest,
    AbortRequest,
    Fetched(fetch::ResponseDataResult<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            model.status = Status::WaitingForResponse;
            model.response_data_result = None;
            let request = fetch::Request::new(get_request_url())
                .controller(|controller| model.request_controller = Some(controller));
            orders.perform_cmd(request.fetch_string_data(Msg::Fetched));
        }

        Msg::AbortRequest => {
            model
                .request_controller
                .take()
                .expect("AbortRequest: request_controller hasn't been set!")
                .abort();
            model.status = Status::RequestAborted;
        }

        Msg::Fetched(response_data_result) => {
            model.status = Status::ReadyToSendRequest;
            model.response_data_result = Some(response_data_result);
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, intro: impl FnOnce(&str, &str) -> Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    nodes![
        intro(TITLE, DESCRIPTION),
        match model.status {
            Status::ReadyToSendRequest => vec![
                view_response_data_result(&model.response_data_result),
                button![ev(Ev::Click, |_| Msg::SendRequest), "Send request"],
            ],
            Status::WaitingForResponse => vec![
                div!["Waiting for response..."],
                button![ev(Ev::Click, |_| Msg::AbortRequest), "Abort request"],
            ],
            Status::RequestAborted => vec![
                view_response_data_result(&model.response_data_result),
                button![
                    attrs! {At::Disabled => false.as_at_value()},
                    "Request aborted"
                ],
            ],
        }
    ]
}

fn view_response_data_result(
    response_data_result: &Option<fetch::ResponseDataResult<String>>,
) -> Node<Msg> {
    match &response_data_result {
        None => empty![],
        Some(Ok(response_data)) => div![format!(r#"Response String body: "{}""#, response_data)],
        Some(Err(fail_reason)) => view_fail_reason(fail_reason),
    }
}

fn view_fail_reason(fail_reason: &fetch::FailReason<String>) -> Node<Msg> {
    if let fetch::FailReason::RequestError(fetch::RequestError::DomException(dom_exception), _) =
        fail_reason
    {
        if dom_exception.name() == "AbortError" {
            return div![
                div![format!(r#"Error name: "{}""#, dom_exception.name())],
                div![format!(r#"Error message: "{}""#, dom_exception.message())]
            ];
        }
    }
    log!("Example_C error:", fail_reason);
    empty![]
}
