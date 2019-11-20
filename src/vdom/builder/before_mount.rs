use web_sys::Element;

use crate::{routing::Url, util};

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

impl MountPoint for () {
    fn element(self) -> Element {
        "app".element()
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BeforeMount<MP: MountPoint> {
    pub mount_point: MP,
    pub mount_type: MountType,
}

impl<MP: MountPoint> BeforeMount<MP> {
    pub fn new(mp: MP) -> Self {
        Self {
            mount_point: mp,
            mount_type: MountType::default(),
        }
    }

    pub fn mount_point<NewMP: MountPoint>(self, new_mp: NewMP) -> BeforeMount<NewMP> {
        BeforeMount {
            mount_point: new_mp,
            mount_type: self.mount_type,
        }
    }

    pub fn mount_type(mut self, new_mt: MountType) -> Self {
        self.mount_type = new_mt;
        self
    }
}

pub trait Into {
    type MP: MountPoint;
    fn into_before_mount(self, init_url: Url) -> BeforeMount<Self::MP>;
}

impl<MP: MountPoint, F> Into for F
where
    F: FnOnce(Url) -> BeforeMount<MP>,
{
    type MP = MP;
    fn into_before_mount(self, init_url: Url) -> BeforeMount<MP> {
        self(init_url)
    }
}

impl Into for () {
    type MP = ();
    fn into_before_mount(self, _: Url) -> BeforeMount<Self::MP> {
        BeforeMount::default()
    }
}
