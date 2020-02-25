use super::super::{
    url,
    util::{self, ClosureNew},
    Url,
};
use crate::app::{subs, Notification};
use std::convert::{TryFrom, TryInto};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

/// Add a new route using history's `push_state` method.
///
/// # References
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/History_API)
pub fn push_route<U: Into<Url>>(url: U) -> Url {
    let url = url.into();
    // We use data to evaluate the path instead of the path displayed in the url.
    let data =
        JsValue::from_str(&serde_json::to_string(&url).expect("Problem serializing route data"));

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
// pub fn setup_popstate_listener<Ms, SubMs: 'static + Any + Clone>(
pub fn setup_popstate_listener<Ms>(
    update: impl Fn(Ms) + 'static,
    updated_listener: impl Fn(Closure<dyn FnMut(web_sys::Event)>) + 'static,
    notify: impl Fn(Notification) + 'static,
    routes: Option<fn(Url) -> Option<Ms>>,
) where
    Ms: 'static,
{
    let closure = Closure::new(move |ev: web_sys::Event| {
        let ev = ev
            .dyn_ref::<web_sys::PopStateEvent>()
            .expect("Problem casting as Popstate event");

        let url = match ev.state().as_string() {
            Some(state_str) => {
                serde_json::from_str(&state_str).expect("Problem deserializing popstate state")
            }
            // Only update when requested for an update by the user.
            None => url::current(),
        };

        notify(Notification::new(subs::UrlChanged(url.clone())));

        if let Some(routes) = routes {
            if let Some(routing_msg) = routes(url) {
                update(routing_msg);
            }
        }
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
    notify: impl Fn(Notification) + 'static,
    routes: Option<fn(Url) -> Option<Ms>>,
) where
    Ms: 'static,
{
    // todo: DRY with popstate listener
    let closure = Closure::new(move |ev: web_sys::Event| {
        let ev = ev
            .dyn_ref::<web_sys::HashChangeEvent>()
            .expect("Problem casting as hashchange event");

        let url: Url = ev
            .new_url()
            .try_into()
            .expect("cast hashchange event url to `Url`");

        notify(Notification::new(subs::UrlChanged(url.clone())));

        if let Some(routes) = routes {
            if let Some(routing_msg) = routes(url) {
                update(routing_msg);
            }
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
pub fn setup_link_listener<Ms>(
    update: impl Fn(Ms) + 'static,
    notify: impl Fn(Notification) + 'static,
    routes: Option<fn(Url) -> Option<Ms>>,
) where
    Ms: 'static,
{
    let closure = Closure::new(move |event: web_sys::Event| {
        event.target()
            .and_then(|et| et.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest("[href]").ok())
            .flatten()
            .and_then(|href_el| match href_el.tag_name().to_lowercase().as_str() {
                // Base, Link and Use tags use href for something other than navigation.
                "base" | "link" | "use" => None,
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
                    let url = Url::try_from(href).expect("cast link href to `Url`");

                    // @TODO refactor while removing `routes`.
                    let url_request_controller = subs::url_requested::UrlRequest::default();
                    notify(Notification::new(subs::UrlRequested(
                        url.clone(),
                        url_request_controller.clone(),
                    )));
                    match url_request_controller.status() {
                        subs::url_requested::UrlRequestStatus::Unhandled => {
                            push_route(url.clone());
                            event.prevent_default(); // Prevent page refresh
                            notify(Notification::new(subs::UrlChanged(url.clone())));
                        }
                        subs::url_requested::UrlRequestStatus::Handled(prevent_default) => {
                            if prevent_default {
                                event.prevent_default(); // Prevent page refresh
                            }
                        }
                    }

                    if let Some(routes) = routes {
                        if let Some(redirect_msg) = routes(url.clone()) {
                            // Route internally, overriding the default history
                            push_route(url);
                            event.prevent_default(); // Prevent page refresh
                            update(redirect_msg);
                        }
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

        let actual: Url = "/path1/path2".to_string().try_into().unwrap();
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

        let actual: Url = "/path?search=query#hash".to_string().try_into().unwrap();
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

        let actual: Url = "/path#hash".to_string().try_into().unwrap();
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

        let actual: Url = "/#/discover".to_string().try_into().unwrap();
        assert_eq!(expected, actual)
    }

    #[wasm_bindgen_test]
    fn check_url_to_string() {
        let expected = "/foo/bar?q=42&z=13#/discover";

        let actual = Url {
            path: vec!["foo".into(), "bar".into()],
            hash: Some("/discover".into()),
            search: Some("q=42&z=13".into()),
            title: None,
        }
        .to_string();

        assert_eq!(expected, actual)
    }
}
