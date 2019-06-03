//! Provide a wrapper for commonly-used, but verbose web_sys features.
//! This module is decoupled / independent.

use crate::dom_types;
use std::fmt;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys;

pub type RequestAnimationFrameTime = f64;

pub struct RequestAnimationFrameHandle {
    request_id: i32,
    _closure: Closure<FnMut(RequestAnimationFrameTime)>,
}

impl Drop for RequestAnimationFrameHandle {
    fn drop(&mut self) {
        window()
            .cancel_animation_frame(self.request_id)
            .expect("Problem cancelling animation frame request")
    }
}

/// Convenience function to avoid repeating expect logic.
pub fn window() -> web_sys::Window {
    web_sys::window().expect("Can't find the global Window")
}

/// Convenience function to access the web_sys DOM document.
pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Can't find the window's document")
}

/// Convenience function to access the web_sys DOM body.
pub fn body() -> web_sys::HtmlElement {
    document().body().expect("Can't find the document's body")
}

/// Convenience function to access the web_sys history.
pub fn history() -> web_sys::History {
    window().history().expect("Can't find history")
}

/// Request the animation frame.
pub fn request_animation_frame(
    f: Closure<FnMut(RequestAnimationFrameTime)>
) -> RequestAnimationFrameHandle {
    let request_id =
        window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("Problem requesting animation frame");

    RequestAnimationFrameHandle {
        request_id,
        _closure: f,
    }
}

/// Simplify getting the value of input elements; required due to the need to cast
/// from general nodes/elements to HTML_Elements.
pub fn get_value(target: &web_sys::EventTarget) -> String {
    if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
        return input.value();
    }
    if let Some(input) = target.dyn_ref::<web_sys::HtmlTextAreaElement>() {
        return input.value();
    }
    if let Some(input) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
        return input.value();
    }
    "".into()
}

/// Similar to get_value.
pub fn set_value(target: &web_sys::EventTarget, value: &str) {
    if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
        input.set_value(value);
    }
    if let Some(input) = target.dyn_ref::<web_sys::HtmlTextAreaElement>() {
        input.set_value(value);
    }
    if let Some(input) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
        input.set_value(value);
    }
}

// todo: Unable to get this convenience function working
///// Prevent repetition when wrapping closures.
////pub fn make_closure(inner: impl FnMut(web_sys::Event)) -> Box<FnMut(web_sys::Event) + 'static> {
//pub fn make_closure<T>(inner: T) -> Closure<Box<T>>
//    where T: WasmClosure {
////    Closure::wrap(Box::new(inner)) as Box<FnMut(web_sys::Event) + 'static>
//    Closure::wrap(Box::new(inner))
//}

/// Convenience function for logging to the web browser's console.  See also
/// the log! macro, which is more flexible.
pub fn log<D: fmt::Debug>(text: D) {
    web_sys::console::log_1(&format!("{:#?}", &text).into());
}

/// Similar to log, but for errors.
pub fn error<D: fmt::Debug>(text: D) {
    web_sys::console::error_1(&format!("{:#?}", &text).into());
}

/// Trigger update function.
/// It requires Msg to be (De)serializable
/// and to register `trigger_update_handler` in `window_events`.
pub fn update<Ms>(msg: Ms)
    where
        Ms: Clone + 'static + serde::Serialize,
{
    let msg_as_js_value = wasm_bindgen::JsValue::from_serde(&msg)
        .expect("Error: TriggerUpdate - can't serialize given msg!");

    let mut custom_event_config = web_sys::CustomEventInit::new();
    custom_event_config.detail(&msg_as_js_value);

    let event = web_sys::CustomEvent::new_with_event_init_dict(
        dom_types::UPDATE_TRIGGER_EVENT_ID,
        &custom_event_config,
    )
        .expect("Error: TriggerUpdate - create event failed!");

    window()
        .dispatch_event(&event)
        .expect("Error: TriggerUpdate - dispatch)event failed!");
}
