use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    Serde(JsValue),
    Parse(JsValue),
    Stringify(JsValue),
}

type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "swb")]
mod swb;
#[cfg(feature = "swb")]
pub use swb::*;

#[cfg(all(not(feature = "swb"), feature = "serde-json"))]
mod serde_json;
#[cfg(all(not(feature = "swb"), feature = "serde-json"))]
pub use self::serde_json::*;
