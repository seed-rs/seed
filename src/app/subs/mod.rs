use crate::browser::Url;

// ------ UrlRequested sub ------

pub mod url_requested;
pub use url_requested::UrlRequested;

// ------ UrlChanged sub ------

/// Subscribe to url changes.
///
/// # Example
///
/// ```rust,ignore
///orders.subscribe(Msg::UrlChanged).notify(subs::UrlChanged(url));
///...
///update(... Msg::UrlChanged(subs::UrlChanged(url)) =>
/// ```
#[derive(Debug, Clone)]
pub struct UrlChanged(pub Url);
