use super::super::{
    At, AtValue, Attrs, CSSValue, EventHandler, EventHandlerManager, Node, SharedNodeWs, St, Style,
    Tag, Text,
};
use crate::app::MessageMapper;
use crate::browser::{
    dom::{virtual_dom_bridge, Namespace},
    util,
};
use std::borrow::Cow;

/// A component in our virtual DOM.
///
/// _Note:_ `Listener`s in `El`'s `event_handler_manager` are not cloned, but recreated during VDOM patching.
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// [`web_sys` reference](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html)
#[derive(Debug)] // todo: Custom debug implementation where children are on new lines and indented.
pub struct El<Ms> {
    // Ms is a message type, as in part of TEA.
    // We call this 'El' instead of 'Element' for brevity, and to prevent
    // confusion with web_sys::Element.
    pub tag: Tag,
    pub attrs: Attrs,
    pub style: Style,
    pub event_handler_manager: EventHandlerManager<Ms>,
    pub children: Vec<Node<Ms>>,
    pub namespace: Option<Namespace>,
    /// The actual DOM element/node.
    pub node_ws: Option<web_sys::Node>,
    pub refs: Vec<SharedNodeWs>,
}

// @TODO remove custom impl once https://github.com/rust-lang/rust/issues/26925 is fixed
impl<Ms> Clone for El<Ms> {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            attrs: self.attrs.clone(),
            style: self.style.clone(),
            event_handler_manager: self.event_handler_manager.clone(),
            children: self.children.clone(),
            namespace: self.namespace.clone(),
            node_ws: self.node_ws.clone(),
            refs: self.refs.clone(),
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for El<Ms> {
    type SelfWithOtherMs = El<OtherMs>;
    /// Maps an element's message to have another message.
    ///
    /// This allows third party components to integrate with your application without
    /// having to know about your Msg type beforehand.
    ///
    /// # Note
    /// There is an overhead to calling this versus keeping all messages under one type.
    /// The deeper the nested structure of children, the more time this will take to run.
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> El<OtherMs> {
        El {
            tag: self.tag,
            attrs: self.attrs,
            style: self.style,
            children: self
                .children
                .into_iter()
                .map(|c| c.map_msg(f.clone()))
                .collect(),
            node_ws: self.node_ws,
            namespace: self.namespace,
            event_handler_manager: self.event_handler_manager.map_msg(f),
            refs: self.refs,
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Vec<El<Ms>> {
    type SelfWithOtherMs = Vec<El<OtherMs>>;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Vec<El<OtherMs>> {
        self.into_iter().map(|el| el.map_msg(f.clone())).collect()
    }
}

impl<Ms> El<Ms> {
    /// Create an empty element, specifying only the tag
    pub fn empty(tag: Tag) -> Self {
        Self {
            tag,
            attrs: Attrs::empty(),
            style: Style::empty(),
            event_handler_manager: EventHandlerManager::new(),
            children: Vec::new(),
            namespace: None,
            node_ws: None,
            refs: Vec::new(),
        }
    }

    /// Create an empty SVG element, specifying only the tag
    pub fn empty_svg(tag: Tag) -> Self {
        let mut el = El::empty(tag);
        el.namespace = Some(Namespace::Svg);
        el
    }

    // todo: Return El instead of Node here? (Same with from_html)
    /// Create elements from a markdown string.
    /// _Note:_ All additional markdown [extensions](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/struct.Options.html) enabled.
    pub fn from_markdown(markdown: &str) -> Vec<Node<Ms>> {
        let options = pulldown_cmark::Options::all();

        let parser = pulldown_cmark::Parser::new_ext(markdown, options);
        let mut html_text = String::new();
        pulldown_cmark::html::push_html(&mut html_text, parser);

        Self::from_html(&html_text)
    }

    /// Create elements from an HTML string.
    pub fn from_html(html: &str) -> Vec<Node<Ms>> {
        // Create a web_sys::Element, with our HTML wrapped in a (arbitrary) span tag.
        // We allow web_sys to parse into a DOM tree, then analyze the tree to create our vdom
        // element.
        let wrapper = util::document()
            .create_element("placeholder")
            .expect("Problem creating web-sys element");
        wrapper.set_inner_html(html);

        let mut result = Vec::new();
        let children = wrapper.child_nodes();
        for i in 0..children.length() {
            let child = children
                .get(i)
                .expect("Can't find child in raw html element.");

            if let Some(child_vdom) = virtual_dom_bridge::node_from_ws(&child) {
                result.push(child_vdom)
            }
        }
        result
    }

    /// Add a new child to the element
    pub fn add_child(&mut self, element: Node<Ms>) -> &mut Self {
        self.children.push(element);
        self
    }

    /// Add an attribute (eg class, or href)
    pub fn add_attr(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        val: impl Into<AtValue>,
    ) -> &mut Self {
        self.attrs.vals.insert(At::from(key), val.into());
        self
    }

    /// Add a class. May be cleaner than `add_attr`
    pub fn add_class(&mut self, name: impl Into<Cow<'static, str>>) -> &mut Self {
        let name = name.into();
        self.attrs
            .vals
            .entry(At::Class)
            .and_modify(|at_value| match at_value {
                AtValue::Some(v) => {
                    if !v.is_empty() {
                        *v += " ";
                    }
                    *v += name.as_ref();
                }
                _ => *at_value = AtValue::Some(name.clone().into_owned()),
            })
            .or_insert(AtValue::Some(name.into_owned()));
        self
    }

    /// Add a new style (eg display, or height),
    pub fn add_style(&mut self, key: impl Into<St>, val: impl Into<CSSValue>) -> &mut Self {
        self.style.vals.insert(key.into(), val.into());
        self
    }

    /// Add a new event handler.
    pub fn add_event_handler(&mut self, event_handler: EventHandler<Ms>) -> &mut Self {
        self.event_handler_manager
            .add_event_handlers(vec![event_handler]);
        self
    }

    /// Add a text node to the element. (ie between the HTML tags).
    pub fn add_text(&mut self, text: impl Into<Cow<'static, str>>) -> &mut Self {
        self.children.push(Node::Text(Text::new(text)));
        self
    }

    /// Replace the element's text.
    /// Removes all text nodes from element, then adds the new one.
    pub fn replace_text(&mut self, text: impl Into<Cow<'static, str>>) -> &mut Self {
        self.children.retain(|node| !node.is_text());
        self.children.push(Node::new_text(text));
        self
    }

    // Pull text from child text nodes
    pub fn get_text(&self) -> String {
        self.children
            .iter()
            .filter_map(|child| match child {
                Node::Text(text_node) => Some(text_node.text.to_string()),
                _ => None,
            })
            .collect()
    }

    #[cfg(debug_assertions)]
    /// Warn user about potential bugs when having scripts and `Takeover` mount type.
    pub fn warn_about_script_tags(&self) {
        let script_found = match &self.tag {
            Tag::Script => true,
            Tag::Custom(tag) if tag == "script" => true,
            _ => false,
        };
        if script_found {
            error!("Script tag found inside mount point! \
                    Please check https://docs.rs/seed/latest/seed/app/builder/struct.Builder.html#examples");
        }

        for child in &self.children {
            child.warn_about_script_tags();
        }
    }

    /// Remove websys nodes.
    pub fn strip_ws_nodes_from_self_and_children(&mut self) {
        self.node_ws.take();
        for child in &mut self.children {
            child.strip_ws_nodes_from_self_and_children();
        }
    }

    /// Is it a custom element?
    pub fn is_custom(&self) -> bool {
        matches!(self.tag, Tag::Custom(_))
    }
}
