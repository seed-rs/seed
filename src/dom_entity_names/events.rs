/// Similar to tag population.
macro_rules! make_events {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($event_camel:ident => $event:expr),+ } => {

        /// The Ev enum restricts element-creation to only valid event names, as defined here:
        /// [https://developer.mozilla.org/en-US/docs/Web/Evs](https://developer.mozilla.org/en-US/docs/Web/Evs)
        #[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
        pub enum Ev {
            $(
                $event_camel,
            )+
            Custom(String)
        }

        impl Ev {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        Ev::$event_camel => $event,
                    ) +
                    Ev::Custom(val) => &val
                }
            }
        }

        impl From<&str> for Ev {
            fn from(event: &str) -> Self {
                match event {
                    $ (
                          $event => Ev::$event_camel,
                    ) +
                    _ => {
                        Ev::Custom(event.to_owned())
                    }
                }
            }
        }

        impl From<String> for Ev {
            fn from(event: String) -> Self {
                match event.as_ref(){
                    $ (
                          $event => Ev::$event_camel,
                    ) +
                    _ => {
                        Ev::Custom(event)
                    }
                }
            }
        }

        impl ToString for Ev {
            fn to_string( &self ) -> String {
                match self {
                    $ (
                        Ev::$ event_camel => $ event.into(),
                    ) +
                    Ev::Custom(val) => val.clone()
                }
            }
        }
    }
}

mod event_names;
pub use event_names::Ev;
