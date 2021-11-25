//! HTTP headers

use std::borrow::Cow;

// ------ Headers ------

/// Request headers.
#[derive(Clone, Debug, Default)]
pub struct Headers<'a>(Vec<Header<'a>>);

impl<'a> Headers<'a> {
    /// Create a new empty `Headers`.
    pub fn new() -> Self {
        Self::default()
    }

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

impl<'a, N, V> FromIterator<(N, V)> for Headers<'a>
where
    N: Into<Cow<'a, str>>,
    V: Into<Cow<'a, str>>,
{
    fn from_iter<I: IntoIterator<Item = (N, V)>>(iter: I) -> Self {
        let mut headers = Self::default();
        for (name, value) in iter {
            headers.set(Header::custom(name, value));
        }
        headers
    }
}

impl<'a> IntoIterator for Headers<'a> {
    type Item = Header<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[allow(clippy::fallible_impl_from)]
impl<'a, FT: AsRef<web_sys::Headers>> From<FT> for Headers<'a> {
    fn from(headers: FT) -> Self {
        // @TODO refactor once https://github.com/rustwasm/wasm-bindgen/pull/1913 is merged
        js_sys::try_iter(headers.as_ref())
            .unwrap()
            .unwrap()
            .map(|entry| js_sys::Array::from(&entry.unwrap()))
            .map(|entry| {
                (
                    entry.get(0).as_string().unwrap(),
                    entry.get(1).as_string().unwrap(),
                )
            })
            .collect()
    }
}

// ------ Header ------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) value: Cow<'a, str>,
}

impl<'a> Header<'a> {
    /// The "key" of the `Header`, like `Content-Type`, etc.
    pub fn name(&'a self) -> &'a str {
        &self.name
    }

    /// The "value" of the `Header`.
    pub fn value(&'a self) -> &'a str {
        &self.value
    }

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

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_headers_from_ws_headers() {
        // ---- ARRANGE ----
        let ws_headers = web_sys::Headers::new().unwrap();
        ws_headers
            .append("a_header_name", "a_header_value")
            .unwrap();
        // ---- ACT ----
        let headers = Headers::from(&ws_headers);
        // ---- ASSERT ----
        assert_eq!(
            headers.into_iter().next().unwrap(),
            Header::custom("a_header_name", "a_header_value")
        );
    }
}
