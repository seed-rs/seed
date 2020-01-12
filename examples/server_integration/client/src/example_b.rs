use seed::browser::service::fetch;
use seed::{prelude::*, *};
use serde::Deserialize;
use std::borrow::Cow;

pub const TITLE: &str = "Example B";
pub const DESCRIPTION: &str =
    "Click button 'Try to Fetch JSON' to send request to non-existent endpoint.
    Server will return 404 with empty body and Serde then fail to decode body into predefined JSON.";

fn get_request_url() -> impl Into<Cow<'static, str>> {
    "/api/non-existent-endpoint"
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    pub response_with_data_result: Option<fetch::ResponseWithDataResult<ExpectedResponseData>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExpectedResponseData {
    something: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    SendRequest,
    Fetched(fetch::FetchResult<ExpectedResponseData>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            orders.skip().perform_cmd(send_request());
        }

        Msg::Fetched(Ok(response_with_data_result)) => {
            model.response_with_data_result = Some(response_with_data_result);
        }

        Msg::Fetched(Err(request_error)) => {
            log!("Example_B error:", request_error);
            orders.skip();
        }
    }
}

async fn send_request() -> Result<Msg, Msg> {
    fetch::Request::new(get_request_url())
        .fetch_json(|fetch_object| Msg::Fetched(fetch_object.result))
        .await
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, intro: impl FnOnce(&str, &str) -> Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    nodes![
        intro(TITLE, DESCRIPTION),
        match &model.response_with_data_result {
            None => empty![],
            Some(fetch::ResponseWithDataResult { status, data, .. }) => div![
                div![format!("Status code: {}", status.code)],
                div![format!(r#"Status text: "{}""#, status.text)],
                div![format!(r#"Data: "{:#?}""#, data)]
            ],
        },
        button![ev(Ev::Click, |_| Msg::SendRequest), "Try to Fetch JSON"],
    ]
}
