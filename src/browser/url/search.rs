use super::Url;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap, fmt};

#[allow(clippy::module_name_repetitions)]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UrlSearch {
    search: BTreeMap<String, Vec<String>>,
    pub(super) invalid_components: Vec<String>,
}

impl UrlSearch {
    /// Create an empty `UrlSearch` object.
    pub fn new() -> Self {
        Self {
            search: BTreeMap::new(),
            invalid_components: Vec::new(),
        }
    }

    /// Returns `true` if the `UrlSearch` contains a value for the specified key.
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.search.contains_key(key.as_ref())
    }

    /// Returns a reference to values corresponding to the key.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&Vec<String>> {
        self.search.get(key.as_ref())
    }

    /// Returns a mutable reference to values corresponding to the key.
    pub fn get_mut(&mut self, key: impl AsRef<str>) -> Option<&mut Vec<String>> {
        self.search.get_mut(key.as_ref())
    }

    /// Push the value into the vector of values corresponding to the key.
    /// - If the key and values are not present, they will be crated.
    pub fn push_value<'a>(&mut self, key: impl Into<Cow<'a, str>>, value: String) {
        let key = key.into();
        if self.search.contains_key(key.as_ref()) {
            self.search.get_mut(key.as_ref()).unwrap().push(value);
        } else {
            self.search.insert(key.into_owned(), vec![value]);
        }
    }

    /// Inserts a key-values pair into the `UrlSearch`.
    /// - If the `UrlSearch` did not have this key present, `None` is returned.
    /// - If the `UrlSearch` did have this key present, old values are overwritten by new ones,
    /// and old values are returned. The key is not updated.
    pub fn insert(&mut self, key: String, values: Vec<String>) -> Option<Vec<String>> {
        self.search.insert(key, values)
    }

    /// Removes a key from the `UrlSearch`, returning values at the key
    /// if the key was previously in the `UrlSearch`.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<Vec<String>> {
        self.search.remove(key.as_ref())
    }

    /// Gets an iterator over the entries of the `UrlSearch`, sorted by key.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.search.iter()
    }

    /// Get invalid components.
    ///
    /// Undecodable / unparsable components are invalid.
    pub fn invalid_components(&self) -> &[String] {
        &self.invalid_components
    }

    /// Get mutable invalid components.
    ///
    /// Undecodable / unparsable components are invalid.
    pub fn invalid_components_mut(&mut self) -> &mut Vec<String> {
        &mut self.invalid_components
    }
}

/// `UrlSearch` components are automatically encoded.
impl fmt::Display for UrlSearch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let params = web_sys::UrlSearchParams::new().expect("create a new UrlSearchParams");

        for (key, values) in &self.search {
            for value in values {
                params.append(key, value);
            }
        }
        write!(fmt, "{}", String::from(params.to_string()))
    }
}

impl From<web_sys::UrlSearchParams> for UrlSearch {
    /// Creates a new `UrlSearch` from the browser native `UrlSearchParams`.
    /// `UrlSearch`'s components are decoded if possible. When decoding fails, the component is cloned
    /// into `invalid_components` and the original value is used.
    fn from(params: web_sys::UrlSearchParams) -> Self {
        let mut url_search = Self::default();
        let mut invalid_components = Vec::<String>::new();

        for param in js_sys::Array::from(&params).to_vec() {
            let key_value_pair = js_sys::Array::from(&param).to_vec();

            let key = key_value_pair
                .get(0)
                .expect("get UrlSearchParams key from key-value pair")
                .as_string()
                .expect("cast UrlSearchParams key to String");
            let value = key_value_pair
                .get(1)
                .expect("get UrlSearchParams value from key-value pair")
                .as_string()
                .expect("cast UrlSearchParams value to String");

            let key = match Url::decode_uri_component(&key) {
                Ok(decoded_key) => decoded_key,
                Err(_) => {
                    invalid_components.push(key.clone());
                    key
                }
            };
            let value = match Url::decode_uri_component(&value) {
                Ok(decoded_value) => decoded_value,
                Err(_) => {
                    invalid_components.push(value.clone());
                    value
                }
            };

            url_search.push_value(key, value)
        }

        url_search.invalid_components = invalid_components;
        url_search
    }
}

impl<K, V, VS> std::iter::FromIterator<(K, VS)> for UrlSearch
where
    K: Into<String>,
    V: Into<String>,
    VS: IntoIterator<Item = V>,
{
    fn from_iter<I: IntoIterator<Item = (K, VS)>>(iter: I) -> Self {
        let search = iter
            .into_iter()
            .map(|(k, vs)| {
                let k = k.into();
                let v: Vec<_> = vs.into_iter().map(Into::into).collect();
                (k, v)
            })
            .collect();
        Self {
            search,
            invalid_components: Vec::new(),
        }
    }
}

impl<'a, K, V, VS> std::iter::FromIterator<&'a (K, VS)> for UrlSearch
where
    K: 'a,
    &'a K: Into<String>,
    V: Into<String>,
    VS: 'a,
    &'a VS: IntoIterator<Item = V>,
{
    fn from_iter<I: IntoIterator<Item = &'a (K, VS)>>(iter: I) -> Self {
        iter.into_iter()
            .map(|(k, vs)| (k.into(), vs.into_iter()))
            .collect()
    }
}
