//! HTTP headers

use std::borrow::Cow;

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
