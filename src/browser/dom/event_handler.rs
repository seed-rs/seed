//! This module contains code related to event handling; ie things that update the dom, related to
//! `web_sys::Event`

use super::super::util;
use crate::virtual_dom::{Category, Listener};
use serde::de::DeserializeOwned;
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

    Listener::new(
        &trigger.to_string(),
        Some(closure),
        Some(Category::Input),
        None,
    )
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
    Listener::new(
        &trigger.to_string(),
        Some(closure),
        Some(Category::Keyboard),
        None,
    )
}

/// See `keyboard_ev`
pub fn mouse_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(web_sys::MouseEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::MouseEvent>().unwrap().clone())
    };
    Listener::new(
        &trigger.to_string(),
        Some(closure),
        Some(Category::Mouse),
        None,
    )
}

/// See `keyboard_ev`
pub fn pointer_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(web_sys::PointerEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::PointerEvent>().unwrap().clone())
    };
    Listener::new(
        &trigger.to_string(),
        Some(closure),
        Some(Category::Pointer),
        None,
    )
}

/// Create an event that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling
pub fn raw_ev<Ms, T: ToString + Copy>(
    trigger: T,
    handler: impl FnOnce(web_sys::Event) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| (handler.clone())(event);
    Listener::new(
        &trigger.to_string(),
        Some(closure),
        Some(Category::Raw),
        None,
    )
}

/// Create an event that passes no data, other than it occurred. Foregoes using a closure,
/// in favor of pointing to a message directly.
pub fn simple_ev<Ms: Clone, T>(trigger: T, message: Ms) -> Listener<Ms>
where
    Ms: 'static,
    T: ToString + Copy,
{
    let msg_closure = message.clone();
    let handler = || msg_closure;
    let closure = move |_| handler.clone()();
    Listener::new(
        &trigger.to_string(),
        Some(closure),
        Some(Category::Simple),
        Some(message),
    )
}

#[deprecated]
pub const UPDATE_TRIGGER_EVENT_ID: &str = "triggerupdate";

/// Create an event that passes a `web_sys::CustomEvent`, allowing easy access
/// to detail() and then trigger update
#[deprecated]
pub fn trigger_update_ev<Ms>(
    handler: impl FnOnce(web_sys::CustomEvent) -> Ms + 'static + Clone,
) -> Listener<Ms> {
    let closure = move |event: web_sys::Event| {
        (handler.clone())(event.dyn_ref::<web_sys::CustomEvent>().unwrap().clone())
    };
    Listener::new(
        UPDATE_TRIGGER_EVENT_ID,
        Some(closure),
        Some(Category::Custom),
        None,
    )
}

///// Update app state directly, ie not from a Listener/event.
//pub fn update<Ms>() -> Listener<Ms> {
//    let closure = move |event: web_sys::Event| handler(event);
//    Listener::new(&trigger.to_string(), Some(Box::new(closure)))
//}

/// Trigger update function from outside of App
#[deprecated]
pub fn trigger_update_handler<Ms: DeserializeOwned>() -> Listener<Ms> {
    trigger_update_ev(|ev| {
        ev.detail()
            .into_serde()
            .expect("trigger_update_handler: Deserialization failed!")
    })
}
