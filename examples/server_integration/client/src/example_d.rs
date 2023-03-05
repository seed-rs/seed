use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
use seed::{prelude::*, *};
use std::{cell::RefCell, rc::Rc};

pub const TITLE: &str = "Example D";
pub const DESCRIPTION: &str =
    "Click button 'Send request` to send request to endpoint with configurable delay.
    Click again to disable timeout - otherwise the request will time out.";

const TIMEOUT: u32 = 2000;

type FetchResult<T> = Result<T, gloo_net::Error>;

fn get_request_url() -> String {
    let response_delay_ms: u32 = 2500;
    format!("/api/delayed-response/{response_delay_ms}")
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    pub fetch_result: Option<FetchResult<String>>,
    pub timeout_handle: Rc<RefCell<Option<Timeout>>>,
    pub abort_controller: Rc<web_sys::AbortController>,
    pub status: Status,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            fetch_result: None,
            timeout_handle: Rc::new(RefCell::new(None)),
            abort_controller: Rc::new(web_sys::AbortController::new().unwrap()),
            status: Status::default(),
        }
    }
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
    Fetched(FetchResult<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            let abort_signal = model.abort_controller.signal();
            let request = Request::get(&get_request_url()).abort_signal(Some(&abort_signal));
            let abort_controller = Rc::clone(&model.abort_controller);
            model.timeout_handle.replace(Some(
                // abort request on timeout
                Timeout::new(TIMEOUT, move || abort_controller.abort()),
            ));

            model.status = Status::WaitingForResponse(TimeoutStatus::Enabled);
            model.fetch_result = None;

            orders.perform_cmd(async {
                Msg::Fetched(async { request.send().await?.text().await }.await)
            });
        }

        Msg::DisableTimeout => {
            // Cancel timeout by dropping it.
            if model.timeout_handle.replace(None).is_none() {
                // timeout already disabled
            }
            model.status = Status::WaitingForResponse(TimeoutStatus::Disabled);
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
            Some(Ok(result)) => Some(div![format!("Server returned: {result:#?}")]),
            Some(Err(fetch_error)) => Some(div![format!("{fetch_error:#?}")]),
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
