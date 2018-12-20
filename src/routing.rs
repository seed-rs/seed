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

pub fn setup_popstate_listener<Ms, Mdl>(app: crate::vdom::App<Ms, Mdl>, routes: HashMap<&str, Ms>)
    where Ms: Clone + Sized + 'static, Mdl: Clone + Sized + 'static
{
    // Convert to a map over owned strings, to prevent lifetime problems in the closure.
    let mut routes_owned = HashMap::new();
    for (route, msg) in routes {
        routes_owned.insert(route.to_string(), msg);
    }


    let history_closure = Closure::wrap(
        Box::new(move |event: web_sys::Event| {
//            let event = event.dyn_into::<web_sys::PopStateEvent>()
//                .expect("Unable to cast as a PopStateEvent");

            let path = make_window().location().pathname().expect("Can't find pathname");
            let path_trimmed = &path[1..path.len()].to_string();

            if let Some(route_message) = routes_owned.get(path_trimmed) {
                app.update_dom(route_message.clone());
            }

            // todo: It looks like we could use either the event, or path name.
            // todo path name might be easier, since
//                    if let Some(state) = event.state().as_string() {
//                        crate::log("state: ".to_string() + &state);
//                    }
        })
            as Box<FnMut(web_sys::Event) + 'static>,
    );

    (make_window().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("popstate", history_closure.as_ref().unchecked_ref())
        .expect("Problem adding popstate listener");

    history_closure.forget();  // todo: Is this leaking memory?
}

pub fn push(path: &str) {
    let history = make_window().history().expect("Can't find history");
    // The second parameter, title, is currently unused by Firefox at least.
    // The first, an arbitrary state object, we could possibly use.
    // todo: Look into using state
    history.push_state_with_url(&JsValue::from_str(""), "", Some(path))
        .expect("Problem pushing state");
}