use seed::{prelude::*, *};
use std::fmt;

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(state: State) -> Node<Ms> {
    custom![
        Tag::from("checkbox-tristate"),
        attrs! {
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
    pub fn next(self) -> Self {
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
        write!(f, "{}", state)
    }
}
