use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct Error(JsValue);

type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "routing")]
mod swb;
#[cfg(feature = "routing")]
pub use swb::*;
