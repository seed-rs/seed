use crate::browser::util::document;
use web_sys::{Element, HtmlElement};

pub trait GetElement {
    /// Returns wrapped `web_sys::Element` or tries to get one from the DOM.
    ///
    /// # Errors
    ///
    /// Returns error if the element cannot be found.
    fn get_element(self) -> Result<Element, String>;
}

impl GetElement for &str {
    fn get_element(self) -> Result<Element, String> {
        document()
            .get_element_by_id(self)
            .ok_or_else(|| format!("cannot find element with given id: {}", self))
    }
}

impl GetElement for Element {
    fn get_element(self) -> Result<Element, String> {
        Ok(self)
    }
}

impl GetElement for HtmlElement {
    fn get_element(self) -> Result<Element, String> {
        Ok(self.into())
    }
}
