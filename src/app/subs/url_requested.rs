use crate::browser::Url;
use std::{cell::Cell, rc::Rc};

pub type PreventDefault = bool;

// ------ UrlRequested sub ------

/// Subscribe to url requests. Requests are fired on a link click.
///
/// # Example
///
/// ```rust,no_run
///orders.subscribe(Msg::UrlRequested);
///...
///update(... Msg::UrlRequested(subs::UrlRequested(url, url_request))) =>
/// ```
/// See `UrlRequest` for more info.
#[derive(Clone)]
pub struct UrlRequested(pub Url, pub UrlRequest);

// --- UrlRequestStatus ---

#[derive(Copy, Clone)]
pub enum UrlRequestStatus {
    Unhandled,
    Handled(PreventDefault),
}

// --- UrlRequest ---

#[derive(Clone)]
pub struct UrlRequest(Rc<Cell<UrlRequestStatus>>);

impl Default for UrlRequest {
    fn default() -> Self {
        UrlRequest(Rc::new(Cell::new(UrlRequestStatus::Unhandled)))
    }
}

impl UrlRequest {
    /// Flag the url request as unhandled.
    /// - Seed prevents page refresh, pushes the route and fires `UrlChanged` notification.
    /// - It's the default behaviour.
    pub fn unhandled(self) {
        self.0.set(UrlRequestStatus::Unhandled);
    }

    /// Flag the url request as handled.
    /// - Seed doesn't intercept or modify the click event and doesn't fire `UrlChanged` notification.
    pub fn handled(self) {
        self.0.set(UrlRequestStatus::Handled(false));
    }

    /// Flag the url request as handled and prevent page refresh.
    /// - It's almost the same like `handled()` method, but Seed calls `prevent_default` on the click event.
    pub fn handled_and_prevent_refresh(self) {
        self.0.set(UrlRequestStatus::Handled(true));
    }

    pub fn status(self) -> UrlRequestStatus {
        self.0.get()
    }
}
