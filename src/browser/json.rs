use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    Serde(JsValue),
    Parse(JsValue),
    Stringify(JsValue),
}

type Result<T> = std::result::Result<T, Error>;

mod swb;
pub use swb::*;
