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
            Custom(String),
            $(
                $tag_camel,
            )+
        }

        impl Tag {
            pub fn as_str(&self) -> &str {
                match self {
                    Tag::Custom(name) => &name,
                    $ (
                        Tag::$tag_camel => $tag,
                    ) +
                }
            }
        }

        impl<'a, T: Into<std::borrow::Cow<'a, str>>> From<T> for Tag {
            fn from(tag: T) -> Self {
                let tag: std::borrow::Cow<'a, str> = tag.into();
                match tag.as_ref() {
                    $ (
                          $tag => Tag::$tag_camel,
                    ) +
                    _ => {
                        Tag::Custom(tag.to_string())
                    }
                }
            }
        }
    }
}

mod tag_names;
pub use tag_names::Tag;
