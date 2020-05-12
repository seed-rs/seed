use web_sys::Element;

// ------ MountPoint ------

pub trait MountPoint {
    fn element_getter(self) -> Box<dyn FnOnce() -> Element>;
}

// ------ MountType ------

/// Describes the handling of elements already present in the mount element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MountType {
    /// Take control of previously existing elements in the mount. This does not make guarantees of
    /// elements added after the [`crate::app::App`] has been mounted.
    ///
    /// Note that existing elements in the DOM will be recreated. This can be dangerous for script
    /// tags and other, similar tags.
    Takeover,
    /// Leave the previously existing elements in the mount alone. This does not make guarantees of
    /// elements added after the [`crate::app::App`] has been mounted.
    Append,
}

impl Default for MountType {
    fn default() -> Self {
        Self::Append
    }
}
