pub mod dom;
pub mod service;
pub mod url;
pub mod util;

#[cfg(feature = "routing")]
mod json;

pub use url::{Url, UrlSearch, DUMMY_BASE_URL};
