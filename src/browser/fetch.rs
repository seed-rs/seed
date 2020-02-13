#![allow(unused)]
//! Fetch.
use super::Url;

use std::future::Future;

use gloo_timers::callback::Timeout;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{borrow::Cow, cell::RefCell, collections::HashMap, convert::identity, rc::Rc};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys;

pub struct Request {}

pub struct Response {
    raw_response: web_sys::Response,
}

impl Response {
    pub async fn json<T: DeserializeOwned + 'static>(self) -> Result<T, FetchError> {
        let text = self.raw_response
            .text()
            .map(JsFuture::from)
            .unwrap() // promise
            .await
            .unwrap() // json.parse error
            .as_string()
            .unwrap(); // as_string

        serde_json::from_str(&text).map_err(FetchError::SerdeError)
    }

    pub fn status(&self) -> Status {
        Status::from(&self.raw_response)
    }
}

#[derive(Debug)]
pub enum FetchError {
    SerdeError(serde_json::Error),
    DomException(web_sys::DomException),
}

pub async fn fetch(url: &str) -> Result<Response, FetchError> {
    let mut request = web_sys::RequestInit::new();
    let fetch_promise = web_sys::window()
        .expect("fetch: cannot find window")
        .fetch_with_str_and_init(url, &request);

    let raw_response = JsFuture::from(fetch_promise)
        .await
        .map(Into::into)
        .map_err(|js_value_error| FetchError::DomException(js_value_error.into()))?;

    Ok(Response{raw_response})
}

#[derive(Debug, Clone)]
/// Response status.
///
/// It's intended to create `Status` from `web_sys::Response` - eg: `Status::from(&raw_response)`.
pub struct Status {
    /// Code examples: 200, 404, ...
    pub code: u16,
    /// Text examples: "OK", "Not Found", ...
    pub text: String,
    pub category: StatusCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCategory {
    /// Code 1xx
    Informational,
    /// Code 2xx
    Success,
    /// Code 3xx
    Redirection,
    /// Code 4xx
    ClientError,
    /// Code 5xx
    ServerError,
    /// Code < 100 || Code >= 600
    Unknown,
}


#[allow(dead_code)]
impl Status {
    /// Is response status category `ClientError` or `ServerError`? (Code 400-599)
    pub fn is_error(&self) -> bool {
        match self.category {
            StatusCategory::ClientError | StatusCategory::ServerError => true,
            _ => false,
        }
    }
    /// Is response status category `Success`? (Code 200-299)
    pub fn is_ok(&self) -> bool {
        self.category == StatusCategory::Success
    }
}

impl From<&web_sys::Response> for Status {
    fn from(response: &web_sys::Response) -> Self {
        let text = response.status_text();
        match response.status() {
            code @ 100..=199 => Status {
                code,
                text,
                category: StatusCategory::Informational,
            },
            code @ 200..=299 => Status {
                code,
                text,
                category: StatusCategory::Success,
            },
            code @ 300..=399 => Status {
                code,
                text,
                category: StatusCategory::Redirection,
            },
            code @ 400..=499 => Status {
                code,
                text,
                category: StatusCategory::ClientError,
            },
            code @ 500..=599 => Status {
                code,
                text,
                category: StatusCategory::ServerError,
            },
            code => Status {
                code,
                text,
                category: StatusCategory::Unknown,
            },
        }
    }
}
