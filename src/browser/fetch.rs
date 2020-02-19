//! Fetch API.
//!
//! Seed Fetch API is very similar to the browser [native one][fetch-mdn].
//!
//! There is one entry point: [`fetch`][fetch] function.
//! It can accept both, string urls as well as [`Request`][request].
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
//! Use [`Request`][request] methods to set init options:
//! ```rust
//! fetch(Request::new(url).method(Method::Post)).await
//! ```
//!
//!
//! [fetch]: ./fn.fetch.html
//! [request]: ./struct.Request.html
//! [response]: ./struct.Response.html
//! [status]: ./struct.Status.html
//! [fetch-mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API

use crate::browser::Url;
use crate::util::window;
use serde_json;
use std::borrow::Cow;
use wasm_bindgen_futures::JsFuture;
use web_sys;

pub mod header;
mod method;
mod request;
mod response;
mod status;

pub use header::{Header, Headers};
pub use method::*;
pub use request::*;
pub use response::*;
pub use status::*;

/// Convenient type alias.
pub type Result<T> = std::result::Result<T, FetchError>;

/// The main Fetch API function.
/// It fires a HTTP request.
///
/// ## Examples
///
/// Simple `GET` request:
/// ```rust
/// let response = fetch("https://seed-rs.org").await?;
/// let body = response.text().await?;
/// ```
///
/// `POST` request with `JSON` body:
/// ```rust
/// let form = Form{email: "foo@example.com"};
/// let response = fetch(Request::new("/api").method(Method::Post).json(form)).await?;
/// let data: SubmitResponse = response.json().await?;
/// ```
///
/// ## Errors
///
/// `fetch` will return `Err` only on network errors. This means that
/// even if you get `Ok` from this function, you still need to check
/// `Response` status for HTTP errors.
pub async fn fetch<'a>(resourse: impl Into<Resource<'a>>) -> Result<Response> {
    let promise = match resourse.into() {
        Resource::String(string) => window().fetch_with_str(&string),
        Resource::Request(request) => window().fetch_with_request(&request.into()),
    };

    let raw_response = JsFuture::from(promise)
        .await
        .map(Into::into)
        .map_err(FetchError::NetworkError)?;

    Ok(Response { raw_response })
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum FetchError {
    SerdeError(serde_json::Error),
    DomException(web_sys::DomException),
    PromiseError(wasm_bindgen::JsValue),
    NetworkError(wasm_bindgen::JsValue),
    StatusError(Status),
}

/// Wrapper for `fetch` function single argument.
///
/// Consider using `String` or `Request` instead, because there are
/// `From` implementations for those types.
pub enum Resource<'a> {
    String(Cow<'a, str>),
    Request(Request<'a>),
}

impl<'a, T> From<T> for Resource<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(url: T) -> Self {
        Resource::String(url.into())
    }
}

impl From<Url> for Resource<'_> {
    fn from(url: Url) -> Resource<'static> {
        Resource::String(Cow::from(url.to_string()))
    }
}

impl<'a> From<Request<'a>> for Resource<'a> {
    fn from(request: Request<'a>) -> Resource<'a> {
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
