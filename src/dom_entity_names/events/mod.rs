/// Similar to tag population.
macro_rules! make_events {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($event_camel:ident => $event:expr),+ } => {

        /// The Ev enum restricts element-creation to only valid event names, as defined here:
        /// [MDN reference Web/Events](https://developer.mozilla.org/en-US/docs/Web/Events)
        #[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
        pub enum Ev {
            $(
                $event_camel,
            )+
            Custom(std::borrow::Cow<'static, str>)
        }

        impl Ev {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        Ev::$event_camel => $event,
                    ) +
                    Ev::Custom(event) => &event
                }
            }
        }

        impl std::fmt::Display for Ev {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl<T: Into<std::borrow::Cow<'static, str>>> From<T> for Ev {
            fn from(event: T) -> Self {
                let event = event.into();
                match event.as_ref() {
                    $(
                        $event => Ev::$event_camel,
                    ) +
                    _ => {
                        Ev::Custom(event)
                    }
                }
            }
        }
    }
}

mod event_names;
pub use event_names::Ev;
