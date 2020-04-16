use crate::browser::Url;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use web_sys::Event;

pub type PreventDefault = bool;

// ------ UrlRequested sub ------

/// Subscribe to url requests. Requests are fired on `<a>` link click.
///
/// _Note:_ `orders.notify(subs::UrlRequested::new(url))` simulates link click.
///
/// # Example
///
/// ```rust,no_run
///orders.subscribe(Msg::UrlRequested);
///...
///update(... Msg::UrlRequested(subs::UrlRequested(url, url_request))) =>
/// ```
/// See `UrlRequest` for more info.
#[derive(Debug, Clone)]
pub struct UrlRequested(pub Url, pub UrlRequest);

impl UrlRequested {
    pub fn new(url: Url) -> Self {
        Self(url, UrlRequest::default())
    }
}

// --- UrlRequestStatus ---

#[derive(Debug, Copy, Clone)]
pub enum UrlRequestStatus {
    Unhandled,
    Handled(PreventDefault),
}

impl Default for UrlRequestStatus {
    fn default() -> Self {
        Self::Unhandled
    }
}

// --- UrlRequest ---

#[derive(Debug, Clone)]
pub struct UrlRequest {
    pub(crate) status: Rc<Cell<UrlRequestStatus>>,
    pub(crate) event: Rc<RefCell<Option<Event>>>,
}

impl UrlRequest {
    pub(crate) fn new(status: UrlRequestStatus, event: Option<Event>) -> Self {
        Self {
            status: Rc::new(Cell::new(status)),
            event: Rc::new(RefCell::new(event)),
        }
    }
}

impl Default for UrlRequest {
    fn default() -> Self {
        Self {
            status: Rc::new(Cell::new(UrlRequestStatus::default())),
            event: Rc::new(RefCell::new(None)),
        }
    }
}

impl UrlRequest {
    /// Flag the url request as unhandled.
    /// - Seed prevents page refresh, pushes the route and fires `UrlChanged` notification.
    /// - It's the default behaviour.
    pub fn unhandled(self) {
        self.status.set(UrlRequestStatus::Unhandled);
    }

    /// Flag the url request as handled.
    /// - Seed doesn't intercept or modify the click event and doesn't fire `UrlChanged` notification.
    pub fn handled(self) {
        self.status.set(UrlRequestStatus::Handled(false));
    }

    /// Flag the url request as handled and prevent page refresh.
    /// - It's almost the same like `handled()` method, but Seed calls `prevent_default` on the click event.
    pub fn handled_and_prevent_refresh(self) {
        self.status.set(UrlRequestStatus::Handled(true));
    }

    pub fn status(&self) -> UrlRequestStatus {
        self.status.get()
    }
}
