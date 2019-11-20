use web_sys::Element;

use crate::{
    util,
    vdom::{
        alias::*,
        App,
    },
};

pub trait MountPoint {
    fn element(self) -> Element;
}

impl MountPoint for &str {
    fn element(self) -> Element {
        util::document().get_element_by_id(self).unwrap_or_else(|| {
            panic!(
                "Can't find element with id={:?} - app cannot be mounted!\n\
                 (Id defaults to \"app\", or can be set with the .mount() method)",
                self
            )
        })
    }
}

impl MountPoint for Element {
    fn element(self) -> Element {
        self
    }
}

impl MountPoint for web_sys::HtmlElement {
    fn element(self) -> Element {
        self.into()
    }
}

/// Describes the handling of elements already present in the mount element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MountType {
    /// Take control of previously existing elements in the mount. This does not make guarantees of
    /// elements added after the [`App`] has been mounted.
    ///
    /// Note that existing elements in the DOM will be recreated. This can be dangerous for script
    /// tags and other, similar tags.
    Takeover,
    /// Leave the previously existing elements in the mount alone. This does not make guarantees of
    /// elements added after the [`App`] has been mounted.
    Append,
}
