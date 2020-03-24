use seed::{prelude::*, *};
use std::borrow::Cow;

pub const TITLE: &str = "Example D";
pub const DESCRIPTION: &str =
    "Click button 'Send request` to send request to endpoint with configurable delay.
    Click again to disable timeout - otherwise the request will time out.";

const TIMEOUT: u32 = 2000;

fn get_request_url() -> impl Into<Cow<'static, str>> {
    let response_delay_ms: u32 = 2500;
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

pub enum TimeoutStatus {
    Enabled,
    Disabled,
}

pub enum Status {
    ReadyToSendRequest,
    WaitingForResponse(TimeoutStatus),
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
    DisableTimeout,
    Fetched(fetch::Result<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            let (request, controller) = Request::new(get_request_url())
                .timeout(TIMEOUT)
                .controller();

            model.status = Status::WaitingForResponse(TimeoutStatus::Enabled);
            model.fetch_result = None;
            model.request_controller = Some(controller);

            orders.perform_cmd(async {
                Msg::Fetched(async { fetch(request).await?.text().await }.await)
            });
        }

        Msg::DisableTimeout => {
            if let Some(controller) = &model.request_controller {
                controller.disable_timeout().expect("disable timeout");
            }
            model.status = Status::WaitingForResponse(TimeoutStatus::Disabled)
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
        match &model.fetch_result {
            None =>
                IF!(matches!(model.status, Status::WaitingForResponse(_)) => div!["Waiting for response..."]),
            Some(Ok(result)) => Some(div![format!("Server returned: {:#?}", result)]),
            Some(Err(fetch_error)) => Some(div![format!("{:#?}", fetch_error)]),
        },
        view_button(&model.status),
    ]
}

pub fn view_button(status: &Status) -> Node<Msg> {
    match status {
        Status::WaitingForResponse(TimeoutStatus::Enabled) => {
            button![ev(Ev::Click, |_| Msg::DisableTimeout), "Disable timeout"]
        }
        Status::WaitingForResponse(TimeoutStatus::Disabled) => {
            button![attrs! {"disabled" => true}, "Timeout disabled"]
        }
        _ => button![ev(Ev::Click, |_| Msg::SendRequest), "Send request"],
    }
}
