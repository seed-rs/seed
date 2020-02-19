//! Fetch API.
//!
//! Our version of the Fetch API is based mostly on regular web one.
//!
//! There is one entry point: [`fetch`][fetch] function.
//! It can accept both, String urls as well as [`Request`][request].
//!
//! To get a [`Response`][response] you need to `.await` fetch:
//! ```rust
//! let response = fetch("/foo").await?;
//! ```
//!
//! Then you can check [`Status`][status] and extract body in various formats:
//! ```rust
//! let response = fetch("/foo").await?;
//! assert!(response.status().is_ok());
//! let body: FooStruct = response.json().await?;
//! ```
//!
//! As Rust doesn't have optional arguments we have no `fetch(url, init)` version,
//! instead you should use something like this:
//! ```rust
//! fetch(Request::new(url).method(Method::Post)).await
//! ```
//!
//!
//! [fetch]: ./fn.fetch.html
//! [request]: ./struct.Request.html
//! [response]: ./struct.Response.html
//! [status]: ./struct.Status.html

use crate::browser::Url;
use crate::util::window;
use serde_json;
use std::borrow::Cow;
use wasm_bindgen_futures::JsFuture;
use web_sys;

mod method;
mod request;
mod response;
mod status;

pub use method::*;
pub use request::*;
pub use response::*;
pub use status::*;

/// Convenient type alias.
pub type Result<T> = std::result::Result<T, FetchError>;

/// The fetch functions is a main entry point of the Fetch API.
///
/// It start the process of fetching a resource from the network,
/// returning a future which is fulfilled once the response is
/// available. The future resolves to the Response object representing
/// the response to your request. The promise does not reject on HTTP
/// errors — it only rejects on network errors. You must use then
/// handlers to check for HTTP errors.
pub async fn fetch<'a>(resourse: impl Into<Resource<'a>>) -> Result<Response> {
    let promise = match resourse.into() {
        Resource::String(string) => window().fetch_with_str(&string),
        Resource::Request(request) => window().fetch_with_request(&request.into()),
    };

    let raw_response = JsFuture::from(promise)
        .await
        .map(Into::into)
        .map_err(|js_value_error| FetchError::DomException(js_value_error.into()))?;

    Ok(Response { raw_response })
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum FetchError {
    SerdeError(serde_json::Error),
    DomException(web_sys::DomException),
    PromiseError(wasm_bindgen::JsValue),
}

/// Wrapper for `fetch` function single argument.
///
/// Consider using `String` or `Request` instead, because there are
/// `From` implementations for those types.
pub enum Resource<'a> {
    String(Cow<'a, str>),
    Request(Request),
}

impl<'a> From<&'a str> for Resource<'a> {
    fn from(string: &'a str) -> Resource<'a> {
        Resource::String(Cow::from(string))
    }
}

impl From<String> for Resource<'_> {
    fn from(string: String) -> Resource<'static> {
        Resource::String(Cow::from(string))
    }
}

impl From<Url> for Resource<'_> {
    fn from(url: Url) -> Resource<'static> {
        Resource::String(Cow::from(url.to_string()))
    }
}

impl From<Request> for Resource<'_> {
    fn from(request: Request) -> Resource<'static> {
        Resource::Request(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch() {
        let _ = fetch("https://seed-rs.org");
        let _ = fetch(String::from("https://seed-rs.org"));
        let _ = fetch(Url::from(vec!["/", "foo"]));
        let _ = fetch(Request::new("https://seed-rs.org"));
    }
}
