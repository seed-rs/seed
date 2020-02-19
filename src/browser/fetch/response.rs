//! The Response interface of the Fetch API represents the response to a request.
//!
//! See [developer.mozilla.org/en-US/docs/Web/API/Response](https://developer.mozilla.org/en-US/docs/Web/API/Response)

use super::{FetchError, Result, Status};
use serde::de::DeserializeOwned;
use wasm_bindgen_futures::JsFuture;

pub struct Response {
    pub(crate) raw_response: web_sys::Response,
}

impl Response {
    pub async fn text(self) -> Result<String> {
        let js_promise = self.raw_response.text().map_err(FetchError::PromiseError)?;

        let js_value = JsFuture::from(js_promise)
            .await
            .map_err(FetchError::PromiseError)?;

        Ok(js_value
            .as_string()
            .expect("fetch: Response expected `String` after .text()"))
    }

    pub async fn json<T: DeserializeOwned + 'static>(self) -> Result<T> {
        let text = self.text().await?;
        serde_json::from_str(&text).map_err(FetchError::SerdeError)
    }

    pub fn status(&self) -> Status {
        Status::from(&self.raw_response)
    }
}
