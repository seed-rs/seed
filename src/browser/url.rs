use crate::browser::util;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt, str::FromStr};
use wasm_bindgen::JsValue;

mod search;
pub use search::UrlSearch;

pub const DUMMY_BASE_URL: &str = "http://example.com";

// ------ Url ------

/// URL used for routing. The struct also keeps track of the "base" path vs the "relative" path components
/// within the URL. The relative path appended to the base path forms the "absolute" path or simply, the
/// path. For example:
///
/// ```text
/// https://site.com/albums/seedlings/oak-45.png
///                  ^base^ ^----relative------^
///                  ^---------absolute--------^
/// ```
///
/// Note that methods exist to change which parts of the URL are considered the
/// "base" vs the "relative" parts. This concept also applies for "hash paths".
///
/// - It represents relative URL.
/// - Two `Url`s that represent the same absolute path but different base/relative
///   paths (e.g. `pop_path_part()` was called on one of them) are considered
///   different when compared.
///
/// (If the features above are problems for you, please [create an issue on our
/// GitHub page](https://github.com/seed-rs/seed/issues/new). Thank you!)
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Url {
    base_path_len: usize,
    base_hash_path_len: usize,
    path: Vec<String>,
    hash_path: Vec<String>,
    hash: Option<String>,
    search: UrlSearch,
    invalid_components: Vec<String>,
}

// Constructors

impl Url {
    /// Creates a new `Url` with the empty path.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Url` from the one that is currently set in the browser.
    pub fn current() -> Url {
        let current_url = util::window().location().href().expect("get `href`");
        Url::from_str(&current_url).expect("create `web_sys::Url` from the current URL")
    }
}

// Getters

impl Url {
    /// Get the (absolute) path.
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn path(&self) -> &[String] {
        &self.path
    }

    /// Get the base path.
    pub fn base_path(&mut self) -> &[String] {
        &self.path[0..self.base_path_len]
    }

    /// Get the relative path.
    pub fn relative_path(&mut self) -> &[String] {
        &self.path[self.base_path_len..]
    }

    /// Get the hash path.
    pub fn hash_path(&self) -> &[String] {
        &self.path
    }

    /// Get the hash.
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    pub fn hash(&self) -> Option<&String> {
        self.hash.as_ref()
    }

    /// Get the search parameters.
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub const fn search(&self) -> &UrlSearch {
        &self.search
    }

    /// Get a mutable version of the search parameters.
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub fn search_mut(&mut self) -> &mut UrlSearch {
        &mut self.search
    }

    /// Get the invalid components.
    ///
    /// Undecodable / unparsable components are invalid.
    pub fn invalid_components(&self) -> &[String] {
        &self.invalid_components
    }

    /// Get a mutable version of the invalid components.
    ///
    /// Undecodable / unparsable components are invalid.
    pub fn invalid_components_mut(&mut self) -> &mut Vec<String> {
        &mut self.invalid_components
    }
}

// Setters

impl Url {
    /// Sets the (absolute) path and returns the updated `Url`.
    ///
    /// It also resets the base and relative paths.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// Url::new().set_path(&["my", "path"])
    /// ```
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn set_path<T: ToString>(
        mut self,
        into_path_iterator: impl IntoIterator<Item = T>,
    ) -> Self {
        self.path = into_path_iterator
            .into_iter()
            .map(|p| p.to_string())
            .collect();
        self.base_path_len = 0;
        self
    }

    /// Sets the (absolute) hash path and returns the updated `Url`.
    ///
    /// It also resets the base and relative hash paths and sets `hash`.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// Url::new().set_hash_path(&["my", "path"])
    /// ```
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn set_hash_path<T: ToString>(
        mut self,
        into_hash_path_iterator: impl IntoIterator<Item = T>,
    ) -> Self {
        self.hash_path = into_hash_path_iterator
            .into_iter()
            .map(|p| p.to_string())
            .collect();
        self.base_hash_path_len = 0;
        self.hash = Some(self.hash_path.join("/"));
        self
    }

    /// Sets the hash and returns the updated `Url`.
    ///
    /// It also sets the hash path, effectively calling `set_hash_path`.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// Url::new().set_hash("my_hash")
    /// ```
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    pub fn set_hash(self, hash: impl Into<String>) -> Self {
        // TODO: Probably not an issue, but this effectively clones `hash` once.
        // TODO: Optionally implement a private function to handle both.
        self.set_hash_path(hash.into().split('/'))
    }

    /// Sets the search parameters and returns the updated `Url`.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// Url::new().set_search(UrlSearch::new(vec![
    ///     ("x", vec!["1"]),
    ///     ("sort_by", vec!["date", "name"]),
    /// ])
    /// ```
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub fn set_search(mut self, search: UrlSearch) -> Self {
        self.search = search.into();
        self
    }
}

// Browser actions dependent on the Url struct
// TODO: Consider moving all Browser actions into a separate `routing` module.

impl Url {
    /// Change the browser URL, but do not trigger a page load.
    ///
    /// This will add a new entry to the browser history.
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/History_API)
    pub fn go_and_push(&self) {
        // We use data to evaluate the path instead of the path displayed in the url.
        let data = JsValue::from_str(
            &serde_json::to_string(&self).expect("Problem serializing route data"),
        );

        util::history()
            .push_state_with_url(&data, "", Some(&self.to_string()))
            .expect("Problem pushing state");
    }

    /// Change the browser URL, but do not trigger a page load.
    ///
    /// This will NOT add a new entry to the browser history.
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/History_API)
    pub fn go_and_replace(&self) {
        // We use data to evaluate the path instead of the path displayed in the url.
        let data = JsValue::from_str(
            &serde_json::to_string(&self).expect("Problem serializing route data"),
        );

        util::history()
            .replace_state_with_url(&data, "", Some(&self.to_string()))
            .expect("Problem pushing state");
    }

    /// Change the browser URL and trigger a page load.
    pub fn go_and_load(&self) {
        Self::go_and_load_with_str(self.to_string())
    }
}

// Actions independent of the Url struct
// TODO: consider making these free functions

impl Url {
    /// Change the browser URL and trigger a page load.
    ///
    /// Provided `url` isn't checked and directly set to `location.href`.
    pub fn go_and_load_with_str(url: impl AsRef<str>) {
        util::window()
            .location()
            .set_href(url.as_ref())
            .expect("set location href");
    }

    /// Trigger a page reload.
    pub fn reload() {
        util::window().location().reload().expect("reload location");
    }

    /// Trigger a page reload and force reloading from the server.
    pub fn reload_and_skip_cache() {
        util::window()
            .location()
            .reload_with_forceget(true)
            .expect("reload location with forceget");
    }

    /// Move back in `History`.
    ///
    /// - `steps: 0` only reloads the current page.
    /// - Negative steps move you forward - use rather `Url::go_forward` instead.
    /// - If there is no previous page, this call does nothing.
    pub fn go_back(steps: i32) {
        util::history().go_with_delta(-steps).expect("go back");
    }

    /// Move back in `History`.
    ///
    /// - `steps: 0` only reloads the current page.
    /// - Negative steps move you back - use rather `Url::go_back` instead.
    /// - If there is no next page, this call does nothing.
    pub fn go_forward(steps: i32) {
        util::history().go_with_delta(steps).expect("go forward");
    }
}

// Url `base_path`/`active_path` manipulation

impl Url {
    /// Returns the first part of the relative path and advances the base path.
    /// Moves the first part of the relative path into the base path and returns
    /// a reference to the moved portion.
    ///
    /// The effects are as follows. Before:
    ///
    /// ```text
    /// https://site.com/albums/seedlings/oak-45.png
    ///                  ^base^ ^----relative------^
    ///                  ^---------absolute--------^
    /// ```
    ///
    /// and after:
    ///
    /// ```text
    /// https://site.com/albums/seedlings/oak-45.png
    ///                  ^-----base-----^ ^relative^
    ///                  ^---------absolute--------^
    /// ```
    ///
    /// # Code example
    ///
    /// ```rust,no_run
    ///match url.advance_base_path() {
    ///    None => Page::Home,
    ///    Some("report") => Page::Report(page::report::init(url)),
    ///    _ => Page::Unknown(url),
    ///}
    /// ````
    pub fn pop_relative_path_part(&mut self) -> Option<&str> {
        let path_part = self.path.get(self.base_path_len);
        if path_part.is_some() {
            self.base_path_len += 1;
        }
        path_part.map(String::as_str)
    }

    /// Moves the first part of the relative hash path into the base hash path
    /// and returns a reference to the moved portion, similar to `pop_relative_path`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///match url.pop_relative_hash_path() {
    ///    None => Page::Home,
    ///    Some("report") => Page::Report(page::report::init(url)),
    ///    _ => Page::Unknown(url),
    ///}
    /// ````
    pub fn pop_relative_hash_path_part(&mut self) -> Option<&str> {
        let hash_path_part = self.hash_path.get(self.base_hash_path_len);
        if hash_path_part.is_some() {
            self.base_hash_path_len += 1;
        }
        hash_path_part.map(String::as_str)
    }

    /// Moves all the components of the relative path to the base path and
    /// returns them as `Vec<&str>`.
    ///
    /// The effects are as follows. Before:
    ///
    /// ```text
    /// https://site.com/albums/seedlings/oak-45.png
    ///                  ^base^ ^----relative------^
    ///                  ^---------absolute--------^
    /// ```
    ///
    /// and after:
    ///
    /// ```text
    /// https://site.com/albums/seedlings/oak-45.png
    ///                  ^-----------base----------^
    ///                  ^---------absolute--------^
    /// ```
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///match url.consume_relative_path().as_slice() {
    ///    [] => Page::Home,
    ///    ["report", rest @ ..] => {
    ///        match rest {
    ///            ["day"] => Page::ReportDay,
    ///            _ => Page::ReportWeek,
    ///        }
    ///    },
    ///    _ => Page::NotFound,
    ///}
    /// ````
    pub fn consume_relative_path(&mut self) -> Vec<&str> {
        let path_part_index = self.base_path_len;
        self.base_path_len = self.path.len();
        self.path
            .iter()
            .skip(path_part_index)
            .map(String::as_str)
            .collect()
    }

    /// Moves all the components of the relative hash path to the base hash path
    /// and returns them as `Vec<&str>`, similar to `consume_hash_path`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///match url.consume_relative_hash_path().as_slice() {
    ///    [] => Page::Home,
    ///    ["report", rest @ ..] => {
    ///        match rest {
    ///            ["day"] => Page::ReportDay,
    ///            _ => Page::ReportWeek,
    ///        }
    ///    },
    ///    _ => Page::NotFound,
    ///}
    /// ````
    pub fn consume_relative_hash_path(&mut self) -> Vec<&str> {
        let hash_path_part_index = self.base_hash_path_len;
        self.base_hash_path_len = self.hash_path.len();
        self.hash_path
            .iter()
            .skip(hash_path_part_index)
            .map(String::as_str)
            .collect()
    }

    /// Clone the `Url` and strip relative path.
    ///
    /// The effects are as follows. Input:
    ///
    /// ```text
    /// https://site.com/albums/seedlings/oak-45.png
    ///                  ^-----base-----^ ^relative^
    ///                  ^---------absolute--------^
    /// ```
    ///
    /// and output:
    ///
    /// ```text
    /// https://site.com/albums/seedlings
    ///                  ^-----base-----^
    ///                  ^---absolute---^
    /// ```
    pub fn truncate_relative_path(mut self) -> Self {
        self.path.truncate(self.base_path_len);
        self
    }

    /// Clone the `Url` and strip relative hash path. Similar to
    /// `truncate_relative_path`.
    pub fn truncate_relative_hash_path(mut self) -> Self {
        self.hash_path.truncate(self.base_hash_path_len);
        self
    }

    /// If the current `Url`'s path starts with `path_base`, then set the base
    /// path to the provided `path_base` and the rest to the relative path.
    ///
    /// It's used mostly by Seed internals, but it can be useful in combination
    /// with `orders.clone_base_path()`.
    // TODO potentially return `Result` so that the user can act on the check.
    pub fn try_skip_base_path(mut self, path_base: &[String]) -> Self {
        if self.path.starts_with(path_base) {
            self.base_path_len = path_base.len();
        }
        self
    }

    /// Adds the given path part and returns the updated `Url`. The path
    /// part is added to the relative path.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///let link_to_blog = url.push_path_part("blog");
    /// ````
    pub fn push_path_part(mut self, path_part: impl Into<String>) -> Self {
        self.path.push(path_part.into());
        self
    }

    /// Adds the given hash path part and returns the updated `Url`.
    /// It also changes `hash`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///let link_to_blog = url.push_hash_path_part("blog");
    /// ````
    pub fn push_hash_path_part(mut self, hash_path_part: impl Into<String>) -> Self {
        self.hash_path.push(hash_path_part.into());
        self.hash = Some(self.hash_path.join("/"));
        self
    }
}

// Things that don't fit
// TODO: consider making this a free floating function, making it private, or both.

impl Url {
    /// Decodes a Uniform Resource Identifier (URI) component.
    /// Aka percent-decoding.
    ///
    /// _Note:_ All components are automatically decoded when it's possible.
    /// You can find undecodable components in the vector
    /// returned from methods `invalid_components` or `invalid_components_mut`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// Url::decode_uri_component("Hello%20G%C3%BCnter"); // => "Hello Günter"
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error when decoding fails - e.g. _"Error: malformed URI sequence"_.
    pub fn decode_uri_component(component: impl AsRef<str>) -> Result<String, JsValue> {
        let decoded = js_sys::decode_uri_component(component.as_ref())?;
        Ok(String::from(decoded))
    }
}

/// `Url` components are automatically encoded.
impl fmt::Display for Url {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let url = web_sys::Url::new_with_base(&self.path.join("/"), DUMMY_BASE_URL)
            .expect("create native url");

        url.set_search(&self.search.to_string());

        if let Some(hash) = &self.hash {
            url.set_hash(hash);
        }
        // @TODO replace with `strip_prefix` once stable.
        write!(fmt, "{}", &url.href().trim_start_matches(DUMMY_BASE_URL))
    }
}

impl<'a> From<&'a Url> for Cow<'a, Url> {
    fn from(url: &'a Url) -> Cow<'a, Url> {
        Cow::Borrowed(url)
    }
}

impl<'a> From<Url> for Cow<'a, Url> {
    fn from(url: Url) -> Cow<'a, Url> {
        Cow::Owned(url)
    }
}

impl FromStr for Url {
    type Err = String;

    /// Creates a new `Url` from `&str`.
    ///
    /// # Errors
    ///
    /// Returns error when `url` cannot be parsed.
    ///
    /// _Note:_ When only some components are undecodable, no error is returned -
    /// that components are saved into the `Url`s `invalid_components` - see methods
    /// `Url::invalid_components` and `Url::invalid_components_mut`.
    fn from_str(str_url: &str) -> Result<Self, Self::Err> {
        web_sys::Url::new_with_base(str_url, DUMMY_BASE_URL)
            .map(|url| Url::from(&url))
            .map_err(|_| format!("`{}` is invalid relative URL", str_url))
    }
}

impl From<&web_sys::Url> for Url {
    /// Creates a new `Url` from the browser native url.
    /// `Url`'s components are decoded if possible. When decoding fails, the component is cloned
    /// into `invalid_components` and the original value is used.
    fn from(url: &web_sys::Url) -> Self {
        let mut invalid_components = Vec::<String>::new();

        let path: Vec<_> = {
            let path = url.pathname();
            path.split('/')
                .filter(|path_part| !path_part.is_empty())
                .map(|path_part| match Url::decode_uri_component(path_part) {
                    Ok(decoded_path_part) => decoded_path_part,
                    Err(_) => {
                        invalid_components.push(path_part.to_owned());
                        path_part.to_string()
                    }
                })
                .collect()
        };

        let (hash, hash_path) = {
            let hash = url.hash();
            if hash.is_empty() {
                (None, Vec::new())
            } else {
                // Remove leading `#`.
                let hash = &hash['#'.len_utf8()..];

                // Decode hash path parts.
                let hash_path = hash
                    .split('/')
                    .filter(|path_part| !path_part.is_empty())
                    .map(|path_part| match Url::decode_uri_component(path_part) {
                        Ok(decoded_path_part) => decoded_path_part,
                        Err(_) => {
                            invalid_components.push(path_part.to_owned());
                            path_part.to_owned()
                        }
                    })
                    .collect();

                // Decode hash.
                let hash = match Url::decode_uri_component(&hash) {
                    Ok(decoded_hash) => decoded_hash,
                    Err(_) => {
                        invalid_components.push(hash.to_owned());
                        hash.to_owned()
                    }
                };

                // Return `(hash, hash_path)`
                (Some(hash), hash_path)
            }
        };

        let search = UrlSearch::from(url.search_params());
        invalid_components.append(&mut search.invalid_components.clone());

        Self {
            base_path_len: 0,
            base_hash_path_len: 0,
            path,
            hash_path,
            hash,
            search,
            invalid_components,
        }
    }
}

// ------ ------ Tests ------ ------

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    //(https://www.w3schools.com/tags/ref_urlencode.ASP)
    #[wasm_bindgen_test]
    fn parse_url_decoding() {
        // "/Hello Günter/path2?calc=5+6&x=1&x=2#heš"
        let expected = "/Hello%20G%C3%BCnter/path2?calc=5%2B6&x=1&x=2#he%C5%A1";
        let native_url = web_sys::Url::new_with_base(expected, DUMMY_BASE_URL).unwrap();
        let url = Url::from(&native_url);
        let expected_search: UrlSearch = vec![("calc", vec!["5+6"]), ("x", vec!["1", "2"])]
            .into_iter()
            .collect();

        assert_eq!(url.path()[0], "Hello Günter");
        assert_eq!(url.path()[1], "path2");
        assert_eq!(url.search(), &expected_search,);
        assert_eq!(url.hash(), Some(&"heš".to_owned()));

        let actual = url.to_string();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn parse_url_path() {
        let expected = Url::new().set_path(&["path1", "path2"]);
        let actual: Url = "/path1/path2".parse().unwrap();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn parse_url_with_hash_search() {
        let expected = Url::new()
            .set_path(&["path"])
            .set_search(vec![("search", vec!["query"])].into_iter().collect())
            .set_hash("hash");
        let actual: Url = "/path?search=query#hash".parse().unwrap();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn parse_url_with_hash_only() {
        let expected = Url::new().set_path(&["path"]).set_hash("hash");
        let actual: Url = "/path#hash".parse().unwrap();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn parse_url_with_hash_routing() {
        let expected = Url::new().set_hash_path(&["discover"]);
        let actual: Url = "/#discover".parse().unwrap();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn check_url_to_string() {
        let expected = "/foo/bar?q=42&z=13#discover";

        let actual = Url::new()
            .set_path(&["foo", "bar"])
            .set_search(
                vec![("q", vec!["42"]), ("z", vec!["13"])]
                    .into_iter()
                    .collect(),
            )
            .set_hash_path(&["discover"])
            .to_string();

        assert_eq!(expected, actual)
    }
}
