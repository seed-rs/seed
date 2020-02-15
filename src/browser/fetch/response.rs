//! The Response interface of the Fetch API represents the response to a request.
//!
//! See [developer.mozilla.org/en-US/docs/Web/API/Response](https://developer.mozilla.org/en-US/docs/Web/API/Response)

use super::{FetchError, Status};
use serde::de::DeserializeOwned;
use wasm_bindgen_futures::JsFuture;

pub struct Response {
    pub(crate) raw_response: web_sys::Response,
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
