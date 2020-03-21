//! This module contains code related to event handling; ie things that update the dom, related to
//! `web_sys::Event`

use super::super::util;
use crate::virtual_dom::{Ev, EventHandler};
use std::any::{Any, TypeId};
use wasm_bindgen::JsCast;

/// Create an event that passes a String of field text, for fast input handling.
#[allow(clippy::shadow_unrelated)]
// @TODO remove `'static`s once `optin_builtin_traits`
// @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
pub fn input_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(String) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
    let t_type = TypeId::of::<MsU>();
    if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
        panic!("Handler can return only Msg or ()!");
    }
    let handler = move |text| {
        let output = &mut Some(handler.clone()(text)) as &mut dyn Any;
        output.downcast_mut::<Option<Ms>>().and_then(Option::take)
    };

    let closure_handler = move |event: web_sys::Event| {
        let value = event
            .target()
            .as_ref()
            .ok_or("Can't get event target reference")
            .and_then(util::get_value)
            .map_err(crate::error)
            .unwrap_or_default();
        handler(value)
    };
    EventHandler::new(trigger, closure_handler)
}

/// Create an event that passes a `web_sys::KeyboardEvent`, allowing easy access
/// to items like `key_code`() and key().
#[allow(clippy::shadow_unrelated)]
// @TODO remove `'static`s once `optin_builtin_traits`
// @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
pub fn keyboard_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::KeyboardEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
    let t_type = TypeId::of::<MsU>();
    if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
        panic!("Handler can return only Msg or ()!");
    }
    let handler = move |event| {
        let output = &mut Some(handler.clone()(event)) as &mut dyn Any;
        output.downcast_mut::<Option<Ms>>().and_then(Option::take)
    };

    let closure_handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, closure_handler)
}

/// See `keyboard_ev`
#[allow(clippy::shadow_unrelated)]
// @TODO remove `'static`s once `optin_builtin_traits`
// @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
pub fn mouse_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::MouseEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
    let t_type = TypeId::of::<MsU>();
    if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
        panic!("Handler can return only Msg or ()!");
    }
    let handler = move |event| {
        let output = &mut Some(handler.clone()(event)) as &mut dyn Any;
        output.downcast_mut::<Option<Ms>>().and_then(Option::take)
    };

    let closure_handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::MouseEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, closure_handler)
}

/// See `keyboard_ev`
#[allow(clippy::shadow_unrelated)]
// @TODO remove `'static`s once `optin_builtin_traits`
// @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
pub fn pointer_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::PointerEvent) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
    let t_type = TypeId::of::<MsU>();
    if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
        panic!("Handler can return only Msg or ()!");
    }
    let handler = move |event| {
        let output = &mut Some(handler.clone()(event)) as &mut dyn Any;
        output.downcast_mut::<Option<Ms>>().and_then(Option::take)
    };

    let closure_handler = move |event: web_sys::Event| {
        handler(event.dyn_ref::<web_sys::PointerEvent>().unwrap().clone())
    };
    EventHandler::new(trigger, closure_handler)
}

/// Create an event that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling.
#[deprecated(since = "0.6.0", note = "Use `ev` instead.")]
// @TODO remove `'static`s once `optin_builtin_traits`
// @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
pub fn raw_ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::Event) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    ev(trigger, handler)
}

/// Create an event handler that accepts a closure, and passes a `web_sys::Event`, allowing full control of
/// event-handling.
///
/// Handler has to return `Msg` or `()`.
///
/// # Panics
///
/// Panics when handler doesn't return `Msg` or `()`. (It will be changed to a compile-time error).
#[allow(clippy::shadow_unrelated)]
// @TODO remove `'static`s once `optin_builtin_traits`
// @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
pub fn ev<Ms: 'static, MsU: 'static>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(web_sys::Event) -> MsU + 'static + Clone,
) -> EventHandler<Ms> {
    // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
    let t_type = TypeId::of::<MsU>();
    if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
        panic!("Handler can return only Msg or ()!");
    }
    let handler = move |event| {
        let output = &mut Some(handler.clone()(event)) as &mut dyn Any;
        output.downcast_mut::<Option<Ms>>().and_then(Option::take)
    };

    let closure_handler = move |event: web_sys::Event| handler(event);
    EventHandler::new(trigger, closure_handler)
}

/// Create an event that passes no data, other than it occurred. Foregoes using a closure,
/// in favor of pointing to a message directly.
pub fn simple_ev<Ms: Clone + 'static>(trigger: impl Into<Ev>, message: Ms) -> EventHandler<Ms> {
    let handler = || Some(message);
    let closure_handler = move |_| handler.clone()();
    EventHandler::new(trigger, closure_handler)
}
