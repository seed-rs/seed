use crate::fetch::{FetchError, Result};
use serde::Serialize;
use wasm_bindgen::JsValue;

pub struct FormData(web_sys::FormData);

impl FormData {
    pub fn new() -> Self {
        Self::default()
    }

        self.0.append_with_blob(name, blob).unwrap();
    }
    pub fn with_blob(mut self, name: &str, blob: &web_sys::Blob) -> Self {
        self.append_blob(name, blob);
        self
    }

    pub fn append_str(&mut self, name: &str, str: &str) {
        self.0.append_with_str(name, str).unwrap();
    }
    pub fn with_str(mut self, name: &str, str: &str) -> Self {
        self.append_str(name, str);
        self
    }

    pub fn append_json<T>(&mut self, name: &str, data: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        let str = serde_json::to_string(data).map_err(FetchError::SerdeError)?;
        self.0.append_with_str(name, &str).unwrap();
        Ok(())
    }
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
