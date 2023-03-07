/// Similar to tag population.
macro_rules! make_styles {
    // Create shortcut macros for any style; populate these functions in the submodule.
    { $($st_pascal_case:ident => $st:expr),+ } => {

        /// The St enum restricts element-creation to only valid styles.
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum St {
            $(
                $st_pascal_case,
            )+
            Custom(std::borrow::Cow<'static, str>)
        }

        impl St {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        St::$st_pascal_case => $st,
                    ) +
                    St::Custom(style) => &style
                }
            }
        }

        impl std::fmt::Display for St {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl<T: Into<std::borrow::Cow<'static, str>>> From<T> for St {
            fn from(style: T) -> Self {
                let style = style.into();
                match style.as_ref() {
                    $(
                        $st => St::$st_pascal_case,
                    ) +
                    _ => {
                        St::Custom(style)
                    }
                }
            }
        }
    }
}

mod style_names;
pub use style_names::St;
