//! Provide a wrapper for commonly-used, but verbose `web_sys` features.
//! This module is decoupled / independent.

use crate::events;
use std::fmt;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys;

pub type RequestAnimationFrameTime = f64;

pub struct RequestAnimationFrameHandle {
    request_id: i32,
    _closure: Closure<dyn FnMut(RequestAnimationFrameTime)>,
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

/// Convenience function to access the `web_sys` DOM document.
pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Can't find the window's document")
}

/// Convenience function to access the `web_sys` DOM body.
pub fn body() -> web_sys::HtmlElement {
    document().body().expect("Can't find the document's body")
}

/// Convenience function to access the `web_sys` history.
pub fn history() -> web_sys::History {
    window().history().expect("Can't find history")
}

/// Request the animation frame.
pub fn request_animation_frame(
    f: Closure<dyn FnMut(RequestAnimationFrameTime)>,
) -> RequestAnimationFrameHandle {
    let request_id = window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Problem requesting animation frame");

    RequestAnimationFrameHandle {
        request_id,
        _closure: f,
    }
}

/// Simplify getting the value of input elements; required due to the need to cast
/// from general nodes/elements to `HTML_Elements`.
pub fn get_value(target: &web_sys::EventTarget) -> Result<String, &'static str> {
    use web_sys::*;
    macro_rules! get {
        ($element:ty) => {
            get!($element, |_| Ok(()))
        };
        ($element:ty, $result_callback:expr) => {
            if let Some(input) = target.dyn_ref::<$element>() {
                return $result_callback(input).map(|_| input.value().to_string());
            }
        };
    }
    // List of elements
    // https://docs.rs/web-sys/0.3.25/web_sys/struct.HtmlMenuItemElement.html?search=value
    // They should be ordered by expected frequency of use

    get!(HtmlInputElement, |input: &HtmlInputElement| {
        // https://www.w3schools.com/tags/att_input_value.asp
        match input.type_().as_str() {
            "file" => Err(r#"The value attribute cannot be used with <input type="file">."#),
            _ => Ok(()),
        }
    });
    get!(HtmlTextAreaElement);
    get!(HtmlSelectElement);
    get!(HtmlProgressElement);
    get!(HtmlOptionElement);
    get!(HtmlButtonElement);
    get!(HtmlDataElement);
    get!(HtmlMeterElement);
    get!(HtmlLiElement);
    get!(HtmlOutputElement);
    get!(HtmlParamElement);

    Err("Can't use function `get_value` for given element.")
}

/// Similar to `get_value`.
pub fn set_value(target: &web_sys::EventTarget, value: &str) -> Result<(), &'static str> {
    use web_sys::*;
    macro_rules! set {
        ($element:ty) => {
            set!($element, |_| Ok(value))
        };
        ($element:ty, $value_result_callback:expr) => {
            if let Some(input) = target.dyn_ref::<$element>() {
                return $value_result_callback(input).map(|value| input.set_value(value));
            }
        };
    }
    // List of elements
    // https://docs.rs/web-sys/0.3.25/web_sys/struct.HtmlMenuItemElement.html?search=set_value
    // They should be ordered by expected frequency of use

    set!(HtmlInputElement, |input: &HtmlInputElement| {
        // https://www.w3schools.com/tags/att_input_value.asp
        match input.type_().as_str() {
            "file" => Err(r#"The value attribute cannot be used with <input type="file">."#),
            _ => Ok(value),
        }
    });
    set!(HtmlTextAreaElement);
    set!(HtmlSelectElement);
    set!(HtmlProgressElement, |_| value.parse().map_err(|_| {
        "Can't parse value to `f64` for `HtmlProgressElement`."
    }));
    set!(HtmlOptionElement);
    set!(HtmlButtonElement);
    set!(HtmlDataElement);
    set!(HtmlMeterElement, |_| value.parse().map_err(|_| {
        "Can't parse value to `f64` for `HtmlMeterElement`."
    }));
    set!(HtmlLiElement, |_| value.parse().map_err(|_| {
        "Can't parse value to `i32` for `HtmlLiElement`."
    }));
    set!(HtmlOutputElement);
    set!(HtmlParamElement);

    Err("Can't use function `set_value` for given element.")
}

/// Similar to `get_value`
#[allow(dead_code)]
pub fn get_checked(target: &web_sys::EventTarget) -> Result<bool, &'static str> {
    if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
        // https://www.w3schools.com/tags/att_input_checked.asp
        return match input.type_().as_str() {
            "file" => Err(r#"The checked attribute can be used with <input type="checkbox"> and <input type="radio">."#),
            _ => Ok(input.checked())
        };
    }
    if let Some(input) = target.dyn_ref::<web_sys::HtmlMenuItemElement>() {
        return Ok(input.checked());
    }
    Err("Only `HtmlInputElement` and `HtmlMenuItemElement` can be used in function `get_checked`.")
}

/// Similar to `set_value`.
#[allow(clippy::unit_arg)]
pub fn set_checked(target: &web_sys::EventTarget, value: bool) -> Result<(), &'static str> {
    if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
        // https://www.w3schools.com/tags/att_input_checked.asp
        return match input.type_().as_str() {
            "file" => Err(r#"The checked attribute can be used with <input type="checkbox"> and <input type="radio">."#),
            _ => Ok(input.set_checked(value))
        };
    }
    if let Some(input) = target.dyn_ref::<web_sys::HtmlMenuItemElement>() {
        return Ok(input.set_checked(value));
    }
    Err("Only `HtmlInputElement` and `HtmlMenuItemElement` can be used in function `set_checked`.")
}

// @TODO: Delete once `Closure::new` is stable
// https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html
/// Prevent repetition when wrapping closures.
pub trait ClosureNew<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new(inner: impl FnMut(T) + 'static) -> Closure<dyn FnMut(T)>
    where
        T: wasm_bindgen::convert::FromWasmAbi + 'static;
}
impl<T> ClosureNew<T> for Closure<T> {
    #[allow(clippy::new_ret_no_self)]
    fn new(inner: impl FnMut(T) + 'static) -> Closure<dyn FnMut(T)>
    where
        T: wasm_bindgen::convert::FromWasmAbi + 'static,
    {
        Closure::wrap(Box::new(inner))
    }
}

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
/// Consider to use [`App::update`](struct.App.html#method.update) if you have access to the [`App`](struct.App.html) instance.
pub fn update<Ms>(msg: Ms)
where
    Ms: Clone + 'static + serde::Serialize,
{
    let msg_as_js_value = wasm_bindgen::JsValue::from_serde(&msg)
        .expect("Error: TriggerUpdate - can't serialize given msg!");

    let mut custom_event_config = web_sys::CustomEventInit::new();
    custom_event_config.detail(&msg_as_js_value);

    let event = web_sys::CustomEvent::new_with_event_init_dict(
        events::UPDATE_TRIGGER_EVENT_ID,
        &custom_event_config,
    )
    .expect("Error: TriggerUpdate - create event failed!");

    window()
        .dispatch_event(&event)
        .expect("Error: TriggerUpdate - dispatch)event failed!");
}
