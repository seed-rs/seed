//! See readme for details.

#![allow(unused_macros)]

pub mod dom_types;

#[macro_use]
pub mod dom_shortcuts;

//mod fetch;
pub mod vdom;
//mod example;

//pub use vdom::run;  // todo this may start working again in the future.


//// todos:
// todo router
// todo local storage
// todo vdom patching
// todo booleans/values in attrs/style.
// todo streamlined input handling for text etc
// todo maybe?? High-level css-grid and flex api?
// todo Async conflicts with events stepping on each other ?

// todo keyed elements??


/// A convenience function for logging to the web browser's console.
pub fn log(text: &str) {
    web_sys::console::log_1(&text.into());
}

/// Introduce Element-related types into the global namespace, which are
/// required to make the element/etc macros work.
pub mod prelude {
    pub use crate::dom_types::{El, Style, Attrs, Tag, Listener, UpdateEl};
}
