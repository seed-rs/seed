#![allow(dead_code)]

use seed::{prelude::*, *};
use std::borrow::Cow;
use std::rc::Rc;

pub struct Button<S> {
    pub label: S,
}

impl<S: Into<Cow<'static, str>>> Button<S> {
    pub fn into_component<Ms>(self) -> Component<Ms> {
        Component {
            label: self.label.into(),
            outlined: false,
            disabled: false,
            on_clicks: Vec::new(),
        }
    }
}

pub struct Component<Ms: 'static> {
    label: Cow<'static, str>,
    outlined: bool,
    disabled: bool,
    on_clicks: Vec<Rc<dyn Fn() -> Ms>>,
}

impl<Ms> Component<Ms> {
    pub const fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, on_click: impl FnOnce() -> Ms + Clone + 'static) -> Self {
        self.on_clicks.push(Rc::new(move || on_click.clone()()));
        self
    }

    pub fn into_node(self) -> Node<Ms> {
        let attrs = {
            let mut attrs = attrs! {};

            if self.disabled {
                attrs.add(At::from("aria-disabled"), true);
                attrs.add(At::TabIndex, -1);
                attrs.add(At::Disabled, AtValue::None);
            }

            attrs
        };

        let css = {
            let color = "teal";

            let mut css = style! {
                St::TextDecoration => "none",
            };

            if self.outlined {
                css.merge(style! {
                    St::Color => color,
                    St::BackgroundColor => "transparent",
                    St::Border => format!("{} {} {}", px(2), "solid", color),
                });
            } else {
                css.merge(style! { St::Color => "white", St::BackgroundColor => color });
            };

            if self.disabled {
                css.merge(style! {St::Opacity => 0.5});
            } else {
                css.merge(style! {St::Cursor => "pointer"});
            }

            css
        };

        let mut button = button![css, attrs, self.label];

        if !self.disabled {
            for on_click in self.on_clicks {
                button.add_event_handler(ev(Ev::Click, move |_| on_click()));
            }
        }

        button
    }
}
