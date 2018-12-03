//! See readme for details.

#![allow(unused_macros)]

// todo temp>?? (For listener creation / handler
#![feature(impl_trait_in_bindings)]

pub mod dom_types;
#[macro_use]
pub mod dom_shortcuts;
mod vdom;
mod example;

// todo temp
mod mailbox;


// todo temp
pub use self::mailbox::Mailbox;
use std::borrow::Cow;

pub type S = Cow<'static, str>;


/// A convenience function for logging to the web browser's console.
pub fn log(text: &str) {
    web_sys::console::log_1(&text.into());
}

/// The basics, into the global namespace.
pub mod prelude {
    pub use crate::dom_types::{El, Style, Attrs, Tag, Event, Events, UpdateEl};
    pub use crate::vdom::run;
    pub use crate::log;
//    pub use crate::dom_types::{Element, Styles, Attrs, Tag};
//    pub use proc_macros::*;
}


