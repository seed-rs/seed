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

impl From<web_sys::Url> for Url {
    fn from(url: web_sys::Url) -> Self {
        let path = {
            let mut path = url.pathname();
            // Remove leading `/`.
            path.remove(0);
            path
                .split('/')
                .map(|path_part| path_part.to_owned())
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
            path,
            hash,
            search,
            title: None,
        }
    }
}

impl From<String> for Url {
    fn from(string_url: String) -> Self {
        let dummy_base_url = "http://example.com";
        // @TODO remove unwrap
        let url = web_sys::Url::new_with_base(&string_url, dummy_base_url).unwrap();
        url.into()
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

/// For setting up landing page routing. Unlike normal routing, we can't rely
/// on the popstate state, so must go off path, hash, and search directly.
pub fn initial_url() -> Url {
    let current_url = util::window()
        .location()
        .href()
        .expect("get `href`");

    web_sys::Url::new(&current_url)
        .expect("create `web_sys::Url` from the current URL")
        .into()
}

/// Remove prepended / from all items in the Url's path.
fn clean_url(mut url: Url) -> Url {
    url.path = url.path.into_iter().map(|path_part| {
        path_part.trim_start_matches('/').to_owned()
    }).collect();
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
    updated_listener: impl Fn(Closure<dyn FnMut(web_sys::Event)>) + 'static,
    routes: fn(Url) -> Option<Ms>,
) where
    Ms: 'static,
{
    let closure = Closure::new(move |ev: web_sys::Event| {
        let ev = ev
            .dyn_ref::<web_sys::PopStateEvent>()
            .expect("Problem casting as Popstate event");

        if let Some(state_str) = ev.state().as_string() {
            let url: Url =
                serde_json::from_str(&state_str).expect("Problem deserializing popstate state");
            // Only update when requested for an update by the user.
            if let Some(routing_msg) = routes(url) {
                update(routing_msg);
            }
        };
    });

    (util::window().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .expect("Problem adding popstate listener");

    updated_listener(closure);
}

/// Add a listener that handles routing when the url hash is changed.
pub fn setup_hashchange_listener<Ms>(
    update: impl Fn(Ms) + 'static,
    updated_listener: impl Fn(Closure<dyn FnMut(web_sys::Event)>) + 'static,
    routes: fn(Url) -> Option<Ms>,
) where
    Ms: 'static,
{
    // todo: DRY with popstate listener
    let closure = Closure::new(move |ev: web_sys::Event| {
        let ev = ev
            .dyn_ref::<web_sys::HashChangeEvent>()
            .expect("Problem casting as hashchange event");

        let url: Url = ev.new_url().into();

        if let Some(routing_msg) = routes(url) {
            update(routing_msg);
        }
    });

    (util::window().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref())
        .expect("Problem adding hashchange listener");

    updated_listener(closure);
}

/// Set up a listener that intercepts clicks on elements containing an Href attribute,
/// so we can prevent page refresh for internal links, and route internally.  Run this on load.
#[allow(clippy::option_map_unit_fn)]
pub fn setup_link_listener<Ms>(update: impl Fn(Ms) + 'static, routes: fn(Url) -> Option<Ms>)
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
                // @TODO should be empty href ignored?
                if href.is_empty() {
                    event.prevent_default(); // Prevent page refresh
                } else {
                    // Only update when requested for an update by the user.
                    let url = clean_url(Url::from(href));
                    if let Some(redirect_msg) = routes(url.clone()) {
                        // Route internally, overriding the default history
                        push_route(url);
                        event.prevent_default(); // Prevent page refresh
                        update(redirect_msg);
                    }
                }
            });
    });

    (util::document().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Problem setting up link interceptor");

    closure.forget(); // todo: Can we store the closure somewhere to avoid using forget?
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn parse_url_simple() {
        let expected = Url {
            path: vec!["path1".into(), "path2".into()],
            hash: None,
            search: None,
            title: None,
        };

        let actual: Url = "/path1/path2".to_string().into();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn parse_url_with_hash_search() {
        let expected = Url {
            path: vec!["path".into()],
            hash: Some("hash".into()),
            search: Some("search=query".into()),
            title: None,
        };

        let actual: Url = "/path?search=query#hash".to_string().into();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn parse_url_with_hash_only() {
        let expected = Url {
            path: vec!["path".into()],
            hash: Some("hash".into()),
            search: None,
            title: None,
        };

        let actual: Url = "/path#hash".to_string().into();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn parse_url_with_hash_routing() {
        let expected = Url {
            path: vec!["".into()],
            hash: Some("/discover".into()),
            search: None,
            title: None,
        };

        let actual: Url = "/#/discover".to_string().into();
        assert_eq!(expected, actual)
    }
}
