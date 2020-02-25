use crate::browser::util;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// For setting up landing page routing. Unlike normal routing, we can't rely
/// on the popstate state, so must go off path, hash, and search directly.
pub fn current() -> Url {
    let current_url = util::window().location().href().expect("get `href`");

    web_sys::Url::new(&current_url)
        .expect("create `web_sys::Url` from the current URL")
        .into()
}

/// Contains all information used in pushing and handling routes.
/// Based on [React-Reason's router](https://github.com/reasonml/reason-react/blob/master/docs/router.md).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Url {
    pub path: Vec<String>,
    pub search: Option<String>,
    pub hash: Option<String>,
    pub title: Option<String>,
}

impl Url {
    /// Helper that ignores hash, search and title, and converts path to Strings.
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/History_API)
    pub fn new<T: ToString>(path: Vec<T>) -> Self {
        Self {
            path: path.into_iter().map(|p| p.to_string()).collect(),
            hash: None,
            search: None,
            title: None,
        }
    }

    /// Builder-pattern method for defining hash.
    ///
    /// # References
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHyperlinkElementUtils/hash)
    pub fn hash(mut self, hash: &str) -> Self {
        self.hash = Some(hash.into());
        self
    }

    /// Builder-pattern method for defining search.
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHyperlinkElementUtils/search)
    pub fn search(mut self, search: &str) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl fmt::Display for Url {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // Url constructor can fail if given invalid URL. Shouldn't be possible in our case?
        let dummy_base_url = "http://example.com";
        let url = web_sys::Url::new_with_base(&self.path.join("/"), dummy_base_url)
            .expect("cannot create url");
        if let Some(search) = &self.search {
            url.set_search(search);
        }
        if let Some(hash) = &self.hash {
            url.set_hash(hash);
        }
        write!(fmt, "{}", &url.href()[dummy_base_url.len()..])
    }
}

impl From<web_sys::Url> for Url {
    fn from(url: web_sys::Url) -> Self {
        let path = {
            let mut path = url.pathname();
            // Remove leading `/`.
            path.remove(0);
            path.split('/').map(ToOwned::to_owned).collect::<Vec<_>>()
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
            path,
            hash,
            search,
            title: None,
        }
    }
}

impl TryFrom<String> for Url {
    type Error = String;

    fn try_from(relative_url: String) -> Result<Self, Self::Error> {
        let dummy_base_url = "http://example.com";
        web_sys::Url::new_with_base(&relative_url, dummy_base_url)
            .map(Url::from)
            .map_err(|_| format!("`{}` is invalid relative URL", relative_url))
    }
}

impl From<Vec<String>> for Url {
    fn from(path: Vec<String>) -> Self {
        Url::new(path)
    }
}

// todo: Do we need both from Vec<&str> and Vec<String> ?
impl From<Vec<&str>> for Url {
    fn from(path: Vec<&str>) -> Self {
        Url::new(path)
    }
}
