//! See readme for details.

#![allow(unused_macros)]

use std::collections::HashMap;
use std::panic;

pub mod dom_types;
pub mod fetch;
pub mod routing;
#[macro_use]
mod shortcuts;
pub mod storage;
mod vdom;
mod websys_bridge;

// todo: Why does this work without importing web_sys??

//// todos:
// todo local storage
// todo maybe?? High-level css-grid and flex api?
// todo Async conflicts with events stepping on each other ?
// todo keyed elements??
// todo: Msg as copy type?

pub use crate::{
    //    dom_types::{did_mount, did_update, will_unmount},  // todo: Here or in prelude?
    fetch::{Method, RequestOpts, fetch, get, post},
    websys_bridge::{to_input, to_kbevent, to_select, to_textarea, to_html_el},
    routing::push_route,
    vdom::App // todo temp?
};

/// Convenience function to access the web_sys DOM document.
pub fn document() -> web_sys::Document {
    web_sys::window()
        .expect("Can't find window")
        .document()
        .expect("Can't find document")
}

/// Create an element flagged in a way that it will not be rendered. Useful
/// in ternary operations.
pub fn empty<Ms: Clone>() -> dom_types::El<Ms> {
    // The tag doesn't matter here, but this seems semantically appropriate.
    let mut el = dom_types::El::empty(dom_types::Tag::Del);
    el.add_attr("dummy-element".into(), "true".into());
    el
}

/// A convenience function for logging to the web browser's console.  See also
/// the log! macro, which is more flexible.
pub fn log<S: ToString>(text: S) {
    web_sys::console::log_1(&text.to_string().into());
}

// todo: Perhaps put did_mount etc here so we call with seed:: instead of in prelude.
// todo or maybe not, for consistency with events.

/// Introduce El into the global namespace for convenience (It will be repeated
/// often in the output type of components), and UpdateEl, which is required
/// for element-creation macros, input event constructors, and the History struct.
pub mod prelude {
    pub use crate::dom_types::{
        El, UpdateEl, simple_ev, input_ev, keyboard_ev, raw_ev, did_mount, did_update, will_unmount
    };
    pub use std::collections::HashMap;
}

/// App initialization: Collect its fundamental components, setup, and perform
/// an initial render.
pub fn run<Ms, Mdl>(
    model: Mdl,
    update: fn(Ms, Mdl) -> Mdl,
    view: fn(vdom::App<Ms, Mdl>, Mdl) -> dom_types::El<Ms>,
    mount_point_id: &str,
    routes: Option<HashMap<String, Ms>>)
    where Ms: Clone + 'static, Mdl: Clone + 'static
{
    let mut app = vdom::App::new(model.clone(), update, view, mount_point_id, routes.clone());

    // Our initial render. Can't initialize in new due to mailbox() requiring self.
    // todo maybe have view take an update_dom instead of whole app??
    let mut topel_vdom = (app.data.view)(app.clone(), model);  // todo clone, etc.
    let document = &web_sys::window().unwrap().document().unwrap();
    vdom::setup_els(&document, &mut topel_vdom, 0, 0);

    vdom::attach_listeners(&mut topel_vdom, &app.mailbox());

    // Attach all children: This is where our initial render occurs.
    websys_bridge::attach_els(&mut topel_vdom, &app.data.mount_point);

    app.data.main_el_vdom.replace(topel_vdom);

    // If a route map is inlcluded, update the state on page load, based
    // on the starting URL. Must be set up on the server as well.
    if let Some(routes_inner) = routes {
        app = routing::initial(app, routes_inner.clone());
        routing::update_popstate_listener(&app, routes_inner);
    }

    // Allows panic messages to output to the browser console.error.
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}