//! Fetch API.
//!
//! Seed Fetch API is very similar to the browser [native one][fetch-mdn].
//!
//! There is one entry point: [`fetch`][fetch] function.
//! It can accept both string urls as well as [`Request`][request].
//!
//! To get a [`Response`][response] you need to `.await` fetch:
//! ```rust
//! let response = fetch("/foo").await?;
//! ```
//!
//! Then you can check [`Status`][status] and extract body in various formats:
//! ```rust
//! let response = fetch("/foo").await?.check_status()?;
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

#[cfg(any(feature = "serde-json", feature = "serde-wasm-bindgen"))]
use crate::browser::json;
use crate::util::window;
use std::convert::TryInto;
use wasm_bindgen_futures::JsFuture;

pub mod form_data;
pub mod header;
mod method;
mod request;
mod response;
mod status;

pub use form_data::FormData;
pub use header::{Header, Headers};
pub use json::Error as JsonError;
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
/// let request = Request::new("/api").method(Method::Post).json(form).expect("Error in parsing Request");
/// let response = fetch(request).await?;
/// let data: SubmitResponse = response.json().await?;
/// ```
///
/// ## Errors
///
/// `fetch` will return `Err` only on network errors. This means that
/// even if you get `Ok` from this function, you still need to check
/// `Response` status for HTTP errors.
pub async fn fetch<'a>(request: impl Into<Request<'a>>) -> Result<Response> {
    let request = request.into();
    let promise = window().fetch_with_request(&request.try_into()?);

    let raw_response = JsFuture::from(promise)
        .await
        .map(Into::into)
        .map_err(FetchError::NetworkError)?;

    Ok(Response { raw_response })
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum FetchError {
    #[cfg(any(feature = "serde-json", feature = "serde-wasm-bindgen"))]
    JsonError(JsonError),
    DomException(web_sys::DomException),
    PromiseError(wasm_bindgen::JsValue),
    NetworkError(wasm_bindgen::JsValue),
    /// Request construction failed.
    RequestError(wasm_bindgen::JsValue),
    StatusError(Status),
}

#[cfg(any(feature = "serde-json", feature = "serde-wasm-bindgen"))]
impl From<json::Error> for FetchError {
    fn from(v: json::Error) -> Self {
        Self::JsonError(v)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use super::*;
    use crate::browser::Url;

    #[wasm_bindgen_test]
    fn test_fetch_args() {
        let _ = fetch("https://seed-rs.org");
        let _ = fetch(String::from("https://seed-rs.org"));
        let _ = fetch(Url::new().set_path(&["/", "foo"]));
        let _ = fetch(Request::new("https://seed-rs.org"));
    }
}
