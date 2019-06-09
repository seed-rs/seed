//! Raw bindings to the Javascript APIs we need, namely set(Timeout|Interval) and clear(Timeout|Interval).
//! Depending on how rustwasm/wasm-bindgen#1046 is resolved, we may be able to remove this at a later date.

use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setTimeout")]
    pub fn set_timeout(handler: &Function, timeout: i32) -> i32;

    #[wasm_bindgen(js_name = "clearTimeout")]
    pub fn clear_timeout(token: i32);

    #[wasm_bindgen(js_name = "setInterval")]
    pub fn set_interval(handler: &Function, timeout: i32) -> i32;

    #[wasm_bindgen(js_name = "clearInterval")]
    pub fn clear_interval(token: i32);
}
