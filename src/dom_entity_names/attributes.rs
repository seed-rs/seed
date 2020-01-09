/// Similar to tag population.
macro_rules! make_attrs {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($attr_camel:ident => $attr:expr),+ } => {

        /// The At enum restricts element-creation to only valid event names, as defined here:
        /// [https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes)
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum At {
            $(
                $attr_camel,
            )+
            Custom(std::borrow::Cow<'static, str>)
        }

        impl At {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        At::$attr_camel => $attr,
                    ) +
                    At::Custom(attr) => &attr
                }
            }
        }

        impl std::fmt::Display for At {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl<T: Into<std::borrow::Cow<'static, str>>> From<T> for At {
            fn from(attr: T) -> Self {
                let attr = attr.into();
                match attr.as_ref() {
                    $(
                        $attr => At::$attr_camel,
                    ) +
                    _ => {
                        At::Custom(attr)
                    }
                }
            }
        }
    }
}

mod attribute_names;
pub use attribute_names::At;
