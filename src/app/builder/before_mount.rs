use crate::browser::util;
use web_sys::Element;

// ------ MountPoint ------

pub struct UndefinedMountPoint;

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
    /// How to handle elements already present in the mount.
    /// Defaults to `MountType::Append` in the constructors.
    pub(crate) mount_type: MountType,
}

impl BeforeMount {
    /// Creates a new `BeforeMount` instance. It's the alias for `BeforeMount::default`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Choose the element where the application will be mounted.
    /// The default one is the element with `id` = "app".
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// // argument is `&str`
    /// mount_point("another_id")
    ///
    /// // argument is `HTMLElement`
    ///
    /// // NOTE: Be careful with mounting into body!
    /// // If you render directly into document.body, you risk collisions
    /// // with scripts that do something with it (e.g. Google Font Loader or
    /// // third party browser extensions) which produce very weird and hard
    /// // to debug errors in production.
    /// // (from https://github.com/facebook/create-react-app/issues/1568)
    ///
    /// mount_point(seed::body())
    ///
    /// // argument is `Element`
    /// mount_point(seed::body().querySelector("section").unwrap().unwrap())
    /// ```
    pub fn mount_point(mut self, mount_point: impl MountPoint + 'static) -> BeforeMount {
        self.mount_point_getter = Box::new(mount_point.element_getter());
        self
    }

    /// How to handle elements already present in the mount point. Defaults to `MountType::Append`.
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
