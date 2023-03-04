pub mod dom;
pub mod service;
pub mod url;
pub mod util;

#[cfg(any(feature = "serde-wasm-bindgen", feature = "serde-json"))]
mod json;

pub use url::{Url, UrlSearch, DUMMY_BASE_URL};
