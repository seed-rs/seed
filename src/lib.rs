//! See readme for details.

#![allow(unused_macros)]

pub mod dom_types;


pub mod shortcuts;

pub mod fetch;
pub mod storage;
mod vdom;
mod websys_bridge;

// For fetch:
#[macro_use]
extern crate serde_derive;

//// todos:
// Passing values to enums that have arguments without lifetime issues.
// todo router
// todo local storage
// todo vdom patching
// todo maybe?? High-level css-grid and flex api?
// todo Async conflicts with events stepping on each other ?
// todo keyed elements??


/// The entry point for the app
pub fn run<Ms, Mdl>(model: Mdl, update: fn(Ms, &Mdl) -> Mdl,
          view: fn(Mdl) -> dom_types::El<Ms>, mount_point_id: &str)
    where Ms: Clone + Sized + 'static, Mdl: Clone + Sized + 'static
{
    let app = vdom::App::new(model.clone(), update, view, mount_point_id);

    // Our initial render. Can't initialize in new due to mailbox() requiring self.
    let mut main_el_vdom = (app.data.view)(model);
    app.setup_vdom(&mut main_el_vdom, 0, 0);
    // Attach all children: This is where our initial render occurs.
    websys_bridge::attach(&mut main_el_vdom, &app.data.mount_point);

    app.data.main_el_vdom.replace(main_el_vdom);
}

/// Create an element flagged in a way that it will not be rendered. Useful
/// in ternary operations.
pub fn empty<Ms: Clone>() -> dom_types::El<Ms> {
    // The tag doesn't matter here, but this seems semantically appropriate.
    let mut el = dom_types::El::empty(dom_types::Tag::Del);
    el.add_attr("dummy-element".into(), "true".into());
    el
}

/// A convenience function for logging to the web browser's console.
pub fn log(text: &str) {
    web_sys::console::log_1(&text.into());
}


/// Introduce El into the global namespace for convenience (It will be repeated
/// often in the output type of components), and UpdateEl, which is required
/// for our element-creation macros.
pub mod prelude {
    pub use crate::dom_types::{El, UpdateEl, UpdateListener,
                               simple_ev, input_ev, keyboard_ev, raw_ev};
}
