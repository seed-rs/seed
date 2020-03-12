use crate::browser::Url;

// ------ UrlRequested sub ------

pub mod url_requested;
pub use url_requested::UrlRequested;

// ------ UrlChanged sub ------

/// Subscribe to url changes.
///
/// # Example
///
/// ```rust,no_run
///orders.subscribe(Msg::UrlChanged);
///...
///update(... Msg::UrlChanged(subs::UrlChanged(url)) =>
/// ```
#[derive(Clone)]
pub struct UrlChanged(pub Url);
