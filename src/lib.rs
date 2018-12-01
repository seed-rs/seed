//! See readme for details.

//#![feature(proc_macro)]
//#![feature(proc_macro_hygiene)]  //todo ?


#![allow(unused_macros)]

pub mod dom_types;
#[macro_use]
pub mod dom_shortcuts;
//mod scheduler;
mod vdom;
mod example;


/// The basics, into the global namespace.
pub mod prelude {
    pub use crate::dom_types::{El, Style, Attrs, Tag, Event, Events, UpdateEl};
    pub use crate::vdom::App;
//    pub use crate::dom_types::{Element, Styles, Attrs, Tag};
//    pub use proc_macros::*;
}





