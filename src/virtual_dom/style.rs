use super::{CSSValue, St};
use indexmap::IndexMap;
use std::fmt;

/// Handle Style separately from Attrs, since it commonly involves multiple parts,
/// and has a different semantic meaning.
#[derive(Clone, Debug, PartialEq)]
pub struct Style {
    pub vals: IndexMap<St, CSSValue>,
}

impl Style {
    pub const fn new(vals: IndexMap<St, CSSValue>) -> Self {
        Self { vals }
    }

    pub fn empty() -> Self {
        Self {
            vals: IndexMap::new(),
        }
    }

    pub fn add(&mut self, key: impl Into<St>, val: impl Into<CSSValue>) {
        self.vals.insert(key.into(), val.into());
    }

    /// Combine with another Style; if there's a conflict, use the other one.
    pub fn merge(&mut self, other: Self) {
        self.vals.extend(other.vals.into_iter());
    }
}

/// Output style as a string, as would be set in the DOM as the attribute value
/// for 'style'. Eg: "display: flex; font-size: 1.5em"
impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = if self.vals.keys().len() > 0 {
            self.vals
                .iter()
                .filter_map(|(k, v)| match v {
                    CSSValue::Ignored => None,
                    CSSValue::Some(value) => Some(format!("{}:{}", k.as_str(), value)),
                })
                .collect::<Vec<_>>()
                .join(";")
        } else {
            String::new()
        };
        write!(f, "{}", string)
    }
}
