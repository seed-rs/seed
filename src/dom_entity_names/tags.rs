// Populate tags using a macro, to reduce code repetition.
// The tag enum primarily exists to ensure only valid elements are allowed.
// We leave out non-body tags like html, meta, title, and body.
macro_rules! make_tags {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($tag_camel:ident => $tag:expr),+ } => {

        /// The Tag enum restricts element-creation to only valid tags, as defined here:
        /// [https://developer.mozilla.org/en-US/docs/Web/HTML/Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
        #[derive(Clone, Debug, PartialEq)]
        pub enum Tag {
            $(
                $tag_camel,
            )+
            Custom(std::borrow::Cow<'static, str>)
        }

        impl Tag {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        Tag::$tag_camel => $tag,
                    ) +
                    Tag::Custom(tag) => &tag
                }
            }
        }

        impl std::fmt::Display for Tag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl<T: Into<std::borrow::Cow<'static, str>>> From<T> for Tag {
            fn from(tag: T) -> Self {
                let tag = tag.into();
                match tag.as_ref() {
                    $(
                        $tag => Tag::$tag_camel,
                    ) +
                    _ => {
                        Tag::Custom(tag)
                    }
                }
            }
        }
    }
}

mod tag_names;
pub use tag_names::Tag;
