use wasm_bindgen::JsCast;

/// Convenience function used in event handling: Convert an event target
/// to an input element; eg so you can take its value.
pub fn to_input(target: &web_sys::EventTarget) -> &web_sys::HtmlInputElement {
    target
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("Unable to cast as an input element")
}

/// See [`to_input`](fn.to_input.html)
pub fn to_textarea(target: &web_sys::EventTarget) -> &web_sys::HtmlTextAreaElement {
    target
        .dyn_ref::<web_sys::HtmlTextAreaElement>()
        .expect("Unable to cast as a textarea element")
}

/// See [`to_input`](fn.to_input.html)
pub fn to_select(target: &web_sys::EventTarget) -> &web_sys::HtmlSelectElement {
    target
        .dyn_ref::<web_sys::HtmlSelectElement>()
        .expect("Unable to cast as a select element")
}

/// See [`to_input`](fn.to_input.html)
pub fn to_html_el(target: &web_sys::EventTarget) -> &web_sys::HtmlElement {
    target
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("Unable to cast as an HTML element")
}

/// Convert a `web_sys::Event` to a `web_sys::KeyboardEvent`. Useful for extracting
/// info like which key has been pressed, which is not available with normal Events.
pub fn to_kbevent(event: &web_sys::Event) -> &web_sys::KeyboardEvent {
    event
        .dyn_ref::<web_sys::KeyboardEvent>()
        .expect("Unable to cast as a keyboard event")
}

/// See `to_kbevent`
pub fn to_mouse_event(event: &web_sys::Event) -> &web_sys::MouseEvent {
    event
        .dyn_ref::<web_sys::MouseEvent>()
        .expect("Unable to cast as a mouse event")
}
