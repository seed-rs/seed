use std::collections::HashMap;

use wasm_bindgen::{closure::Closure, JsCast, JsValue};


/// Convenience function, mainly to avoid repeating expect logic.
fn make_window() -> web_sys::Window {
    web_sys::window().expect("Can't find the global Window.")
}

pub fn initial<Ms, Mdl>(app: crate::vdom::App<Ms, Mdl>, routes: HashMap<&str, Ms>) -> crate::vdom::App<Ms, Mdl>
    where Ms: Clone + Sized + 'static, Mdl: Clone + Sized + 'static
{
    let path = make_window().location().pathname().expect("Can't find pathname");
    for (route, route_message) in routes.into_iter() {
        if route == &path {
            app.update_dom(route_message);
            break;
        }
    }
    app
}

pub fn update_popstate_listener<Ms, Mdl>(app: crate::vdom::App<Ms, Mdl>, routes: HashMap<&str, Ms>)
    where Ms: Clone + Sized + 'static, Mdl: Clone + Sized + 'static
{

    let window = make_window();
    if let Some(ps_closure) = app.data.popstate_closure.borrow().as_ref() {
        (window.as_ref() as &web_sys::EventTarget)
            .remove_event_listener_with_callback("popstate", ps_closure.as_ref().unchecked_ref())
            .expect("Problem removing old popstate listener");
    }

    // Convert to a map over owned strings, to prevent lifetime problems in the closure.
    let mut routes_owned = HashMap::new();
    for (route, msg) in routes {
        routes_owned.insert(route.to_string(), msg);
    }

    // We can't reuse the app later to store the popstate once moved into the closure.
    let app_for_closure = app.clone();

    let closure = Closure::wrap(
        Box::new(move |event: web_sys::Event| {
            // todo we currently don't use state/events.
//            let event = event.dyn_into::<web_sys::PopStateEvent>()
//                .expect("Unable to cast as a PopStateEvent");

            let path = make_window().location().pathname().expect("Can't find pathname");
            let path_trimmed = &path[1..path.len()].to_string();

            if let Some(route_message) = routes_owned.get(path_trimmed) {
                app_for_closure.update_dom(route_message.clone());
            }

            // todo: It looks like we could use either the event, or path name.
            // todo path name might be easier, since
//                    if let Some(state) = event.state().as_string() {
//                        crate::log("state: ".to_string() + &state);
//                    }
        })
            as Box<FnMut(web_sys::Event) + 'static>,
    );

    (window.as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .expect("Problem adding popstate listener");
    app.data.popstate_closure.replace(Some(closure));
}

pub fn push<Ms: Clone + Sized + 'static>(path: &str, message: Ms) {
    let history = make_window().history().expect("Can't find history");
    // The second parameter, title, is currently unused by Firefox at least.
    // The first, an arbitrary state object, we could possibly use.
    // todo: Look into using state/events

    // We're documenting our API to not prepend /. Prepending / means replace
    // the existing path. Not doing so will add the path to the existing one.
    let path = &(String::from("/") + path);
    history.push_state_with_url(&JsValue::null(), "", Some(path))
        .expect("Problem pushing state");
}