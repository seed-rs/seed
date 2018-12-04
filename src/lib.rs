//! See readme for details.

#![allow(unused_macros)]

// todo temp>?? (For listener creation / handler
#![feature(impl_trait_in_bindings)]

pub mod dom_types;
#[macro_use]
pub mod dom_shortcuts;
//mod fetch;
mod vdom;
mod example;

/// A convenience function for logging to the web browser's console.
pub fn log(text: &str) {
    web_sys::console::log_1(&text.into());
}

/// The basics, into the global namespace.  Note that some of these from dom_types are required
/// to be in the global namespace for the view-creation macros to work.
pub mod prelude {
    pub use crate::dom_types::{El, Style, Attrs, Tag, Event, Events, UpdateEl};
    pub use crate::vdom::run;
    pub use crate::log;
}


