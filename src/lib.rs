//! See readme for details.

#![allow(unused_macros)]

pub mod dom_types;
#[macro_use]
pub mod dom_shortcuts;
mod vdom;
mod example;

// todo temp
mod mailbox;
mod subscription;
mod node;
mod element;
mod text;


// todo temp
//pub use self::app::{start, App, Instance};
pub use self::element::{h, s};
pub use self::element::{Element, KeyedElement, NonKeyedElement};
pub use self::mailbox::Mailbox;
pub use self::node::Node;
pub use self::subscription::{Subscription, Unsubscribe};
pub use self::text::Text;
use std::borrow::Cow;

pub type S = Cow<'static, str>;

pub fn select(selector: &str) -> Option<web_sys::Element> {
    web_sys::window()?
        .document()?
        .query_selector(selector)
        .ok()?
}


/// The basics, into the global namespace.
pub mod prelude {
    pub use crate::dom_types::{El, Style, Attrs, Tag, Event, Events, UpdateEl};
    pub use crate::vdom::run;
//    pub use crate::dom_types::{Element, Styles, Attrs, Tag};
//    pub use proc_macros::*;
}





