//! The Response interface of the Fetch API represents the response to a request.

use super::{FetchError, Result, Status};
use serde::de::DeserializeOwned;
use wasm_bindgen_futures::JsFuture;

/// Response of the fetch request.
/// To get one you need to use [`fetch`](./fn.fetch.html) function.
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Response)
pub struct Response {
    pub(crate) raw_response: web_sys::Response,
}

impl Response {
    /// Get a `String` from response body.
    ///
    /// # Errors
    /// Returns `FetchError::PromiseError`.
    pub async fn text(&self) -> Result<String> {
        Ok(self
            .raw_response
            .text()
            .map_err(FetchError::PromiseError)
            .map(JsFuture::from)?
            .await
            .map_err(FetchError::PromiseError)?
            .as_string()
            .expect("fetch: Response expected `String` after .text()"))
    }

    /// JSON parse response body into provided type.
    ///
    /// # Errors
    /// Returns `FetchError::SerdeError` or `FetchError::PromiseError`.
    pub async fn json<T: DeserializeOwned + 'static>(&self) -> Result<T> {
        let text = self.text().await?;
        serde_json::from_str(&text).map_err(FetchError::SerdeError)
    }

    /// Return response body as `Vec<u8>`.
    ///
    /// # Errors
    /// Returns `FetchError::PromiseError`.
    pub async fn bytes(&self) -> Result<Vec<u8>> {
        Ok(self
            .raw_response
            .array_buffer()
            .map_err(FetchError::PromiseError)
            .map(JsFuture::from)?
            .await
            .map_err(FetchError::PromiseError)
            .map(|array_buffer| js_sys::Uint8Array::new(&array_buffer))?
            .to_vec())
    }

    /// Get request status.
    pub fn status(&self) -> Status {
        Status::from(&self.raw_response)
    }

    /// Check that response status is ok (2xx).
    ///
    /// ```rust
    /// fetch(url).await?.check_status()?
    ///
    /// ```
    ///
    /// Or with combinators:
    ///
    /// ```rust
    /// fetch(url)
    ///     .map(|result| result.and_then(Response::check_status))
    ///     .and_then(Response.json)
    ///     .map(Msg::Fetched)
    /// ```
    ///
    /// # Errors
    /// Returns `FetchError::StatusError` if status isn't 2xx.
    pub fn check_status(self) -> Result<Self> {
        let status = self.status();
        if status.is_ok() {
            Ok(self)
        } else {
            Err(FetchError::StatusError(status))
        }
    }

    /// Get underlying `web_sys::Response`.
    ///
    /// This is an escape path if current API can't handle your needs.
    /// Should you find yourself using it, please consider [opening an issue][issue].
    ///
    /// [issue]: https://github.com/seed-rs/seed/issues
    pub const fn raw_response(&self) -> &web_sys::Response {
        &self.raw_response
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Obj {
        key: String,
    }

    #[wasm_bindgen_test]
    async fn response_json() {
        let response = Response {
            raw_response: web_sys::Response::new_with_opt_str(Some(r#"{ "key": "value" }"#))
                .unwrap(),
        };

        let obj: Obj = response.json().await.unwrap();
        assert_eq!(obj.key, "value");
    }

    #[wasm_bindgen_test]
    async fn response_string() {
        let response = Response {
            raw_response: web_sys::Response::new_with_opt_str(Some("response")).unwrap(),
        };

        let string = response.text().await.unwrap();
        assert_eq!(string, "response");
    }

    #[wasm_bindgen_test]
    async fn response_bytes() {
        let mut body = Vec::from(&b"response"[..]);
        let response = Response {
            raw_response: web_sys::Response::new_with_opt_u8_array(Some(&mut body)).unwrap(),
        };

        let vec = response.bytes().await.unwrap();
        assert_eq!(&vec, b"response");
    }
}
