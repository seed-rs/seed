use crate::browser::Url;
use std::{cell::Cell, rc::Rc};

pub type PreventDefault = bool;

#[derive(Copy, Clone)]
pub enum UrlRequestStatus {
    Unhandled,
    Handled(PreventDefault),
}

#[derive(Clone)]
pub struct UrlRequest(Rc<Cell<UrlRequestStatus>>);

impl Default for UrlRequest {
    fn default() -> Self {
        UrlRequest(Rc::new(Cell::new(UrlRequestStatus::Unhandled)))
    }
}

impl UrlRequest {
    pub fn unhandled(self) {
        self.0.set(UrlRequestStatus::Unhandled);
    }

    pub fn handled(self) {
        self.0.set(UrlRequestStatus::Handled(false));
    }

    pub fn handled_and_prevent_default(self) {
        self.0.set(UrlRequestStatus::Handled(true));
    }

    pub fn status(self) -> UrlRequestStatus {
        self.0.get()
    }
}

#[derive(Clone)]
pub struct UrlRequested(pub Url, pub UrlRequest);
