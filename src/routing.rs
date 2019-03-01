use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::{util, App};

/// Contains all information used in pushing and handling routes.
/// Based on React-Reason's router:
/// https://github.com/reasonml/reason-react/blob/master/docs/router.md
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    pub path: Vec<String>,
    pub hash: Option<String>,
    pub search: Option<String>,
    pub title: Option<String>,
}

impl Url {
    /// Helper that ignores hash, search and title, and converts path to Strings.
    /// https://developer.mozilla.org/en-US/docs/Web/API/History_API
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
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLHyperlinkElementUtils/hash
    pub fn hash(mut self, hash: &str) -> Self {
        self.hash = Some(hash.into());
        self
    }

    /// Builder-pattern method for defining search.
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLHyperlinkElementUtils/search
    pub fn search(mut self, search: &str) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// Get the current url path, without a prepended /
fn get_path() -> String {
    let path = util::window()
        .location()
        .pathname()
        .expect("Can't find pathname");
    path[1..path.len()].to_string()  // Remove leading /
}

// todo DRY
fn get_hash() -> String {
    let hash = util::window()
        .location()
        .hash()
        .expect("Can't find hash");
    hash.to_string().replace("#", "")
}

fn get_search() -> String {
    let search = util::window()
        .location()
        .search()
        .expect("Can't find search");
    search.to_string().replace("?", "")
}

/// For setting up landing page routing. Unlike normal routing, we can't rely
/// on the popstate state, so must go off path, hash, and search directly.
pub fn initial<Ms, Mdl>(app: App<Ms, Mdl>, routes: fn(&Url) -> Ms) -> App<Ms, Mdl>
    where
        Ms: Clone + 'static,
        Mdl: Clone + 'static,
{
    let raw_path = get_path();
    let path_ref: Vec<&str> = raw_path.split('/').collect();
    let path: Vec<String> = path_ref.into_iter().map(|p| p.to_string()).collect();

    let raw_hash = get_hash();
    let hash = match raw_hash.len() {
        0 => None,
        _ => Some(raw_hash)
    };

    let raw_search = get_search();
    let search = match raw_search.len() {
        0 => None,
        _ => Some(raw_search)
    };

    let url = Url {
        path,
        hash,
        search,
        title: None,
    };

    app.update(routes(&url));
    app
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

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

/// Add a new route using history's push_state method.
///https://developer.mozilla.org/en-US/docs/Web/API/History_API
pub fn push_route(mut url: Url) {

    // Purge leading / from each part, if it exists, eg passed by user.
    url = clean_url(url);

    // We use data to evaluate the path instead of the path displayed in the url.
    let data = JsValue::from_serde(
        &serde_json::to_string(&url).expect("Problem serializing route data")
    )
        .expect("Problem converting route data to JsValue");

    // title is currently unused by Firefox.
    let title = match url.title {
        Some(t) => t,
        None => "".into(),
    };


    // Prepending / means replace
    // the existing path. Not doing so will add the path to the existing one.
    let path = String::from("/") + &url.path.join("/");

    crate::log(&path);

    util::window()
        .history()
        .expect("Can't find history")
        .push_state_with_url(&data, &title, Some(&path))
        .expect("Problem pushing state");

    // Must set hash and search after push_state, or the url will be overwritten.
    let location = util::window().location();

    if let Some(hash) = url.hash {
        location.set_hash(&hash).expect("Problem setting hash");
    }

    if let Some(search) = url.search {
        location.set_search(&search).expect("Problem setting search");
    }
}

/// A convenience function, for use when only a path is required.
pub fn push_path<T: ToString>(path: Vec<T>) {
    push_route(Url::new(path));
}

pub fn setup_popstate_listener<Ms, Mdl>(app: &App<Ms, Mdl>, routes: fn(&Url) -> Ms)
    where
        Ms: Clone,
        Mdl: Clone,
{
    // We can't reuse the app later to store the popstate once moved into the closure.
    let app_for_closure = app.clone();
    let closure = Closure::wrap(
        Box::new(move |ev: web_sys::Event| {
            let ev = ev
                .dyn_ref::<web_sys::PopStateEvent>()
                .expect("Problem casting as Popstate event");

            let url: Url = match ev.state().as_string() {
                Some(state_str) => serde_json::from_str(&state_str)
                    .expect("Problem deserialzing popstate state"),
                // This might happen if we go back to a page before we started routing. (?)
                None => {
                    let empty: Vec<String> = Vec::new();
                    Url::new(empty)
                }
            };

            app_for_closure.update(routes(&url));
        })
            as Box<FnMut(web_sys::Event) + 'static>
    );

    (util::window().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback(
            "popstate",
            closure.as_ref().unchecked_ref())
        .expect("Problem adding popstate listener");

    app.data.popstate_closure.replace(Some(closure));
}

/// Set up a listener that intercepts clicks on <a> and <button> tags, so we can prevent page reloads for
/// internal links.  Run this on load.
pub fn setup_link_listener<Ms: Clone, Mdl: Clone>(app: &App<Ms, Mdl>, routes: fn(&Url) -> Ms) {
    // todo DRY with setup_popstate listener.
    // todo Deal with excessive nesting.

    let app_for_closure = app.clone();
    let closure = Closure::wrap(
        Box::new(move |event: web_sys::Event| {
            if let Some(et) = event.target() {
                if let Some(el) = et.dyn_ref::<web_sys::Element>() {
                    let tag = el.tag_name();
                    // Base and Link tags use href for something other than navigation.
                    if tag == "Base" || tag == "Link" {
                        return
                    }
                    if let Some(href) = el.get_attribute("href") {
                        if let Some(first) = href.chars().next() {
                            if first == '/' {
                                event.prevent_default();
                                // Route internally based on href
                                let url = Url::new(vec![href]);
                                app_for_closure.update(routes(&url));
                                push_route(url);
                            }
                        }
                    }

                }
            }
        })
            as Box<FnMut(web_sys::Event) + 'static>,
    );

    (util::document().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback(
            "click",
            closure.as_ref().unchecked_ref(),
        )
        .expect("Problem setting up link interceptor");

    closure.forget();  // todo: Can we store the closure somewhere to avoid using forget?
}