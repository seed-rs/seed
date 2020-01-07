//! This module contains code related to event handling; ie things that update the dom, related to
//! `web_sys::Event`

use super::super::util;
use crate::virtual_dom::{Ev, EventHandler};
use wasm_bindgen::JsCast;

/// Create an event that passes a String of field text, for fast input handling.
pub fn input_ev<Ms>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(String) -> Ms + 'static + Clone,
) -> EventHandler<Ms> {
    let closure_handler = move |event: web_sys::Event| {
        let value = event
            .target()
            .as_ref()
            .ok_or("Can't get event target reference")
            .and_then(util::get_value)
            .map_err(crate::error)
            .unwrap_or_default();

        (handler.clone())(value)
    };
    EventHandler::new(trigger, closure_handler)
}

/// Create an event that passes a `web_sys::KeyboardEvent`, allowing easy access
/// to items like `key_code`() and key().
pub fn keyboard_ev<Ms>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::KeyboardEvent) -> Ms + 'static + Clone,
) -> EventHandler<Ms> {
    let closure_handler = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, closure_handler)
}

/// See `keyboard_ev`
pub fn mouse_ev<Ms>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::MouseEvent) -> Ms + 'static + Clone,
) -> EventHandler<Ms> {
    let closure_handler = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::MouseEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, closure_handler)
}

/// See `keyboard_ev`
pub fn pointer_ev<Ms>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::PointerEvent) -> Ms + 'static + Clone,
) -> EventHandler<Ms> {
    let closure_handler = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::PointerEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, closure_handler)
}

/// Create an event that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling.
#[deprecated(since = "0.6.0", note = "Use `ev` instead.")]
pub fn raw_ev<Ms>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::Event) -> Ms + 'static + Clone,
) -> EventHandler<Ms> {
    ev(trigger, handler)
}

/// Create an event handler that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling.
pub fn ev<Ms>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::Event) -> Ms + 'static + Clone,
) -> EventHandler<Ms> {
    let closure_handler = move |event: web_sys::Event| (handler.clone())(event);
    EventHandler::new(trigger, closure_handler)
}

/// Create an event that passes no data, other than it occurred. Foregoes using a closure,
/// in favor of pointing to a message directly.
pub fn simple_ev<Ms: Clone + 'static>(trigger: impl Into<Ev>, message: Ms) -> EventHandler<Ms> {
    let handler = || message;
    let closure_handler = move |_| handler.clone()();
    EventHandler::new(trigger, closure_handler)
}
