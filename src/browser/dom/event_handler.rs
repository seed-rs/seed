//! This module contains code related to event handling; ie things that update the dom, related to
//! `web_sys::Event`

use super::super::util;
use crate::virtual_dom::{Ev, EventHandler};
use std::rc::Rc;
use wasm_bindgen::JsCast;

/// Create an event that passes a String of field text, for fast input handling.
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::missing_panics_doc)]
pub fn input_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(String) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    let handler = map_callback_return_to_option_ms!(
        dyn Fn(String) -> Option<Ms>,
        handler.clone(),
        "Handler can return only Msg, Option<Msg> or ()!",
        Rc
    );
    let handler = move |event: web_sys::Event| {
        let value = event
            .target()
            .as_ref()
            .ok_or("Can't get event target reference")
            .and_then(util::get_value)
            .map_err(crate::error)
            .unwrap_or_default();
        handler(value)
    };
    EventHandler::new(trigger, handler)
}

/// Create an event that passes a `web_sys::KeyboardEvent`, allowing easy access
/// to items like `key_code`() and key().
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::missing_panics_doc)]
pub fn keyboard_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::KeyboardEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    let handler = map_callback_return_to_option_ms!(
        dyn Fn(web_sys::KeyboardEvent) -> Option<Ms>,
        handler.clone(),
        "Handler can return only Msg, Option<Msg> or ()!",
        Rc
    );
    let handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, handler)
}

/// See `keyboard_ev`
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::missing_panics_doc)]
pub fn mouse_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::MouseEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    let handler = map_callback_return_to_option_ms!(
        dyn Fn(web_sys::MouseEvent) -> Option<Ms>,
        handler.clone(),
        "Handler can return only Msg, Option<Msg> or ()!",
        Rc
    );
    let handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::MouseEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, handler)
}

/// See `keyboard_ev`
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::missing_panics_doc)]
pub fn touch_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::TouchEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    let handler = map_callback_return_to_option_ms!(
        dyn Fn(web_sys::TouchEvent) -> Option<Ms>,
        handler.clone(),
        "Handler can return only Msg, Option<Msg> or ()!",
        Rc
    );
    let handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::TouchEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, handler)
}

/// See `keyboard_ev`
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::missing_panics_doc)]
pub fn drag_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::DragEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    let handler = map_callback_return_to_option_ms!(
        dyn Fn(web_sys::DragEvent) -> Option<Ms>,
        handler.clone(),
        "Handler can return only Msg, Option<Msg> or ()!",
        Rc
    );
    let handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::DragEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, handler)
}

/// See `keyboard_ev`
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::missing_panics_doc)]
pub fn pointer_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::PointerEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    let handler = map_callback_return_to_option_ms!(
        dyn Fn(web_sys::PointerEvent) -> Option<Ms>,
        handler.clone(),
        "Handler can return only Msg, Option<Msg> or ()!",
        Rc
    );
    let handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::PointerEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, handler)
}

/// See `keyboard_ev`
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::missing_panics_doc)]
pub fn wheel_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::WheelEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    let handler = map_callback_return_to_option_ms!(
        dyn Fn(web_sys::WheelEvent) -> Option<Ms>,
        handler.clone(),
        "Handler can return only Msg, Option<Msg> or ()!",
        Rc
    );
    let handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::WheelEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, handler)
}

/// Create an event that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling.
#[deprecated(since = "0.6.0", note = "Use `ev` instead.")]
pub fn raw_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::Event) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    ev(trigger, handler)
}

/// Create an event handler that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling.
///
/// Handler has to return `Msg`, `Option<Msg>` or `()`.
///
/// # Panics
///
/// Panics when the handler doesn't return `Msg` or `()`. (It will be changed to a compile-time error).
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::missing_panics_doc)]
pub fn ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::Event) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    let handler = map_callback_return_to_option_ms!(
        dyn Fn(web_sys::Event) -> Option<Ms>,
        handler.clone(),
        "Handler can return only Msg, Option<Msg> or ()!",
        Rc
    );
    #[allow(clippy::redundant_closure)]
    EventHandler::new(trigger, move |event| handler(event))
}

/// Create an event that passes no data, other than it occurred. Foregoes using a closure,
/// in favor of pointing to a message directly.
#[deprecated(since = "0.8.0", note = "Use `ev` instead.")]
pub fn simple_ev<Ms: Clone + 'static>(trigger: impl Into<Ev>, message: Ms) -> EventHandler<Ms> {
    let handler = || Some(message);
    let closure_handler = move |_| handler.clone()();
    EventHandler::new(trigger, closure_handler)
}
