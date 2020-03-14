use super::{At, AtValue};
use indexmap::IndexMap;
use std::fmt;

/// A thinly-wrapped `HashMap` holding DOM attributes
#[derive(Clone, Debug, PartialEq)]
pub struct Attrs {
    // We use an IndexMap instead of HashMap here, and in Style, to preserve order.
    pub vals: IndexMap<At, AtValue>,
}

/// Create an HTML-compatible string representation
impl fmt::Display for Attrs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .vals
            .iter()
            .filter_map(|(k, v)| match v {
                AtValue::Ignored => None,
                AtValue::None => Some(k.to_string()),
                AtValue::Some(value) => Some(format!("{}=\"{}\"", k.as_str(), value)),
            })
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{}", string)
    }
}

impl Attrs {
    pub const fn new(vals: IndexMap<At, AtValue>) -> Self {
        Self { vals }
    }

    pub fn empty() -> Self {
        Self {
            vals: IndexMap::new(),
        }
    }

    /// Convenience function. Ideal when there's one id, and no other attrs.
    /// Generally called with the id! macro.
    pub fn from_id(name: impl Into<AtValue>) -> Self {
        let mut result = Self::empty();
        result.add(At::Id, name.into());
        result
    }

    /// Add a new key, value pair
    pub fn add(&mut self, key: At, val: impl Into<AtValue>) {
        self.vals.insert(key, val.into());
    }

    /// Add multiple values for a single attribute. Useful for classes.
    pub fn add_multiple(&mut self, key: At, items: &[&str]) {
        self.add(
            key,
            items
                .iter()
                .filter_map(|item| {
                    if item.is_empty() {
                        None
                    } else {
                        #[allow(clippy::useless_asref)]
                        Some(item.as_ref())
                    }
                })
                .collect::<Vec<&str>>()
                .join(" "),
        );
    }

    /// Combine with another Attrs
    pub fn merge(&mut self, other: Self) {
        for (other_key, other_value) in other.vals {
            match self.vals.get_mut(&other_key) {
                Some(original_value) => {
                    Self::merge_attribute_values(&other_key, original_value, other_value);
                }
                None => {
                    self.vals.insert(other_key, other_value);
                }
            }
        }
    }

    fn merge_attribute_values(
        key: &At,
        mut original_value: &mut AtValue,
        mut other_value: AtValue,
    ) {
        match (key, &mut original_value, &mut other_value) {
            (At::Class, AtValue::Some(original), AtValue::Some(other)) => {
                if !original.is_empty() {
                    original.push(' ');
                }
                original.push_str(other);
            }
            (..) => *original_value = other_value,
        }
    }
}
