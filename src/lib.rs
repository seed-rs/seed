//! See readme for details.

#![allow(unused_macros)]

pub mod dom_types;
pub mod fetch;
pub mod routing;
pub mod shortcuts;
pub mod storage;
mod util;
mod vdom;
mod websys_bridge;

use wasm_bindgen::{closure::Closure, JsCast};

// todo: Why does this work without importing web_sys??

//// todos:
// todo local storage
// todo High-level css-grid and flex api?
// todo Async conflicts with events stepping on each other ?
// todo keyed elements?

pub use crate::{
    dom_types::{Listener},
    fetch::{Method, Request, spawn_local},
    websys_bridge::{to_input, to_kbevent, to_mouse_event, to_select, to_textarea, to_html_el},
    routing::push_route,
    vdom::{App, run}
};

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

/// A high-level wrapper for web_sys::window.set_interval_with_callback_and_timeout_and_arguments_0:
/// https://rustwasm.github.io/wasm-bindgen/examples/closures.html
/// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html
pub fn set_interval(handler: Box<Fn()>, timeout: i32) {
    let callback = Closure::wrap(handler as Box<dyn Fn()>);
    let window = web_sys::window().expect("Can't find window");
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        timeout
    ).expect("Problem setting interval");
    callback.forget();
}

// todo: Perhaps put did_mount etc here so we call with seed:: instead of in prelude.
// todo or maybe not, for consistency with events.

/// Introduce El and Tag into the global namespace for convenience (El will be repeated
/// often in the output type of components), and UpdateEl, which is required
/// for element-creation macros, input event constructors, and the History struct.
/// Expose the wasm_bindgen prelude, and lifecycle hooks.
pub mod prelude {
    pub use std::collections::HashMap;

    pub use crate::{
        dom_types::{
            El, Tag, UpdateEl, simple_ev, input_ev, keyboard_ev, mouse_ev, raw_ev,
            did_mount, did_update, will_unmount
        },
        shortcuts::*,  // appears not to work.
    };

    pub use wasm_bindgen::prelude::*;
//    pub use wasm_bindgen_macro::wasm_bindgen;

}
