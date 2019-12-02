use web_sys::Element;

use crate::util;

// ------ MountPoint ------

pub trait MountPoint {
    fn element_getter(self) -> Box<dyn FnOnce() -> Element>;
}

impl MountPoint for &str {
    fn element_getter(self) -> Box<dyn FnOnce() -> Element> {
        let id = self.to_owned();
        Box::new(move || {
            util::document().get_element_by_id(&id).unwrap_or_else(|| {
                panic!(
                    "Can't find element with id={:?} - app cannot be mounted!\n\
                     (Id defaults to \"app\", or can be set with the .mount() method)",
                    id
                )
            })
        })
    }
}

impl MountPoint for Element {
    fn element_getter(self) -> Box<dyn FnOnce() -> Element> {
        Box::new(|| self)
    }
}

impl MountPoint for web_sys::HtmlElement {
    fn element_getter(self) -> Box<dyn FnOnce() -> Element> {
        Box::new(|| self.into())
    }
}

// ------ MountType ------

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

impl Default for MountType {
    fn default() -> Self {
        Self::Append
    }
}

// ------ BeforeMount ------

pub struct BeforeMount {
    pub(crate) mount_point_getter: Box<dyn FnOnce() -> Element>,
    /// How to handle elements already present in the mount. Defaults to [`MountType::Append`]
    /// in the constructors.
    pub(crate) mount_type: MountType,
}

impl BeforeMount {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mount_point(mut self, mount_point: impl MountPoint + 'static) -> BeforeMount {
        self.mount_point_getter = Box::new(mount_point.element_getter());
        self
    }

    pub const fn mount_type(mut self, mount_type: MountType) -> Self {
        self.mount_type = mount_type;
        self
    }
}

impl Default for BeforeMount {
    fn default() -> Self {
        Self {
            mount_point_getter: "app".element_getter(),
            mount_type: MountType::default(),
        }
    }
}
