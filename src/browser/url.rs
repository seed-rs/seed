use crate::browser::util;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};
use wasm_bindgen::JsValue;

pub const DUMMY_BASE_URL: &str = "http://example.com";

/// URL used for routing.
///
/// - It represents relative URL.
/// - Two, almost identical, `Url`s that differ only with differently advanced
/// internal path or hash path iterators (e.g. `next_path_part()` was called on one of them)
/// are considered different also during comparison.
///
/// (If the features above are problems for you, create an [issue](https://github.com/seed-rs/seed/issues/new))
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Url {
    next_path_part_index: usize,
    next_hash_path_part_index: usize,
    path: Vec<String>,
    hash_path: Vec<String>,
    hash: Option<String>,
    search: Option<String>,
}

impl Url {
    /// Creates a new `Url` with the empty path.
    pub fn new() -> Self {
        Self {
            next_path_part_index: 0,
            next_hash_path_part_index: 0,
            path: Vec::new(),
            hash_path: Vec::new(),
            hash: None,
            search: None,
        }
    }

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

    /// Creates a new `Url` from `&str`.
    ///
    /// # Errors
    ///
    /// Returns error when `url` is invalid.
    pub fn from_str(url: impl AsRef<str>) -> Result<Self, String> {
        let str_url = url.as_ref();
        web_sys::Url::new_with_base(str_url, DUMMY_BASE_URL)
            .map(|url| Url::from_native_url(&url))
            .map_err(|_| format!("`{}` is invalid relative URL", str_url))
    }

    /// Creates a new `Url` from the browser native url.
    pub fn from_native_url(url: &web_sys::Url) -> Self {
        let path = {
            let path = url.pathname();
            path.split('/')
                .filter_map(|path_part| {
                    if path_part.is_empty() {
                        None
                    } else {
                        Some(path_part.to_owned())
                    }
                })
                .collect::<Vec<_>>()
        };

        let hash = {
            let mut hash = url.hash();
            if hash.is_empty() {
                None
            } else {
                // Remove leading `#`.
                hash.remove(0);
                Some(hash)
            }
        };

        let hash_path = {
            if let Some(hash) = &hash {
                hash.split('/')
                    .filter_map(|path_part| {
                        if path_part.is_empty() {
                            None
                        } else {
                            Some(path_part.to_owned())
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        };

        let search = {
            let mut search = url.search();
            if search.is_empty() {
                None
            } else {
                // Remove leading `?`.
                search.remove(0);
                Some(search)
            }
        };

        Self {
            next_path_part_index: 0,
            next_hash_path_part_index: 0,
            path,
            hash_path,
            hash,
            search,
        }
    }

    /// Creates a new `Url` from the one that is currently set in the browser.
    pub fn current() -> Url {
        let current_url = util::window().location().href().expect("get `href`");
        Url::from_str(&current_url).expect("create `web_sys::Url` from the current URL")
    }

    /// Advances the internal path iterator and returns the next path part as `Option<&str>`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///match url.next_path_part() {
    ///    None => Page::Home,
    ///    Some("report") => Page::Report(page::report::init(url)),
    ///    _ => Page::Unknown(url),
    ///}
    /// ````
    pub fn next_path_part(&mut self) -> Option<&str> {
        let path_part = self.path.get(self.next_path_part_index);
        if path_part.is_some() {
            self.next_path_part_index += 1;
        }
        path_part.map(String::as_str)
    }

    /// Advances the internal hash path iterator and returns the next hash path part as `Option<&str>`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///match url.next_hash_path_part() {
    ///    None => Page::Home,
    ///    Some("report") => Page::Report(page::report::init(url)),
    ///    _ => Page::Unknown(url),
    ///}
    /// ````
    pub fn next_hash_path_part(&mut self) -> Option<&str> {
        let hash_path_part = self.hash_path.get(self.next_hash_path_part_index);
        if hash_path_part.is_some() {
            self.next_hash_path_part_index += 1;
        }
        hash_path_part.map(String::as_str)
    }

    /// Collects the internal path iterator and returns it as `Vec<&str>`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///match url.remaining_path_parts().as_slice() {
    ///    [""] | [] => Page::Home,
    ///    ["report", rest @ ..] => {
    ///        match rest {
    ///            ["day"] => Page::ReportDay,
    ///            _ => Page::ReportWeek,
    ///        }
    ///    },
    ///    _ => Page::NotFound,
    ///}
    /// ````
    pub fn remaining_path_parts(&mut self) -> Vec<&str> {
        let path_part_index = self.next_path_part_index;
        self.next_path_part_index = self.path.len();
        self.path
            .iter()
            .skip(path_part_index)
            .map(String::as_str)
            .collect()
    }

    /// Collects the internal hash path iterator and returns it as `Vec<&str>`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///match url.remaining_hash_path_parts().as_slice() {
    ///    [""] | [] => Page::Home,
    ///    ["report", rest @ ..] => {
    ///        match rest {
    ///            ["day"] => Page::ReportDay,
    ///            _ => Page::ReportWeek,
    ///        }
    ///    },
    ///    _ => Page::NotFound,
    ///}
    /// ````
    pub fn remaining_hash_path_parts(&mut self) -> Vec<&str> {
        let hash_path_part_index = self.next_hash_path_part_index;
        self.next_hash_path_part_index = self.hash_path.len();
        self.hash_path
            .iter()
            .skip(hash_path_part_index)
            .map(String::as_str)
            .collect()
    }

    /// Adds given path part and returns updated `Url`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///let link_to_blog = url.add_path_part("blog");
    /// ````
    pub fn add_path_part(mut self, path_part: impl ToString) -> Self {
        self.path.push(path_part.to_string());
        self
    }

    /// Adds given hash path part and returns updated `Url`.
    /// It also changes `hash`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///let link_to_blog = url.add_hash_path_part("blog");
    /// ````
    pub fn add_hash_path_part(mut self, hash_path_part: impl ToString) -> Self {
        self.hash_path.push(hash_path_part.to_string());
        self.hash = Some(self.hash_path.join("/"));
        self
    }

    /// Clone the `Url` and strip remaining path parts.
    pub fn to_base_url(&self) -> Self {
        let mut url = self.clone();
        url.path.truncate(self.next_path_part_index);
        url
    }

    /// Clone the `Url` and strip remaining hash path parts.
    pub fn to_hash_base_url(&self) -> Self {
        let mut url = self.clone();
        url.hash_path.truncate(self.next_hash_path_part_index);
        url
    }

    /// Sets path and returns updated `Url`. It also resets internal path iterator.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// Url::new().set_path(&["my", "path"])
    /// ```
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn set_path<T: ToString>(mut self, into_path_iterator: impl IntoIterator<Item = T>) -> Self {
        self.path = into_path_iterator.into_iter().map(|p| p.to_string()).collect();
        self.next_path_part_index = 0;
        self
    }

    /// Sets hash path and returns updated `Url`.
    /// It also resets internal hash path iterator and sets `hash`.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// Url::new().set_hash_path(&["my", "path"])
    /// ```
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn set_hash_path<T: ToString>(mut self, into_hash_path_iterator: impl IntoIterator<Item = T>) -> Self {
        self.hash_path = into_hash_path_iterator.into_iter().map(|p| p.to_string()).collect();
        self.next_hash_path_part_index = 0;
        self.hash = Some(self.hash_path.join("/"));
        self
    }

    /// Sets hash and returns updated `Url`.
    /// I also sets `hash_path`.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// Url::new().set_hash("my_hash")
    /// ```
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    pub fn set_hash(mut self, hash: impl ToString) -> Self {
        let hash = hash.to_string();
        self.hash_path = hash.split("/").map(ToOwned::to_owned).collect();
        self.hash = Some(hash);
        self
    }

    /// Sets search and returns updated `Url`.
    ///
    /// # Example
    ///
    /// ```rust, no_run
    /// Url::new().set_search("x=1&y=2")
    /// ```
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub fn set_search(mut self, search: impl ToString) -> Self {
        self.search = Some(search.to_string());
        self
    }

    /// Get path.
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn path(&self) -> &[String] {
        &self.path
    }

    /// Get hash path.
    pub fn hash_path(&self) -> &[String] {
        &self.path
    }

    /// Get hash.
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    pub fn hash(&self) -> Option<&String> {
        self.hash.as_ref()
    }

    /// Get search.
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub fn search(&self) -> Option<&String> {
        self.search.as_ref()
    }

    /// Change the browser URL and trigger a page load.
    pub fn go_and_load(&self) {
        util::window().location().set_href(&self.to_string()).expect("set location href");
    }

    /// Change the browser URL and trigger a page load.
    ///
    /// Provided `url` isn't checked and it's passed into `location.href`.
    pub fn go_and_load_with_str(url: impl AsRef<str>) {
        util::window().location().set_href(url.as_ref()).expect("set location href");
    }

    /// Trigger a page reload.
    pub fn reload() {
        util::window().location().reload().expect("reload location");
    }

    /// Trigger a page reload and force reloading from the server.
    pub fn reload_and_skip_cache() {
        util::window().location().reload_with_forceget(true).expect("reload location with forceget");
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

    /// If the current `Url`'s path prefix is equal to `path_base`,
    /// then reset the internal path iterator and advance it to skip the prefix (aka `path_base`).
    ///
    /// It's used mostly by Seed internals, but it can be useful in combination
    /// with `orders.clone_base_path()`.
    pub fn skip_base_path(mut self, path_base: &[String]) -> Self {
        if self.path.starts_with(path_base) {
            self.next_path_part_index = path_base.len();
        }
        self
    }
}

impl fmt::Display for Url {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let url = web_sys::Url::new_with_base(&self.path.join("/"), DUMMY_BASE_URL)
            .expect("create native url");

        if let Some(search) = &self.search {
            url.set_search(search);
        }

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

// @TODO write tests or move here the ones from `routing.rs` and maybe refactor `from_native_url`.
