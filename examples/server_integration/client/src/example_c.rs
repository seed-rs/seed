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
    pub fetch_result: Option<fetch::Result<String>>,
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
    Fetched(fetch::Result<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            let (request, controller) = Request::new(get_request_url()).controller();
            model.status = Status::WaitingForResponse;
            model.fetch_result = None;
            model.request_controller = Some(controller);
            orders.perform_cmd(async {
                Msg::Fetched(async { fetch(request).await?.text().await }.await)
            });
        }

        Msg::AbortRequest => {
            if let Some(controller) = &model.request_controller {
                controller.abort();
            }
            model.status = Status::RequestAborted;
        }

        Msg::Fetched(fetch_result) => {
            model.status = Status::ReadyToSendRequest;
            model.fetch_result = Some(fetch_result);
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
            Status::ReadyToSendRequest => nodes![
                model
                    .fetch_result
                    .as_ref()
                    .map(|result| div![format!("{:#?}", result)]),
                button![ev(Ev::Click, |_| Msg::SendRequest), "Send request"],
            ],
            Status::WaitingForResponse => nodes![
                div!["Waiting for response..."],
                button![ev(Ev::Click, |_| Msg::AbortRequest), "Abort request"],
            ],
            Status::RequestAborted => nodes![
                model
                    .fetch_result
                    .as_ref()
                    .map(|result| div![format!("{:#?}", result)]),
                button![
                    attrs! {At::Disabled => false.as_at_value()},
                    "Request aborted"
                ],
            ],
        }
    ]
}
