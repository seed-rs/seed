use super::{Result, WebSocketError};
use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
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
        self.data
            .as_string()
            .ok_or_else(|| WebSocketError::TextError("data is not a valid utf-8 string"))
    }

    /// JSON parse message data into provided type.
    ///
    /// # Errors
    ///
    /// Returns
    /// - `WebSocketError::TextError` if data isn't a valid utf-8 string.
    /// - `WebSocketError::SerdeError` when JSON decoding fails.
    pub fn json<T: DeserializeOwned + 'static>(&self) -> Result<T> {
        let text = self.text()?;
        serde_json::from_str(&text).map_err(WebSocketError::SerdeError)
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
            let bytes = JsFuture::from(blob.array_buffer())
                .await
                .map_err(WebSocketError::PromiseError)
                .map(|array_buffer| js_sys::Uint8Array::new(&array_buffer))?
                .to_vec();
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
