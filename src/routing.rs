use std::collections::HashMap;

use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::util;

/// A convenience function to prevent repetitions
fn get_path() -> String {
    let path = util::window()
        .location()
        .pathname()
        .expect("Can't find pathname");
    path[1..path.len()].to_string()
}

pub fn initial<Ms, Mdl>(
    app: crate::vdom::App<Ms, Mdl>,
    routes: HashMap<String, Ms>,
) -> crate::vdom::App<Ms, Mdl>
where
    Ms: Clone + 'static,
    Mdl: Clone + 'static,
{
    for (route, route_message) in routes.into_iter() {
        if route == get_path() {
            app.update(route_message);
            break;
        }
    }
    app
}

pub fn update_popstate_listener<Ms, Mdl>(
    app: &crate::vdom::App<Ms, Mdl>,
    routes: HashMap<String, Ms>,
) where
    Ms: Clone + 'static,
    Mdl: Clone + 'static,
{
    let window = util::window();
    if let Some(ps_closure) = app.data.popstate_closure.borrow().as_ref() {
        (window.as_ref() as &web_sys::EventTarget)
            .remove_event_listener_with_callback("popstate", ps_closure.as_ref().unchecked_ref())
            .expect("Problem removing old popstate listener");
    }

    // We can't reuse the app later to store the popstate once moved into the closure.
    let app_for_closure = app.clone();

    let closure = Closure::wrap(Box::new(move |_| {
        if let Some(route_message) = routes.get(&get_path()) {
            app_for_closure.update(route_message.clone());
        }

        // todo we currently don't use state/events.
        //            let event = event.dyn_into::<web_sys::PopStateEvent>()
        //                .expect("Unable to cast as a PopStateEvent");
        // todo: It looks like we could use either the event, or path name.
        // todo path name might be easier, since
        //                    if let Some(state) = event.state().as_string() {
        //                        crate::log("state: ".to_string() + &state);
        //                    }
    }) as Box<FnMut(web_sys::Event) + 'static>);

    (window.as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .expect("Problem adding popstate listener");
    app.data.popstate_closure.replace(Some(closure));
}

//pub fn push<Ms: Clone + 'static>(path: &str, message: Ms) {
pub fn push_route(path: &str) {
    let history = util::window().history().expect("Can't find history");
    // The second parameter, title, is currently unused by Firefox at least.
    // The first, an arbitrary state object, we could possibly use.
    // todo: Look into using state/events

    // We're documenting our API to not prepend /. Prepending / means replace
    // the existing path. Not doing so will add the path to the existing one.
    let path = &(String::from("/") + path);
    history
        .push_state_with_url(&JsValue::null(), "", Some(path))
        .expect("Problem pushing state");
}
