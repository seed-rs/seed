//! Provide a wrapper for commonly-used, but verbose `web_sys` features.
//! This module is decoupled / independent.

// @TODO refactor (ideally once `Unsized` and `Specialization` are stable)

use std::borrow::Cow;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub use gloo_utils::{document, history, window};

#[deprecated(
    since = "0.8.0",
    note = "see [`request_animation_frame`](fn.request_animation_frame.html)"
)]
pub type RequestAnimationFrameTime = f64;

#[must_use]
#[deprecated(
    since = "0.8.0",
    note = "see [`request_animation_frame`](fn.request_animation_frame.html)"
)]
pub struct RequestAnimationFrameHandle {
    request_id: i32,
    _closure: Closure<dyn FnMut(RequestAnimationFrameTime)>,
}

impl Drop for RequestAnimationFrameHandle {
    fn drop(&mut self) {
        window()
            .cancel_animation_frame(self.request_id)
            .expect("Problem cancelling animation frame request");
    }
}

/// Convenience function to access the `web_sys` DOM body.
pub fn body() -> web_sys::HtmlElement {
    document().body().expect("Can't find the document's body")
}

/// Convenience function to access the `web_sys::HtmlDocument`.
pub fn html_document() -> web_sys::HtmlDocument {
    wasm_bindgen::JsValue::from(document()).unchecked_into::<web_sys::HtmlDocument>()
}
/// Convenience function to access the `web_sys::HtmlCanvasElement`.
/// /// _Note:_ Returns `None` if there is no element with the given `id` or the element isn't `HtmlCanvasElement`.
pub fn canvas(id: &str) -> Option<web_sys::HtmlCanvasElement> {
    document()
        .get_element_by_id(id)
        .and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok())
}

/// Convenience function to access the `web_sys::CanvasRenderingContext2d`.
pub fn canvas_context_2d(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("Problem getting canvas context")
        .expect("The canvas context is empty")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Problem casting as web_sys::CanvasRenderingContext2d")
}

/// Convenience function to get all cookies from the current `HtmlDocument`
/// _Note:_ Returns `None` if parsing cookies fails or there are no cookies.
pub fn cookies() -> Option<cookie::CookieJar> {
    let cookies_str = html_document().cookie().ok()?;
    let mut jar = cookie::CookieJar::new();

    for cookie_str in cookies_str.split(';') {
        let cookie = cookie::Cookie::parse_encoded(cookie_str).ok()?;
        jar.add(cookie.into_owned());
    }

    let jar_is_empty = jar.iter().next().is_none();
    if jar_is_empty {
        None
    } else {
        Some(jar)
    }
}

#[deprecated(
    since = "0.8.0",
    note = "use [`Orders::after_next_render`](../../app/orders/trait.Orders.html#method.after_next_render) instead"
)]
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
///
/// # Errors
///
/// Will return error if it's not possible to call `get_value` for given `target`.
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

#[allow(clippy::missing_errors_doc)]
/// Similar to `get_value`.
pub fn set_value(target: &web_sys::EventTarget, value: &str) -> Result<(), Cow<'static, str>> {
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

    if let Some(input) = target.dyn_ref::<HtmlInputElement>() {
        return set_html_input_element_value(input, value);
    }
    set!(HtmlTextAreaElement);
    set!(HtmlSelectElement);
    set!(HtmlProgressElement, |_| value.parse().map_err(|error| {
        Cow::from(format!(
            "Can't parse value to `f64` for `HtmlProgressElement`. Error: {:?}",
            error
        ))
    }));
    set!(HtmlOptionElement);
    set!(HtmlButtonElement);
    set!(HtmlDataElement);
    set!(HtmlMeterElement, |_| value.parse().map_err(|error| {
        Cow::from(format!(
            "Can't parse value to `f64` for `HtmlMeterElement`. Error: {:?}",
            error
        ))
    }));
    set!(HtmlLiElement, |_| value.parse().map_err(|error| {
        Cow::from(format!(
            "Can't parse value to `i32` for `HtmlLiElement`. Error: {:?}",
            error
        ))
    }));
    set!(HtmlOutputElement);
    set!(HtmlParamElement);

    Err(Cow::from(
        "Can't use function `set_value` for given element.",
    ))
}

fn set_html_input_element_value(
    input: &web_sys::HtmlInputElement,
    value: &str,
) -> Result<(), Cow<'static, str>> {
    // Don't update if value hasn't changed
    if value == input.value() {
        return Ok(());
    }

    // In some cases we need to set selection manually because
    // otherwise the cursor would jump at the end on some platforms.

    // `selectionStart` and `selectionEnd`
    // - "If this element is an input element, and selectionStart does not apply to this element, return null."
    //   - https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#dom-textarea/input-selectionstart
    // - => return values if the element type is:
    //   -  `text`, `search`, `url`, `tel`, `password` and probably also `week`, `month`
    //   - https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement
    //   - https://html.spec.whatwg.org/multipage/input.html#do-not-apply
    let selection_update_required = match input.type_().as_str() {
        // https://www.w3schools.com/tags/att_input_value.asp
        "file" => {
            return Err(Cow::from(
                r#"The value attribute cannot be used with <input type="file">."#,
            ))
        }
        "text" | "password" | "search" | "tel" | "url" | "week" | "month" => true,
        _ => false,
    };

    // We don't want to set selection in inactive input because
    // that input would "steal" focus from the active element on some platforms.
    if selection_update_required && is_active(input) {
        let selection_start = input
            .selection_start()
            .expect("get `HtmlInputElement` selection start");
        let selection_end = input
            .selection_end()
            .expect("get `HtmlInputElement` selection end");

        input.set_value(value);

        input
            .set_selection_start(selection_start)
            .expect("set `HtmlInputElement` selection start");
        input
            .set_selection_end(selection_end)
            .expect("set `HtmlInputElement` selection end");
    } else {
        input.set_value(value);
    }

    Ok(())
}

/// Return true if passed element is active.
fn is_active(element: &web_sys::Element) -> bool {
    document().active_element().as_ref() == Some(element)
}

#[allow(clippy::missing_errors_doc)]
/// Similar to `get_value`
#[allow(dead_code)]
pub fn get_checked(target: &web_sys::EventTarget) -> Result<bool, Cow<str>> {
    if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
        // https://www.w3schools.com/tags/att_input_checked.asp
        return match input.type_().as_str() {
            "file" => Err(Cow::from(
                r#"The checked attribute can be used with <input type="checkbox"> and <input type="radio">."#,
            )),
            _ => Ok(input.checked()),
        };
    }
    if let Some(input) = target.dyn_ref::<web_sys::HtmlMenuItemElement>() {
        return Ok(input.checked());
    }
    Err(Cow::from(
        "Only `HtmlInputElement` and `HtmlMenuItemElement` can be used in function `get_checked`.",
    ))
}

#[allow(clippy::missing_errors_doc)]
/// Similar to `set_value`.
#[allow(clippy::unit_arg)]
pub fn set_checked(target: &web_sys::EventTarget, value: bool) -> Result<(), Cow<str>> {
    if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
        // https://www.w3schools.com/tags/att_input_checked.asp
        return match input.type_().as_str() {
            "file" => Err(Cow::from(
                r#"The checked attribute can be used with <input type="checkbox"> and <input type="radio">."#,
            )),
            _ => Ok(input.set_checked(value)),
        };
    }
    if let Some(input) = target.dyn_ref::<web_sys::HtmlMenuItemElement>() {
        return Ok(input.set_checked(value));
    }
    Err(Cow::from(
        "Only `HtmlInputElement` and `HtmlMenuItemElement` can be used in function `set_checked`.",
    ))
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
#[cfg(use_nightly)]
pub fn log<T>(object: T) -> T {
    web_sys::console::log_1(&format!("{:#?}", dbg::WrapDebug(&object)).into());
    object
}

/// Convenience function for logging to the web browser's console.  See also
/// the log! macro, which is more flexible.
#[cfg(not(use_nightly))]
pub fn log<T: std::fmt::Debug>(object: T) -> T {
    web_sys::console::log_1(&format!("{:#?}", &object).into());
    object
}

/// Similar to log, but for errors.
#[cfg(use_nightly)]
pub fn error<T>(object: T) -> T {
    web_sys::console::error_1(&format!("{:#?}", dbg::WrapDebug(&object)).into());
    object
}

/// Similar to log, but for errors.
#[cfg(not(use_nightly))]
pub fn error<T: std::fmt::Debug>(object: T) -> T {
    web_sys::console::error_1(&format!("{:#?}", &object).into());
    object
}
