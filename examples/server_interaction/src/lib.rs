//! [https://rustwasm.github.io/wasm-bindgen/examples/fetch.html](https://rustwasm.github.io/wasm-bindgen/examples/fetch.html)
//! [https://serde.rs/](https://serde.rs/)

#![allow(clippy::large_enum_variant)]

#[macro_use]
extern crate seed;

use futures::Future;
use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Deserialize, Serialize};

const REPOSITORY_URL: &str = "https://api.github.com/repos/david-oconnor/seed/branches/master";
const CONTACT_URL: &str = "https://infinitea.herokuapp.com/api/contact";

#[derive(Serialize)]
struct SendMessageRequestBody {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct SendMessageResponseBody {
    pub success: bool,
}

// Model

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Commit {
    pub sha: String,
}

struct Model {
    branch: Branch,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            branch: Branch {
                name: "Loading...".into(),
                commit: Commit {
                    sha: "Loading...".into(),
                },
            },
        }
    }
}

#[derive(Clone)]
enum Msg {
    FetchRepositoryInfo,
    RepositoryInfoFetched(fetch::ResponseDataResult<Branch>),
    SendMessage,
    MessageSent(fetch::ResponseDataResult<SendMessageResponseBody>),
    OnFetchError {
        label: &'static str,
        fail_reason: fetch::FailReason,
    },
}

fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::FetchRepositoryInfo => {
            orders.skip().perform_cmd(fetch_repository_info());
        }

        Msg::RepositoryInfoFetched(Ok(branch)) => model.branch = branch,

        Msg::RepositoryInfoFetched(Err(fail_reason)) => {
            orders
                .send_msg(Msg::OnFetchError {
                    label: "Fetching repository info failed",
                    fail_reason,
                })
                .skip();
        }

        Msg::SendMessage => {
            orders.skip().perform_cmd(send_message());
        }

        Msg::MessageSent(Ok(response_data)) => {
            log!(format!("Response data: {:#?}", response_data));
            orders.skip();
        }

        Msg::MessageSent(Err(fail_reason)) => {
            orders
                .send_msg(Msg::OnFetchError {
                    label: "Sending message failed",
                    fail_reason,
                })
                .skip();
        }

        Msg::OnFetchError { label, fail_reason } => {
            error!(format!("Fetch error - {} - {:#?}", label, fail_reason));
            orders.skip();
        }
    }
}

fn fetch_repository_info() -> impl Future<Item = Msg, Error = Msg> {
    Request::new(REPOSITORY_URL.into()).fetch_json_data(Msg::RepositoryInfoFetched)
}

fn send_message() -> impl Future<Item = Msg, Error = Msg> {
    let message = SendMessageRequestBody {
        name: "Mark Watney".into(),
        email: "mark@crypt.kk".into(),
        message: "I wanna be like Iron Man".into(),
    };

    Request::new(CONTACT_URL.into())
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(Msg::MessageSent)
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        div![format!(
            "Repo info: Name: {}, SHA: {}",
            model.branch.name, model.branch.commit.sha
        )],
        button![
            simple_ev(Ev::Click, Msg::SendMessage),
            "Send an urgent message (see console log)"
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    let app = seed::App::build(Model::default(), update, view)
        .finish()
        .run();

    app.update(Msg::FetchRepositoryInfo);
}
