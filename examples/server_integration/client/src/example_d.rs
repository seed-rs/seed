use futures::Future;
use seed::fetch;
use seed::prelude::*;
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

// Model

#[derive(Default)]
pub struct Model {
    pub response_result: Option<fetch::ResponseResult<()>>,
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
        Status::ReadyToSendRequest
    }
}

// Update

#[derive(Clone)]
pub enum Msg {
    SendRequest,
    DisableTimeout,
    Fetched(fetch::FetchObject<()>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            model.status = Status::WaitingForResponse(TimeoutStatus::Enabled);
            model.response_result = None;
            orders.perform_cmd(send_request(&mut model.request_controller));
        }

        Msg::DisableTimeout => {
            model
                .request_controller
                .take()
                .ok_or("Msg:DisableTimeout: request controller cannot be None")
                .and_then(|controller| controller.disable_timeout())
                .unwrap_or_else(|err| log!(err));
            model.status = Status::WaitingForResponse(TimeoutStatus::Disabled)
        }

        Msg::Fetched(fetch_object) => {
            model.status = Status::ReadyToSendRequest;
            model.response_result = Some(fetch_object.response());
        }
    }
}

fn send_request(
    request_controller: &mut Option<fetch::RequestController>,
) -> impl Future<Item = Msg, Error = Msg> {
    fetch::Request::new(get_request_url())
        .controller(|controller| *request_controller = Some(controller))
        .timeout(TIMEOUT)
        .fetch(Msg::Fetched)
}

// View

pub fn view(model: &Model) -> impl View<Msg> {
    match &model.response_result {
        None => vec![
            if let Status::WaitingForResponse(_) = model.status {
                div!["Waiting for response..."]
            } else {
                empty![]
            },
            view_button(&model.status),
        ],
        Some(Ok(response)) => vec![
            div![format!("Server returned {}.", response.status.text)],
            view_button(&model.status),
        ],
        Some(Err(fail_reason)) => view_fail_reason(fail_reason, &model.status),
    }
}

fn view_fail_reason(fail_reason: &fetch::FailReason<()>, status: &Status) -> Vec<Node<Msg>> {
    if let fetch::FailReason::RequestError(fetch::RequestError::DomException(dom_exception), _) =
        fail_reason
    {
        if dom_exception.name() == "AbortError" {
            return vec![div!["Request aborted."], view_button(status)];
        }
    }
    log!("Example_D error:", fail_reason);
    vec![]
}

pub fn view_button(status: &Status) -> Node<Msg> {
    match status {
        Status::WaitingForResponse(TimeoutStatus::Enabled) => {
            button![simple_ev(Ev::Click, Msg::DisableTimeout), "Disable timeout"]
        }
        Status::WaitingForResponse(TimeoutStatus::Disabled) => {
            button![attrs! {"disabled" => true}, "Timeout disabled"]
        }
        _ => button![simple_ev(Ev::Click, Msg::SendRequest), "Send request"],
    }
}
