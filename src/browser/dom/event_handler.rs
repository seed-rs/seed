//! This module contains code related to event handling; ie things that update the dom, related to
//! `web_sys::Event`

use super::super::util;
use crate::virtual_dom::Listener;
use wasm_bindgen::JsCast;

/// Create an event that passes a String of field text, for fast input handling.
pub fn input_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(String) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        let value = event
            .target()
            .as_ref()
            .ok_or("Can't get event target reference")
            .and_then(util::get_value)
            .map_err(crate::error)
            .unwrap_or_default();

        (handler.clone())(value)
    };

    Listener::new(&trigger.to_string(), Some(closure))
}

/// Create an event that passes a `web_sys::KeyboardEvent`, allowing easy access
/// to items like `key_code`() and key().
pub fn keyboard_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(web_sys::KeyboardEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone())
    };
    Listener::new(&trigger.to_string(), Some(closure))
}

/// See `keyboard_ev`
pub fn mouse_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(web_sys::MouseEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::MouseEvent>().unwrap().clone())
    };
    Listener::new(&trigger.to_string(), Some(closure))
}

/// See `keyboard_ev`
pub fn pointer_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(web_sys::PointerEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::PointerEvent>().unwrap().clone())
    };
    Listener::new(&trigger.to_string(), Some(closure))
}

/// Create an event that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling
pub fn raw_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(web_sys::Event) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| (handler.clone())(event);
    Listener::new(&trigger.to_string(), Some(closure))
}

/// Create an event that passes no data, other than it occurred. Foregoes using a closure,
/// in favor of pointing to a message directly.
pub fn simple_ev<Ms: Clone, T>(trigger: T, message: Ms) -> Listener<Ms>
where
    Ms: 'static,
    T: ToString + Copy,
{
    let handler = || message;
    let closure = move |_| handler.clone()();
    Listener::new(&trigger.to_string(), Some(closure))
}
