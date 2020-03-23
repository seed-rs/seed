use super::{AtValue, CSSValue, EventHandler, St};
use crate::app::MessageMapper;
use std::borrow::Cow;

pub mod el;
pub mod into_nodes;
pub mod text;

pub use el::El;
pub use into_nodes::IntoNodes;
pub use text::Text;

/// A component in our virtual DOM.
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Node)
/// [`web_sys` reference](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html)
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Node<Ms> {
    Element(El<Ms>),
    //    Svg(El<Ms>),  // May be best to handle using namespace field on El
    Text(Text),
    Empty,
}

// @TODO remove custom impl once https://github.com/rust-lang/rust/issues/26925 is fixed
impl<Ms> Clone for Node<Ms> {
    fn clone(&self) -> Self {
        match self {
            Self::Element(element) => Self::Element(element.clone()),
            Self::Text(text) => Self::Text(text.clone()),
            Self::Empty => Self::Empty,
        }
    }
}

// Element methods
impl<Ms> Node<Ms> {
    /// See `El::from_markdown`
    pub fn from_markdown(markdown: &str) -> Vec<Node<Ms>> {
        El::from_markdown(markdown)
    }

    /// See `El::from_html`
    pub fn from_html(html: &str) -> Vec<Node<Ms>> {
        El::from_html(html)
    }

    /// See `El::add_child`
    pub fn add_child(&mut self, node: Node<Ms>) -> &mut Self {
        if let Node::Element(el) = self {
            el.add_child(node);
        }
        self
    }

    /// See `El::add_attr`
    pub fn add_attr(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        val: impl Into<AtValue>,
    ) -> &mut Self {
        if let Node::Element(el) = self {
            el.add_attr(key, val);
        }
        self
    }

    /// See `El::add_class`
    pub fn add_class(&mut self, name: impl Into<Cow<'static, str>>) -> &mut Self {
        if let Node::Element(el) = self {
            el.add_class(name);
        }
        self
    }

    /// See `El::add_style`
    pub fn add_style(&mut self, key: impl Into<St>, val: impl Into<CSSValue>) -> &mut Self {
        if let Node::Element(el) = self {
            el.add_style(key, val);
        }
        self
    }

    /// See `El::add_event_handler`
    pub fn add_event_handler(&mut self, event_handler: EventHandler<Ms>) -> &mut Self {
        if let Node::Element(el) = self {
            el.add_event_handler(event_handler);
        }
        self
    }

    /// See `El::add_text`
    pub fn add_text(&mut self, text: impl Into<Cow<'static, str>>) -> &mut Self {
        if let Node::Element(el) = self {
            el.add_text(text);
        }
        self
    }

    /// See `El::replace_text`
    pub fn replace_text(&mut self, text: impl Into<Cow<'static, str>>) -> &mut Self {
        if let Node::Element(el) = self {
            el.replace_text(text);
        }
        self
    }

    /// See `El::get_text`
    pub fn get_text(&self) -> String {
        match self {
            Node::Element(el) => el.get_text(),
            Node::Text(text) => text.text.to_string(),
            _ => "".to_string(),
        }
    }
}

// Convenience methods
impl<Ms> Node<Ms> {
    pub fn new_text(text: impl Into<Cow<'static, str>>) -> Self {
        Node::Text(Text::new(text))
    }

    pub fn is_text(&self) -> bool {
        if let Node::Text(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_el(&self) -> bool {
        if let Node::Element(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_empty(&self) -> bool {
        if let Node::Empty = self {
            true
        } else {
            false
        }
    }

    pub fn text(&self) -> Option<&Text> {
        if let Node::Text(t) = self {
            Some(t)
        } else {
            None
        }
    }
    pub fn el(&self) -> Option<&El<Ms>> {
        if let Node::Element(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

// Backing node manipulation
impl<Ms> Node<Ms> {
    pub fn strip_ws_nodes_from_self_and_children(&mut self) {
        match self {
            Node::Text(t) => t.strip_ws_node(),
            Node::Element(e) => e.strip_ws_nodes_from_self_and_children(),
            Node::Empty => (),
        }
    }

    #[cfg(debug_assertions)]
    pub fn warn_about_script_tags(&self) {
        if let Node::Element(e) = self {
            e.warn_about_script_tags();
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Node<Ms> {
    type SelfWithOtherMs = Node<OtherMs>;
    /// See note on impl for El
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Node<OtherMs> {
        match self {
            Node::Element(el) => Node::Element(el.map_msg(f)),
            Node::Text(text) => Node::Text(text),
            Node::Empty => Node::Empty,
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Vec<Node<Ms>> {
    type SelfWithOtherMs = Vec<Node<OtherMs>>;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Vec<Node<OtherMs>> {
        self.into_iter()
            .map(|node| node.map_msg(f.clone()))
            .collect()
    }
}
