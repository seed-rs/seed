use super::super::{
    util::{self, ClosureNew},
    Url,
};
use crate::app::{subs, Notification};
use std::rc::Rc;
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

    util::history()
        .push_state_with_url(&data, "", Some(&url.to_string()))
        .expect("Problem pushing state");
    url
}

/// Add a listener that handles routing for navigation events like forward and back.
pub fn setup_popstate_listener<Ms>(
    update: impl Fn(Ms) + 'static,
    updated_listener: impl Fn(Closure<dyn FnMut(web_sys::Event)>) + 'static,
    notify: impl Fn(Notification) + 'static,
    routes: Option<fn(Url) -> Option<Ms>>,
    base_path: Rc<Vec<String>>,
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
            None => Url::current(),
        };

        notify(Notification::new(subs::UrlChanged(
            url.clone().try_skip_base_path(&base_path),
        )));

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
    base_path: Rc<Vec<String>>,
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
            .parse()
            .expect("cast hashchange event url to `Url`");

        notify(Notification::new(subs::UrlChanged(
            url.clone().try_skip_base_path(&base_path),
        )));

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

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn url_request_handler(
    sub_data: subs::UrlRequested,
    base_path: Rc<Vec<String>>,
    notify: impl Fn(Notification) + 'static,
) {
    let subs::UrlRequested(url, request) = sub_data;

    match request.status() {
        subs::url_requested::UrlRequestStatus::Unhandled => {
            push_route(url.clone());
            if let Some(event) = request.event.borrow_mut().take() {
                event.prevent_default(); // Prevent page refresh
            }
            notify(Notification::new(subs::UrlChanged(
                url.try_skip_base_path(&base_path),
            )));
        }
        subs::url_requested::UrlRequestStatus::Handled(prevent_default) => {
            if prevent_default {
                if let Some(event) = request.event.borrow_mut().take() {
                    event.prevent_default(); // Prevent page refresh
                }
            }
        }
    }
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
                    let url: Url = href.parse().expect("cast link href to `Url`");

                    notify(Notification::new(subs::UrlRequested(
                        url.clone(),
                        subs::url_requested::UrlRequest::new(
                            subs::url_requested::UrlRequestStatus::default(),
                            Some(event.clone()),
                        ),
                    )));

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
