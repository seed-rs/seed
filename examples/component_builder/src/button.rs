#![allow(dead_code)]

use seed::virtual_dom::IntoNodes;
use seed::{prelude::*, Style as Css, *};
use std::borrow::Cow;
use std::rc::Rc;
use web_sys::HtmlElement;

// ------ Button ------

pub struct Button<Ms: 'static> {
    title: Option<Cow<'static, str>>,
    style: Style,
    outline: bool,
    size: Size,
    block: bool,
    element: Element,
    attrs: Attrs,
    disabled: bool,
    on_clicks: Vec<Rc<dyn Fn() -> Ms>>,
    content: Vec<Node<Ms>>,
    el_ref: ElRef<HtmlElement>,
    css: Css,
}

impl<Ms> Button<Ms> {
    pub fn new(title: impl Into<Cow<'static, str>>) -> Self {
        Self::default().title(title)
    }

    pub fn title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    // --- style ---

    const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub const fn primary(self) -> Self {
        self.style(Style::Primary)
    }

    pub const fn secondary(self) -> Self {
        self.style(Style::Secondary)
    }

    // --- // ---

    pub const fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    // --- size ---

    const fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub const fn medium(self) -> Self {
        self.size(Size::Medium)
    }

    pub const fn large(self) -> Self {
        self.size(Size::Large)
    }

    // --- // ---

    pub const fn block(mut self) -> Self {
        self.block = true;
        self
    }

    // --- element ---

    #[allow(clippy::missing_const_for_fn)]
    fn element(mut self, element: Element) -> Self {
        self.element = element;
        self
    }

    pub fn a(self, href: impl Into<Cow<'static, str>>) -> Self {
        self.element(Element::A(Href(href.into())))
    }

    pub fn button(self) -> Self {
        self.element(Element::Button)
    }

    // --- // ---

    pub fn add_attrs(mut self, attrs: Attrs) -> Self {
        self.attrs.merge(attrs);
        self
    }

    pub fn add_css(mut self, css: Css) -> Self {
        self.css.merge(css);
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn add_on_click(mut self, on_click: impl FnOnce() -> Ms + Clone + 'static) -> Self {
        self.on_clicks.push(Rc::new(move || on_click.clone()()));
        self
    }

    pub fn content(mut self, content: impl IntoNodes<Ms>) -> Self {
        self.content = content.into_nodes();
        self
    }

    pub fn el_ref(mut self, el_ref: &ElRef<HtmlElement>) -> Self {
        self.el_ref = el_ref.clone();
        self
    }

    fn view(mut self) -> Node<Ms> {
        let tag = {
            match self.element {
                Element::A(_) => Tag::A,
                Element::Button => Tag::Button,
            }
        };

        let content = self.title.take().map(Node::new_text);

        let attrs = {
            let mut attrs = attrs! {};

            if self.disabled {
                attrs.add(At::from("aria-disabled"), true);
                attrs.add(At::TabIndex, -1);
            }

            match self.element {
                Element::A(href) => {
                    if not(self.disabled) {
                        attrs.add(At::Href, href.as_str());
                    }
                }
                Element::Button => {
                    if self.disabled {
                        attrs.add(At::Disabled, AtValue::None);
                    }
                }
            }
            attrs
        };

        let css = {
            let mut css = style! {
                St::TextDecoration => "none",
            };

            let color = match self.style {
                Style::Primary => "blue",
                Style::Secondary => "gray",
            };

            if self.outline {
                css.merge(style! {
                    St::Color => color,
                    St::BackgroundColor => "transparent",
                    St::Border => format!("{} {} {}", px(2), "solid", color),
                });
            } else {
                css.merge(style! { St::Color => "white", St::BackgroundColor => color });
            };

            match self.size {
                Size::Medium => {
                    css.merge(style! {St::Padding => format!("{} {}", rem(0.2), rem(0.7))});
                }
                Size::Large => {
                    css.merge(style!{St::Padding => format!("{} {}", rem(0.5), rem(1)), St::FontSize => rem(1.25)});
                }
            }

            if self.block {
                css.merge(style! {St::Display => "block"});
            }

            if self.disabled {
                css.merge(style! {St::Opacity => 0.5});
            } else {
                css.merge(style! {St::Cursor => "pointer"});
            }

            css
        };

        let mut button = custom![
            tag,
            el_ref(&self.el_ref),
            css,
            self.css,
            attrs,
            self.attrs,
            content,
            self.content,
        ];

        if !self.disabled {
            for on_click in self.on_clicks {
                button.add_event_handler(ev(Ev::Click, move |_| on_click()));
            }
        }

        button
    }
}

impl<Ms> Default for Button<Ms> {
    fn default() -> Self {
        Self {
            title: None,
            style: Style::default(),
            outline: false,
            size: Size::default(),
            block: false,
            element: Element::default(),
            attrs: Attrs::empty(),
            disabled: false,
            on_clicks: Vec::new(),
            content: Vec::new(),
            el_ref: ElRef::default(),
            css: Css::empty(),
        }
    }
}

// It allows us to use `Button` directly in element macros without calling `view` explicitly.
// E.g. `div![Button::new("My button")]`
impl<Ms> UpdateEl<Ms> for Button<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}

// ------ Style ------

enum Style {
    Primary,
    Secondary,
}

impl Default for Style {
    fn default() -> Self {
        Self::Primary
    }
}

// ------ Size ------

enum Size {
    Medium,
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Self::Medium
    }
}

// ------ Element ------

enum Element {
    A(Href),
    Button,
}

impl Default for Element {
    fn default() -> Self {
        Self::Button
    }
}

// ------ Href ------

pub struct Href(Cow<'static, str>);

impl Href {
    fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}
