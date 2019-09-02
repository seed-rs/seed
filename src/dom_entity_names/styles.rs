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
            Custom(String)
        }

        impl St {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        St::$st_pascal_case => $st,
                    ) +
                    St::Custom(val) => &val
                }
            }
        }

        impl From<&str> for St {
            fn from(st: &str) -> Self {
                match st {
                    $ (
                          $st => St::$st_pascal_case,
                    ) +
                    _ => {
                        crate::error(&format!("Can't find this style: {}", st));
                        St::Custom(st.to_owned())
                    }
                }
            }
        }
        impl From<String> for St {
            fn from(st: String) -> Self {
                match st.as_ref() {
                    $ (
                          $st => St::$st_pascal_case,
                    ) +
                    _ => {
                        crate::error(&format!("Can't find this style: {}", st));
                        St::Custom(st)
                    }
                }
            }
        }

    }
}

mod style_names;
pub use style_names::St;
