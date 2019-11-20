use web_sys::Element;

use crate::{
    dom_types::View,
    orders::OrdersContainer,
    routing, util,
    vdom::{
        alias::*,
        App,
    },
};

/// Used for handling initial routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UrlHandling {
    PassToRoutes,
    None,
    // todo: Expand later, as-required
}

