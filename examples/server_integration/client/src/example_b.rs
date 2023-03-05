use gloo_net::http::Request;
use seed::{prelude::*, *};
use serde::Deserialize;

pub const TITLE: &str = "Example B";
pub const DESCRIPTION: &str =
    "Click button 'Try to Fetch JSON' to send request to non-existent endpoint.
    Server will return status 404 with empty body. `Response::check_status` then return error.";

type FetchResult<T> = Result<T, gloo_net::Error>;

const fn get_request_url() -> &'static str {
    "/api/non-existent-endpoint"
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    pub fetch_result: Option<FetchResult<ExpectedResponseData>>,
}

#[derive(Debug, Deserialize)]
pub struct ExpectedResponseData {
    #[allow(dead_code)]
    something: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    SendRequest,
    Fetched(FetchResult<ExpectedResponseData>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            orders.skip().perform_cmd(async {
                Msg::Fetched(
                    async { Request::get(get_request_url()).send().await?.json().await }.await,
                )
            });
        }

        Msg::Fetched(fetch_result) => {
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
        model
            .fetch_result
            .as_ref()
            .map(|result| div![format!("{result:#?}")]),
        button![ev(Ev::Click, |_| Msg::SendRequest), "Try to Fetch JSON"],
    ]
}
