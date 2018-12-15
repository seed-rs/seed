//! See readme for details.

#![allow(unused_macros)]

use std::panic;

use wasm_bindgen::JsCast;

pub mod dom_types;
pub mod fetch;
#[macro_use]
pub mod shortcuts;
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
// todo maybe?? High-level css-grid and flex api?
// todo Async conflicts with events stepping on each other ?
// todo keyed elements??
// todo composable styles and attrs?


/// Convenience function used in event handling: Convert an event target
/// to an input element; eg so you can take its value.
pub fn to_input(target: &web_sys::EventTarget ) -> &web_sys::HtmlInputElement {
    // This might be more appropriate for web_sys::bridge, but I'd
    // like to expose it without making websys_bridge public.
    target.dyn_ref::<web_sys::HtmlInputElement>().expect("Unable to cast as an input element")
}

/// See to_input
pub fn to_textarea(target: &web_sys::EventTarget ) -> &web_sys::HtmlTextAreaElement {
    // This might be more appropriate for web_sys::bridge, but I'd
    // like to expose it without making websys_bridge public.
    target.dyn_ref::<web_sys::HtmlTextAreaElement>().expect("Unable to cast as a textarea element")
}

/// See to_input
pub fn to_select(target: &web_sys::EventTarget ) -> &web_sys::HtmlSelectElement {
    // This might be more appropriate for web_sys::bridge, but I'd
    // like to expose it without making websys_bridge public.
    target.dyn_ref::<web_sys::HtmlSelectElement>().expect("Unable to cast as a select element")
}

pub fn to_kbevent(event: &web_sys::Event ) -> &web_sys::KeyboardEvent {
    // This might be more appropriate for web_sys::bridge, but I'd
    // like to expose it without making websys_bridge public.
    event.dyn_ref::<web_sys::KeyboardEvent>().expect("Unable to cast as a keyboard event")
}

/// The entry point for the app
pub fn run<Ms, Mdl>(model: Mdl, update: fn(Ms, Mdl) -> Mdl,
          view: fn(Mdl) -> dom_types::El<Ms>, mount_point_id: &str)
    where Ms: Clone + Sized + 'static, Mdl: Clone + Sized + 'static
{
    let app = vdom::App::new(model.clone(), update, view, mount_point_id);

    // Our initial render. Can't initialize in new due to mailbox() requiring self.
    let mut topel_vdom = (app.data.view)(model);
    app.setup_vdom(&mut topel_vdom, 0, 0);

    vdom::attach_listeners(&mut topel_vdom, app.mailbox());

    // Attach all children: This is where our initial render occurs.
    websys_bridge::attach_els(&mut topel_vdom, &app.data.mount_point);

    app.data.main_el_vdom.replace(topel_vdom);

    // Allows panic messages to output to the browser console.error.
    panic::set_hook(Box::new(console_error_panic_hook::hook));
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


/// Introduce El into the global namespace for convenience (It will be repeated
/// often in the output type of components), and UpdateEl, which is required
/// for element-creation macros.
pub mod prelude {
    pub use crate::dom_types::{El, UpdateEl, simple_ev, input_ev, keyboard_ev, raw_ev};
}
