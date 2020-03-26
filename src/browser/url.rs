use crate::browser::util;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};
use wasm_bindgen::JsValue;

/// Contains all information used in pushing and handling routes.
/// Based on [React-Reason's router](https://github.com/reasonml/reason-react/blob/master/docs/router.md).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Url {
    next_path_part_index: usize,
    path: Vec<String>,
    search: Option<String>,
    hash: Option<String>,
}

impl Url {
    /// Creates a new `Url`.
    pub fn new() -> Self {
        Self {
            next_path_part_index: 0,
            path: Vec::new(),
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

    /// Creates a new `Url` from `&str` that represents relative url.
    ///
    /// # Errors
    ///
    /// Returns error when `relative_url` is invalid relative url.
    pub fn relative_from_str(relative_url: &str) -> Result<Self, String> {
        let dummy_base_url = "http://example.com";
        web_sys::Url::new_with_base(&relative_url, dummy_base_url)
            .map(|url| Url::relative_from_native_url(&url))
            .map_err(|_| format!("`{}` is invalid relative URL", relative_url))
    }

    /// Creates a new `Url` from browser native url.
    pub fn relative_from_native_url(url: &web_sys::Url) -> Self {
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
            path,
            hash,
            search,
        }
    }

    /// Creates a new `Url` from the one that is currently set in the browser.
    pub fn current() -> Url {
        let current_url = util::window().location().href().expect("get `href`");
        Url::relative_from_str(&current_url).expect("create `web_sys::Url` from the current URL")
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

    pub fn to_base_url(&self) -> Self {
        let mut url = self.clone();
        url.path.truncate(self.next_path_part_index);
        url
    }

    /// Sets path and returns updated `Url`. It also resets internal path iterator.
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn set_path<T: ToString>(mut self, path_iterator: impl Iterator<Item = T>) -> Self {
        self.path = path_iterator.map(|p| p.to_string()).collect();
        self.next_path_part_index = 0;
        self
    }

    /// Sets hash and returns updated `Url`.
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    pub fn set_hash(mut self, hash: impl ToString) -> Self {
        self.hash = Some(hash.to_string());
        self
    }

    /// Sets search and returns updated `Url`.
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

    pub fn go_and_load(&self) {
        todo!("browser back")
    }

    pub fn reload() {
        todo!("browser back")
    }

    pub fn reload_and_skip_cache(&self) {
        todo!("browser back")
    }

    pub fn go_back(_steps: u32) {
        todo!("browser back")
    }

    pub fn go_forward(_steps: u32) {
        todo!("browser back")
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut path = self.path().join("/");
        if !path.starts_with("/") {
            path = "/".to_owned() + &path;
        }
        if let Some(search) = self.search() {
            path = path + "?" + search;
        }
        if let Some(hash) = self.hash() {
            path = path + "#" + hash;
        }
        write!(f, "{}", path)
    }
}

// @TODO remove or replace the current implementation?
// impl fmt::Display for Url {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         // Url constructor can fail if given invalid URL. Shouldn't be possible in our case?
//         let dummy_base_url = "http://example.com";
//         let url = web_sys::Url::new_with_base(&self.path.join("/"), dummy_base_url)
//             .expect("cannot create url");
//         if let Some(search) = &self.search {
//             url.set_search(search);
//         }
//         if let Some(hash) = &self.hash {
//             url.set_hash(hash);
//         }
//         write!(fmt, "{}", &url.href()[dummy_base_url.len()..])
//     }
// }

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
