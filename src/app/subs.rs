use crate::browser::Url;

// ------ UrlRequested sub ------

pub mod url_requested;
pub use url_requested::UrlRequested;

// ------ UrlChanged sub ------

#[derive(Clone)]
pub struct UrlChanged(pub Url);
