//! HTTP headers

use std::borrow::Cow;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

/// Request headers.
#[derive(Clone, Debug, Default)]
pub struct Headers<'a>(Vec<Header<'a>>);

impl<'a> Headers<'a> {
    /// Sets a new value for an existing header or adds the header if
    /// it does not already exist.
    pub fn set(&mut self, header: Header<'a>) {
        self.0.retain(|old_header| header.name != old_header.name);
        self.0.push(header);
    }

    /// Add the header.
    ///
    /// Headers with the same name are not modified or removed.
    pub fn add(&mut self, header: Header<'a>) {
        self.0.push(header);
    }
}

impl<'a> IntoIterator for Headers<'a> {
    type Item = Header<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> From<web_sys::Headers> for Headers<'a> {
    fn from(hs: web_sys::Headers) -> Self {
        let mut headers = Headers::default();
        let js: &JsValue = hs.as_ref();

        // FIXME This `into_serde` decodes successfully, but the resulting
        // `HashMap` had nothing in it. It's unclear if the original `Headers`
        // had any content though, so I'm not sure if this is working as
        // intended.
        if let Ok(hm) = js.into_serde::<HashMap<String, String>>() {
            for (h, v) in hm {
                let header = Header::custom(h, v);
                headers.set(header);
            }
        }

        headers
    }
}

#[derive(Clone, Debug)]
pub struct Header<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) value: Cow<'a, str>,
}

impl<'a> Header<'a> {
    /// Create `Content-Type` header.
    pub fn content_type(value: impl Into<Cow<'a, str>>) -> Header<'a> {
        Self::custom("Content-Type", value)
    }

    /// Create `Authorization` header.
    pub fn authorization(value: impl Into<Cow<'a, str>>) -> Header<'a> {
        Self::custom("Authorization", value)
    }

    /// Create `Authorization: Bearer xxx` header.
    pub fn bearer(token: impl Into<Cow<'a, str>>) -> Header<'a> {
        Self::custom("Authorization", format!("Bearer {}", token.into()))
    }

    /// Create custom header.
    pub fn custom(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Header<'a> {
        Header {
            name: name.into(),
            value: value.into(),
        }
    }
}
