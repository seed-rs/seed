use seed::prelude::*;
use seed::fetch;
use futures::Future;
use serde::Deserialize;

pub const TITLE: &str = "Example B";
pub const DESCRIPTION: &str =
    "Click button 'Try to Fetch JSON' to send request to non-existent endpoint.
    Server will return 404 with empty body and Serde then fail to decode body into predefined JSON.";

fn get_request_url() -> String {
    "/api/non-existent-endpoint".into()
}

// Model

#[derive(Default)]
pub struct Model {
    pub fetch_result: Option<fetch::FetchResult<ExpectedResponseData>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExpectedResponseData {
    something: String
}

// Update

#[derive(Clone)]
pub enum Msg {
    SendRequest,
    Fetched(fetch::FetchObject<ExpectedResponseData>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            orders
                .skip()
                .perform_cmd(send_request());
        }

        Msg::Fetched(fetch_object) => {
            model.fetch_result = Some(fetch_object.result);
        }
    }
}

fn send_request() -> impl Future<Item=Msg, Error=Msg> {
    fetch::Request::new(get_request_url())
        .fetch_json(Msg::Fetched)
}

// View

pub fn view(model: &Model) -> impl ElContainer<Msg> {
    vec![
        match &model.fetch_result {
            None => empty![],
            Some(result) => {
                match result {
                    Err(request_error) => {
                        log!("Example_B error:", request_error);
                        empty![]
                    }
                    Ok(response_with_data_result) => {
                        div![
                            div![format!("Status code: {}", response_with_data_result.status.code)],
                            div![format!(r#"Status text: "{}""#, response_with_data_result.status.text)],
                            div![format!(r#"Data: "{:#?}""#, response_with_data_result.data)]
                        ]
                    }
                }
            }
        },
        button![
            simple_ev(Ev::Click, Msg::SendRequest),
            "Try to Fetch JSON"
        ],
    ]
}