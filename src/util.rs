//use wasm_bindgen::{closure::{Closure, WasmClosure}, JsCast};
use wasm_bindgen::JsCast;
use web_sys;

/// Convenience function to avoid repeating expect logic.
pub fn window() -> web_sys::Window {
    web_sys::window().expect("Can't find the global Window")
}

/// Convenience function to access the web_sys DOM document.
pub fn document() -> web_sys::Document {
    window().document().expect("Can't find document")
}

/// Simplify getting the value of input elements; required due to the need to cast
/// from general nodes/elements to HTML__Elements.
pub fn input_value(target: &web_sys::EventTarget) -> String {
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

/// todo more DRY that could be addressed by Traits. ?
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

/// A convenience function for logging to the web browser's console.  See also
/// the log! macro, which is more flexible.
pub fn log<S: ToString>(text: S) {
    // ignore clippy about &S
    web_sys::console::log_1(&text.to_string().into());
}

/// Similar to log, but for errors.
pub fn error<S: ToString>(text: S) {
    // ignore clippy about &S
    web_sys::console::error_1(&text.to_string().into());
}
