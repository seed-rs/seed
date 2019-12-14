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
            Custom(String)
        }

        impl At {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        At::$attr_camel => $attr,
                    ) +
                    At::Custom(val) => &val
                }
            }
        }

        impl From<&str> for At {
            fn from(attr: &str) -> Self {
                match attr {
                    $ (
                          $attr => At::$attr_camel,
                    ) +
                    _ => {
                        At::Custom(attr.to_owned())
                    }
                }
            }
        }
        impl From<String> for At {
            fn from(attr: String) -> Self {
                match attr.as_ref() {
                    $ (
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
