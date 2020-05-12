use super::{MountType, UrlHandling};

/// Used as a flexible wrapper for the init function.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    since = "0.5.0",
    note = "Part of old Init API. Use a combination of `BeforeMount` and `AfterMount` instead."
)]
pub struct Init<Mdl> {
    /// Initial model to be used when the app begins.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub model: Mdl,
    /// How to handle initial url routing. Defaults to [`UrlHandling::PassToRoutes`] in the
    /// constructors.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub url_handling: UrlHandling,
    /// How to handle elements already present in the mount. Defaults to [`MountType::Append`]
    /// in the constructors.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `BeforeMount` instead."
    )]
    pub mount_type: MountType,
}

impl<Mdl> Init<Mdl> {
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub const fn new(model: Mdl) -> Self {
        Self {
            model,
            url_handling: UrlHandling::PassToRoutes,
            mount_type: MountType::Append,
        }
    }
}
