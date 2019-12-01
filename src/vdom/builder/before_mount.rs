use web_sys::Element;

use crate::{routing::Url, util};

pub trait MountPoint {
    fn element_getter(self) -> Box<dyn FnOnce() -> Element>;
}

impl MountPoint for &str {
    fn element_getter(self) -> Box<dyn FnOnce() -> Element> {
        let id = self.to_owned();
        Box::new(move || util::document().get_element_by_id(&id).unwrap_or_else(|| {
            panic!(
                "Can't find element with id={:?} - app cannot be mounted!\n\
                 (Id defaults to \"app\", or can be set with the .mount() method)",
                id
            )
        }))
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

pub struct BeforeMount {
    pub mount_point_getter: Box<dyn FnOnce() -> Element>,
    /// How to handle elements already present in the mount. Defaults to [`MountType::Append`]
    /// in the constructors.
    pub mount_type: MountType,
}

impl BeforeMount {
    pub fn new(mp: impl MountPoint + 'static) -> Self {
        Self {
            mount_point_getter: Box::new(mp.element_getter()),
            mount_type: MountType::default(),
        }
    }

    pub fn mount_point(self, new_mp: impl MountPoint + 'static) -> BeforeMount {
        BeforeMount {
            mount_point_getter: Box::new(new_mp.element_getter()),
            mount_type: self.mount_type,
        }
    }

    pub fn mount_type(mut self, new_mt: MountType) -> Self {
        self.mount_type = new_mt;
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

#[allow(clippy::module_name_repetitions)]
pub trait IntoBeforeMount {
    fn into_before_mount(self, init_url: Url) -> BeforeMount;
}

impl IntoBeforeMount for BeforeMount {
    fn into_before_mount(self, _: Url) -> BeforeMount {
        self
    }
}

impl<F> IntoBeforeMount for F
where
    F: FnOnce(Url) -> BeforeMount,
{
    fn into_before_mount(self, init_url: Url) -> BeforeMount {
        self(init_url)
    }
}

impl IntoBeforeMount for () {
    fn into_before_mount(self, _: Url) -> BeforeMount {
        BeforeMount::default()
    }
}
