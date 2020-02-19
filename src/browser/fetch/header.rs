//! HTTP headers

use std::borrow::Cow;

/// Request headers.
#[derive(Clone, Debug, Default)]
pub struct Headers<'a>(Vec<Header<'a>>);

impl<'a> Headers<'a> {
    /// Sets a new value for an existing header or adds the header if
    /// it does not already exist.
    pub fn set(&mut self, header: Header<'a>) {
        self.0.retain(|Header { name, .. }| &header.name != name);
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

#[derive(Clone, Debug)]
pub struct Header<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) value: Cow<'a, str>,
}

/// Create `Content-Type` header.
pub fn content_type<'a>(value: impl Into<Cow<'a, str>>) -> Header<'a> {
    custom("Content-Type", value.into())
}

/// Create `Authorization` header.
pub fn authorization<'a>(value: impl Into<Cow<'a, str>>) -> Header<'a> {
    custom("Authorization", value)
}

/// Create `Authorization: Bearer xxx` header.
pub fn bearer<'a>(value: impl Into<Cow<'a, str>>) -> Header<'a> {
    custom("Authorization", format!("Bearer {}", value.into()))
}

/// Create custom header.
pub fn custom<'a>(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Header<'a> {
    Header {
        name: name.into(),
        value: value.into(),
    }
}
