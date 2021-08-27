//! Provides a simplified (and incomplete) interface to the `FormData` Web API,
//! in order to facilitate the creation of multipart request bodies for seed's
//! Fetch API.
//!
//! ## Example
//!
//! ```
//! let form_data = FormData::new()
//!     .with_str("first-name", "Bob")
//!     .with_str("last-name", "Jones");
//!
//! Request::new("/api/")
//!     .method(Method::Post)
//!     .form_data(form_data)
//!     .fetch()
//!
//! ```
//!
//! See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/FormData) for
//! details on the behavior of the underlying API.

use crate::fetch::{FetchError, Result};
use serde::Serialize;
use serde_wasm_bindgen as swb;
use wasm_bindgen::JsValue;

pub struct FormData(web_sys::FormData);

impl FormData {
    /// Creates a new empty `FormData` object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a blob value.
    #[allow(clippy::missing_panics_doc)]
    pub fn append_blob(&mut self, name: &str, blob: &web_sys::Blob) {
        self.0.append_with_blob(name, blob).unwrap();
    }
    /// The builder-style variant of `append_blob`.
    pub fn with_blob(mut self, name: &str, blob: &web_sys::Blob) -> Self {
        self.append_blob(name, blob);
        self
    }

    /// Appends a string value.
    #[allow(clippy::missing_panics_doc)]
    pub fn append_str(&mut self, name: &str, str: &str) {
        self.0.append_with_str(name, str).unwrap();
    }
    /// The builder-style variant of `append_str`,
    pub fn with_str(mut self, name: &str, str: &str) -> Self {
        self.append_str(name, str);
        self
    }

    /// Appends a json value.
    ///
    /// ## Errors
    /// Will return `Err` if serialization fails.
    #[allow(clippy::missing_panics_doc)]
    pub fn append_json<T>(&mut self, name: &str, data: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        // @TODO Can a different `append` be used to append a `JsValue` directly?
        let str = swb::to_value(data)?
            .as_string()
            .ok_or(FetchError::ConversionError)?;
        self.0.append_with_str(name, &str).unwrap();
        Ok(())
    }
    /// The builder-style variant of `append_json`
    ///
    /// ## Errors
    /// Will return `Err` if serialization fails.
    pub fn with_json<T>(mut self, name: &str, data: &T) -> Result<Self>
    where
        T: Serialize + ?Sized,
    {
        self.append_json(name, data)?;
        Ok(self)
    }
}

impl Default for FormData {
    #[allow(clippy::missing_panics_doc)]
    fn default() -> Self {
        FormData(web_sys::FormData::new().unwrap())
    }
}

impl From<web_sys::FormData> for FormData {
    fn from(form_data: web_sys::FormData) -> Self {
        FormData(form_data)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&web_sys::HtmlFormElement> for FormData {
    fn from(form: &web_sys::HtmlFormElement) -> Self {
        FormData(web_sys::FormData::new_with_form(form).unwrap())
    }
}

impl From<FormData> for JsValue {
    fn from(form_data: FormData) -> JsValue {
        JsValue::from(form_data.0)
    }
}
