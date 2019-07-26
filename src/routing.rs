//! This module is decoupled / independent.

use crate::util::ClosureNew;
use serde::{Deserialize, Serialize};
use std::convert::identity;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

/// Repeated here from `seed::util`, to make this module standalone.  Once we have a Gloo module
/// that handles this, use it.
mod util {
    /// Convenience function to avoid repeating expect logic.
    pub fn window() -> web_sys::Window {
        web_sys::window().expect("Can't find the global Window")
    }

    /// Convenience function to access the `web_sys` DOM document.
    pub fn document() -> web_sys::Document {
        window().document().expect("Can't find document")
    }

    /// Convenience function to access `web_sys` history
    pub fn history() -> web_sys::History {
        window().history().expect("Can't find history")
    }
}

/// Contains all information used in pushing and handling routes.
/// Based on [React-Reason's router](https://github.com/reasonml/reason-react/blob/master/docs/router.md).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    pub path: Vec<String>,
    pub search: Option<String>,
    pub hash: Option<String>,
    pub title: Option<String>,
}

impl Url {
    /// Helper that ignores hash, search and title, and converts path to Strings.
    ///
    /// # Refenences
    /// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/History_API)
    pub fn new<T: ToString>(path: Vec<T>) -> Self {
        let result = Self {
            path: path.into_iter().map(|p| p.to_string()).collect(),
            hash: None,
            search: None,
            title: None,
        };
        clean_url(result)
    }

    /// Builder-pattern method for defining hash.
    ///
    /// # Refenences
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

impl From<String> for Url {
    // todo: Include hash and search.
    fn from(s: String) -> Self {
        let mut path: Vec<String> = s.split('/').map(ToString::to_string).collect();
        path.remove(0); // Remove a leading empty string.
        Self {
            path,
            hash: None,
            search: None,
            title: None,
        }
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

/// Get the current url path, without a prepended /
fn get_path() -> String {
    let path = util::window()
        .location()
        .pathname()
        .expect("Can't find pathname");
    path[1..path.len()].to_string() // Remove leading /
}

fn get_hash() -> String {
    util::window()
        .location()
        .hash()
        .expect("Can't find hash")
        .replace("#", "")
}

fn get_search() -> String {
    util::window()
        .location()
        .search()
        .expect("Can't find search")
        .replace("?", "")
}

/// For setting up landing page routing. Unlike normal routing, we can't rely
/// on the popstate state, so must go off path, hash, and search directly.
pub fn initial_url() -> Url {
    let raw_path = get_path();
    let path_ref: Vec<&str> = raw_path.split('/').collect();
    let path: Vec<String> = path_ref.into_iter().map(ToString::to_string).collect();

    let raw_hash = get_hash();
    let hash = match raw_hash.len() {
        0 => None,
        _ => Some(raw_hash),
    };

    let raw_search = get_search();
    let search = match raw_search.len() {
        0 => None,
        _ => Some(raw_search),
    };

    Url {
        path,
        hash,
        search,
        title: None,
    }
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

/// Remove prepended / from all items in the Url's path.
fn clean_url(mut url: Url) -> Url {
    let mut cleaned_path = vec![];
    for part in &url.path {
        if let Some(first) = part.chars().next() {
            if first == '/' {
                cleaned_path.push(remove_first(part).unwrap().to_string());
            } else {
                cleaned_path.push(part.to_string());
            }
        }
    }

    url.path = cleaned_path;
    url
}

/// Add a new route using history's `push_state` method.
///
/// # Refenences
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/History_API)
pub fn push_route<U: Into<Url>>(url: U) -> Url {
    let mut url = url.into();
    // Purge leading / from each part, if it exists, eg passed by user.
    url = clean_url(url);

    // We use data to evaluate the path instead of the path displayed in the url.
    let data =
        JsValue::from_serde(&serde_json::to_string(&url).expect("Problem serializing route data"))
            .expect("Problem converting route data to JsValue");

    // title is currently unused by Firefox.
    let title = match &url.title {
        Some(t) => t,
        None => "",
    };

    // Prepending / means replace
    // the existing path. Not doing so will add the path to the existing one.
    let mut path = String::from("/") + &url.path.join("/");
    if let Some(search) = &url.search {
        path = path + "?" + search;
    }

    if let Some(hash) = &url.hash {
        path = path + "#" + hash;
    }

    util::history()
        .push_state_with_url(&data, title, Some(&path))
        .expect("Problem pushing state");

    url
}

/// Add a listener that handles routing for navigation events like forward and back.
pub fn setup_popstate_listener<Ms>(
    update: impl Fn(Ms) + 'static,
    update_ps_listener: impl Fn(Closure<dyn FnMut(web_sys::Event)>) + 'static,
    routes: fn(Url) -> Ms,
) where
    Ms: 'static,
{
    let closure = Closure::new(move |ev: web_sys::Event| {
        let ev = ev
            .dyn_ref::<web_sys::PopStateEvent>()
            .expect("Problem casting as Popstate event");

        let url: Url = match ev.state().as_string() {
            Some(state_str) => {
                serde_json::from_str(&state_str).expect("Problem deserializing popstate state")
            }
            // This might happen if we go back to a page before we started routing. (?)
            None => {
                let empty: Vec<String> = Vec::new();
                Url::new(empty)
            }
        };
        update(routes(url));
    });

    (util::window().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .expect("Problem adding popstate listener");

    update_ps_listener(closure);
}

/// Set up a listener that intercepts clicks on elements containing an Href attribute,
/// so we can prevent page refresh for internal links, and route internally.  Run this on load.
#[allow(clippy::option_map_unit_fn)]
pub fn setup_link_listener<Ms>(update: impl Fn(Ms) + 'static, routes: fn(Url) -> Ms)
where
    Ms: 'static,
{
    let closure = Closure::new(move |event: web_sys::Event| {
        event.target()
            .and_then(|et| et.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest("[href]").ok())
            .and_then(identity)  // Option::flatten not stable (https://github.com/rust-lang/rust/issues/60258)
            .and_then(|href_el| match href_el.tag_name().as_str() {
                // Base and Link tags use href for something other than navigation.
                "Base" | "Link" => None,
                _ => Some(href_el)
            })
            .and_then(|href_el| href_el.get_attribute("href"))
            // The first character being / or empty href indicates a rel link, which is what
            // we're intercepting.
            // @TODO: Resolve it properly, see Elm implementation:
            // @TODO: https://github.com/elm/browser/blob/9f52d88b424dd12cab391195d5b090dd4639c3b0/src/Elm/Kernel/Browser.js#L157
            .and_then(|href| {
                if href.is_empty() || href.starts_with('/') {
                    Some(href)
                } else {
                    None
                }
            })
            .map(|href| {
                event.prevent_default(); // Prevent page refresh
                // @TODO should be empty href ignored?
                if !href.is_empty() {
                    // Route internally based on href's value
                    let url = push_route(Url::from(href));
                    update(routes(url));
                }
            });
    });

    (util::document().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Problem setting up link interceptor");

    closure.forget(); // todo: Can we store the closure somewhere to avoid using forget?
}
