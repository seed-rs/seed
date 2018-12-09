//! See readme for details.

#![allow(unused_macros)]

pub mod dom_types;

#[macro_use]
pub mod dom_shortcuts;

//mod fetch;
pub mod vdom;

//pub use vdom::run;  // todo this may start working again in the future.


//// todos:
// todo router
// todo local storage
// todo vdom patching
// todo streamlined input handling for text etc
// todo maybe?? High-level css-grid and flex api?
// todo Async conflicts with events stepping on each other ?

// todo keyed elements??


/// A convenience function for logging to the web browser's console.
pub fn log(text: &str) {
    web_sys::console::log_1(&text.into());
}

/// Introduce El into the global namespace for convenience (It will be repeated
/// often in the output type of components), and UpdateEl, which is required
/// for our element-creation macros.
pub mod prelude {
    pub use crate::dom_types::{El, UpdateEl};
}
