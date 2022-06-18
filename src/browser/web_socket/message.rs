use super::{Result, WebSocketError};
#[cfg(any(feature = "serde-json", feature = "serde-wasm-bindgen"))]
use crate::browser::json;
#[cfg(any(feature = "serde-json", feature = "serde-wasm-bindgen"))]
use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::MessageEvent;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct WebSocketMessage {
    pub(crate) data: JsValue,
    pub(crate) message_event: MessageEvent,
}

impl WebSocketMessage {
    /// Return message data as `String`.
    ///
    /// # Errors
    ///
    /// Returns `WebSocketError::TextError` if data isn't a valid utf-8 string.
    pub fn text(&self) -> Result<String> {
        self.data.as_string().ok_or(WebSocketError::TextError(
            "data is not a valid utf-8 string",
        ))
    }

    /// JSON parse message data into provided type.
    ///
    /// # Errors
    ///
    /// Returns
    /// - `WebSocketError::SerdeError` when JSON decoding fails.
    #[cfg(any(feature = "serde-json", feature = "serde-wasm-bindgen"))]
    pub fn json<T>(&self) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        if self.data.has_type::<js_sys::JsString>() {
            let json_string = self.data.as_string().ok_or(WebSocketError::TextError(
                "value is not a valid utf-8 string",
            ))?;
            json::from_str(&json_string)
        } else {
            json::from_js_value(&self.data)
        }
        .map_err(WebSocketError::JsonError)
    }

    /// Return message data as `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `WebSocketError::PromiseError` when loading bytes from `Blob` fails.
    /// - `WebSocketError::TextError` if the message data isn't binary but also not a valid utf-8 string.
    pub async fn bytes(&self) -> Result<Vec<u8>> {
        if let Some(array_buffer) = self.data.dyn_ref::<js_sys::ArrayBuffer>() {
            let bytes = js_sys::Uint8Array::new(array_buffer).to_vec();
            return Ok(bytes);
        }

        if let Some(blob) = self.data.dyn_ref::<web_sys::Blob>() {
            let blob = gloo_file::Blob::from(blob.clone());
            let bytes = gloo_file::futures::read_as_bytes(&blob)
                .await
                .map_err(WebSocketError::FileReaderError)?;
            return Ok(bytes);
        }

        Ok(self.text()?.into_bytes())
    }

    /// Return message data as `Blob`.
    ///
    /// # Errors
    ///
    /// Returns `WebSocketError::TextError` if the message data is neither binary nor a valid utf-8 string.
    pub fn blob(self) -> Result<gloo_file::Blob> {
        if self.contains_array_buffer() {
            let array_buffer = self.data.unchecked_into::<js_sys::ArrayBuffer>();
            return Ok(gloo_file::Blob::new(array_buffer));
        }

        if self.contains_blob() {
            let blob = self.data.unchecked_into::<web_sys::Blob>();
            return Ok(gloo_file::Blob::from(blob));
        }

        Ok(gloo_file::Blob::new(self.text()?.as_str()))
    }

    /// Is message data `ArrayBuffer`?
    pub fn contains_array_buffer(&self) -> bool {
        self.data.has_type::<js_sys::ArrayBuffer>()
    }

    /// Is message data `Blob`?
    pub fn contains_blob(&self) -> bool {
        self.data.has_type::<web_sys::Blob>()
    }

    /// Is message data `String`?
    pub fn contains_text(&self) -> bool {
        self.data.has_type::<js_sys::JsString>()
    }

    /// Get underlying data as `wasm_bindgen::JsValue`.
    ///
    /// This is an escape path if current API can't handle your needs.
    /// Should you find yourself using it, please consider [opening an issue][issue].
    ///
    /// [issue]: https://github.com/seed-rs/seed/issues
    pub const fn raw_data(&self) -> &JsValue {
        &self.data
    }

    /// Get underlying `web_sys::MessageEvent`.
    ///
    /// This is an escape path if current API can't handle your needs.
    /// Should you find yourself using it, please consider [opening an issue][issue].
    ///
    /// [issue]: https://github.com/seed-rs/seed/issues
    pub const fn raw_message(&self) -> &web_sys::MessageEvent {
        &self.message_event
    }
}

#[cfg(test)]
pub mod tests {
    use crate::browser::web_socket::WebSocketMessage;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn get_bytes_from_message() {
        let bytes = "some test message".as_bytes();
        let blob = gloo_file::Blob::new(bytes);
        let message_event = web_sys::MessageEvent::new("test").unwrap();
        let ws_msg = WebSocketMessage {
            data: blob.into(),
            message_event,
        };
        let result_bytes = ws_msg.bytes().await.unwrap();
        assert_eq!(bytes, &*result_bytes);
    }

    use serde::{Deserialize, Serialize};
    use wasm_bindgen::JsValue;

    #[derive(Serialize, Deserialize)]
    pub struct Test {
        a: i32,
        b: i32,
    }

    #[wasm_bindgen_test]
    async fn convert_json_string_message_to_struct() {
        let test = Test { a: 1, b: 2 };
        let json_string = serde_json::to_string(&test).unwrap();
        let js_string = JsValue::from_str(&json_string);
        let message_event = web_sys::MessageEvent::new("test-event").unwrap();
        let ws_msg = WebSocketMessage {
            data: js_string,
            message_event,
        };

        let result_bytes = ws_msg.bytes().await.unwrap();
        assert_eq!(json_string.as_bytes(), &*result_bytes);
        assert_eq!(json_string, ws_msg.text().unwrap());

        let result = ws_msg.json::<Test>().unwrap();

        assert_eq!(result.a, 1);
        assert_eq!(result.b, 2);
    }
}
