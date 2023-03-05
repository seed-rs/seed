use seed::{prelude::*, *};
use std::fmt;

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static, F>(name: &str, label: &str, state: State, on_click: F) -> Node<Ms>
where
    F: FnOnce(String) -> Ms + Clone + 'static,
{
    let handler = move |event: web_sys::Event| {
        let event = event.dyn_ref::<web_sys::CustomEvent>().unwrap().clone();
        on_click(event.detail().as_string().unwrap())
    };
    custom![
        Tag::from("checkbox-tristate"),
        ev(Ev::from("cluck"), handler),
        attrs! {
            At::from("name") => name
            At::from("label") => label
            At::from("state") => state
        },
    ]
}

// ------ State ------

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Unchecked,
    Indeterminate,
    Checked,
}

impl State {
    pub const fn next(self) -> Self {
        match self {
            Self::Unchecked => Self::Indeterminate,
            Self::Indeterminate => Self::Checked,
            Self::Checked => Self::Unchecked,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Unchecked
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            Self::Unchecked => "unchecked",
            Self::Indeterminate => "indeterminate",
            Self::Checked => "checked",
        };
        write!(f, "{state}")
    }
}
