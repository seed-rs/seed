//! This file contains interactions with `web_sys`.

use super::Namespace;
use crate::virtual_dom::{At, AtValue, Attrs, El, Mailbox, Node, Style, Text};
use std::borrow::Cow;
use std::cmp::Ordering;
use wasm_bindgen::JsCast;
use web_sys::Document;

/// Convenience function to reduce repetition
fn set_style(el_ws: &web_sys::Node, style: &Style) {
    el_ws
        .dyn_ref::<web_sys::Element>()
        .expect("Problem casting Node as Element while setting style")
        .set_attribute("style", &style.to_string())
        .expect("Problem setting style");
}

pub(crate) fn assign_ws_nodes_to_el<Ms>(document: &Document, el: &mut El<Ms>) {
    let node_ws = make_websys_el(el, document);
    for ref_ in &mut el.refs {
        ref_.set(node_ws.clone());
    }
    el.node_ws = Some(node_ws);
    for child in &mut el.children {
        assign_ws_nodes(document, child);
    }
}
pub(crate) fn assign_ws_nodes_to_text(document: &Document, text: &mut Text) {
    text.node_ws = Some(
        document
            .create_text_node(&text.text)
            .dyn_into::<web_sys::Node>()
            .expect("Problem casting Text as Node."),
    );
}
/// Recursively create `web_sys::Node`s, and place them in the vdom Nodes' fields.
pub(crate) fn assign_ws_nodes<Ms>(document: &Document, node: &mut Node<Ms>) {
    match node {
        Node::Element(el) => assign_ws_nodes_to_el(document, el),
        Node::Text(text) => assign_ws_nodes_to_text(document, text),
        Node::Empty | Node::NoChange => (),
    }
}

fn node_to_element(el_ws: &web_sys::Node) -> Result<&web_sys::Element, Cow<str>> {
    if el_ws.node_type() == web_sys::Node::ELEMENT_NODE {
        el_ws
            .dyn_ref::<web_sys::Element>()
            .ok_or_else(|| Cow::from("Problem casting Node as Element"))
    } else {
        Err(Cow::from("Node isn't Element!"))
    }
}

fn set_attr_value(el_ws: &web_sys::Node, at: &At, at_value: &AtValue) {
    match at_value {
        AtValue::Some(value) => {
            node_to_element(el_ws)
                .and_then(|element| {
                    element.set_attribute(at.as_str(), value).map_err(|error| {
                        Cow::from(format!("Problem setting an attribute: {:?}", error))
                    })
                })
                .unwrap_or_else(|err| {
                    crate::error(err);
                });
        }
        AtValue::None => {
            node_to_element(el_ws)
                .and_then(|element| {
                    element.set_attribute(at.as_str(), "").map_err(|error| {
                        Cow::from(format!("Problem setting an attribute: {:?}", error))
                    })
                })
                .unwrap_or_else(|err| {
                    crate::error(err);
                });
        }
        AtValue::Ignored => {
            node_to_element(el_ws)
                .and_then(|element| {
                    element.remove_attribute(at.as_str()).map_err(|error| {
                        Cow::from(format!("Problem removing an attribute: {:?}", error))
                    })
                })
                .unwrap_or_else(|err| {
                    crate::error(err);
                });
        }
    }
}

/// Create and return a `web_sys` Element from our virtual-dom `El`. The `web_sys`
/// Element is a close analog to JS/DOM elements.
///
/// # References
/// * [`web_sys` Element](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html)
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
/// * See also: [`web_sys` Node](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html)
pub(crate) fn make_websys_el<Ms>(el: &mut El<Ms>, document: &web_sys::Document) -> web_sys::Node {
    let tag = el.tag.as_str();

    let el_ws = match el.namespace {
        Some(ref ns) => document
            .create_element_ns(Some(ns.as_str()), tag)
            .expect("Problem creating web-sys element with namespace"),
        None => document
            .create_element(tag)
            .expect("Problem creating web-sys element"),
    };

    fix_attrs_order(&mut el.attrs);
    for (at, attr_value) in &el.attrs.vals {
        set_attr_value(&el_ws, at, attr_value);
    }
    if let Some(ns) = &el.namespace {
        el_ws
            .dyn_ref::<web_sys::Element>()
            .expect("Problem casting Node as Element while setting an attribute")
            .set_attribute("xmlns", ns.as_str())
            .expect("Problem setting xlmns attribute");
    }

    // Style is just an attribute in the actual Dom, but is handled specially in our vdom;
    // merge the different parts of style here.
    if el.style.vals.keys().len() > 0 {
        set_style(&el_ws, &el.style);
    }

    el_ws.into()
}

/// Similar to `attach_el_and_children`, but for text nodes
pub fn attach_text_node(text: &mut Text, parent: &web_sys::Node) {
    let node_ws = text.node_ws.take().expect("Missing websys node for Text");
    parent
        .append_child(&node_ws)
        .expect("Problem appending text node");
    text.node_ws.replace(node_ws);
}

/// Similar to `attach_el_and_children`, but without attaching the elemnt. Useful for
/// patching, where we want to insert the element at a specific place.
pub fn attach_children<Ms>(el: &mut El<Ms>, mailbox: &Mailbox<Ms>) {
    let el_ws = el
        .node_ws
        .as_ref()
        .expect("Missing websys el in attach_children");
    // appending the its children to the el_ws
    for child in &mut el.children {
        match child {
            // Raise the active level once per recursion.
            Node::Element(child_el) => attach_el_and_children(child_el, el_ws, mailbox),
            Node::Text(child_text) => attach_text_node(child_text, el_ws),
            Node::Empty | Node::NoChange => (),
        }
    }
}

/// Attaches the element, and all children, recursively. Only run this when creating a fresh vdom node, since
/// it performs a rerender of the el and all children; eg a potentially-expensive op.
/// This is where rendering occurs.
pub fn attach_el_and_children<Ms>(el: &mut El<Ms>, parent: &web_sys::Node, mailbox: &Mailbox<Ms>) {
    // No parent means we're operating on the top-level element; append it to the main div.
    // This is how we call this function externally, ie not through recursion.
    let el_ws = el
        .node_ws
        .as_ref()
        .expect("Missing websys el in attach_el_and_children");

    // Append the element

    // todo: This error can occur with raw html elements, but am unsure of the cause.
    if parent.append_child(el_ws).is_err() {
        crate::error("Minor problem with html element (append)");
    }

    el.event_handler_manager
        .attach_listeners(el_ws.clone(), None, mailbox);

    // appending the its children to the el_ws
    for child in &mut el.children {
        match child {
            // Raise the active level once per recursion.
            Node::Element(child_el) => attach_el_and_children(child_el, el_ws, mailbox),
            Node::Text(child_text) => attach_text_node(child_text, el_ws),
            Node::Empty | Node::NoChange => (),
        }
    }

    // Note: Call `set_default_element_state` after child appending,
    // otherwise it breaks autofocus in Firefox
    set_default_element_state(el_ws, el);
}

fn set_default_element_state<Ms>(el_ws: &web_sys::Node, el: &El<Ms>) {
    // @TODO handle also other Auto* attributes?
    // Set focus because of attribute "autofocus"
    if let Some(at_value) = el.attrs.vals.get(&At::AutoFocus) {
        match at_value {
            AtValue::Some(_) | AtValue::None => el_ws
                .dyn_ref::<web_sys::HtmlElement>()
                .expect("Problem casting Node as HtmlElement while focusing")
                .focus()
                .expect("Problem focusing to an element."),
            AtValue::Ignored => (),
        }
    }

    // We set Textarea's initial value through non-standard attribute "value", so we have to simulate
    // the standard way (i.e. `<textarea>A Value</textarea>`)
    if let Some(textarea) = el_ws.dyn_ref::<web_sys::HtmlTextAreaElement>() {
        if let Some(AtValue::Some(value)) = el.attrs.vals.get(&At::Value) {
            textarea.set_value(value);
        }
    }
}

/// Recursively remove all children.
pub fn _remove_children(el: &web_sys::Node) {
    while let Some(child) = el.last_child() {
        el.remove_child(&child).expect("Problem removing child");
    }
}

// Update the attributes, style, text, and events of an element. Does not
// process children, and assumes the tag is the same. Assume we've identfied
// the most-correct pairing between new and old.
pub(crate) fn patch_el_details<Ms>(
    old: &mut El<Ms>,
    new: &mut El<Ms>,
    old_el_ws: &web_sys::Node,
    mailbox: &Mailbox<Ms>,
) {
    fix_attrs_order(&mut new.attrs);

    for (key, new_val) in &new.attrs.vals {
        match old.attrs.vals.get(key) {
            Some(old_val) => {
                // The value's different
                if old_val != new_val {
                    set_attr_value(old_el_ws, key, new_val);
                }
            }
            None => {
                set_attr_value(old_el_ws, key, new_val);
            }
        }

        // We handle value in the vdom using attributes, but the DOM needs
        // to use set_value or set_checked.
        match key {
            At::Value => match new_val {
                AtValue::Some(new_val) => crate::util::set_value(old_el_ws, new_val),
                AtValue::None | AtValue::Ignored => crate::util::set_value(old_el_ws, ""),
            },
            At::Checked => match new_val {
                AtValue::Some(_) | AtValue::None => crate::util::set_checked(old_el_ws, true),
                AtValue::Ignored => crate::util::set_checked(old_el_ws, false),
            },
            _ => Ok(()),
        }
        .unwrap_or_else(|err| {
            crate::error(err);
        });
    }
    // Remove attributes that aren't in the new vdom.
    for (key, old_val) in &old.attrs.vals {
        if new.attrs.vals.get(key).is_none() {
            // todo get to the bottom of this
            match old_el_ws.dyn_ref::<web_sys::Element>() {
                Some(el) => {
                    el.remove_attribute(key.as_str())
                        .expect("Removing an attribute");

                    // We handle value in the vdom using attributes, but the DOM needs
                    // to use set_value or set_checked.
                    match key {
                        At::Value => match old_val {
                            AtValue::Some(_) => crate::util::set_value(old_el_ws, ""),
                            _ => Ok(()),
                        },
                        At::Checked => match old_val {
                            AtValue::Some(_) | AtValue::None => {
                                crate::util::set_checked(old_el_ws, false)
                            }
                            AtValue::Ignored => Ok(()),
                        },
                        _ => Ok(()),
                    }
                    .unwrap_or_else(|err| {
                        crate::error(err);
                    });
                }
                None => {
                    crate::error("Minor error on html element (setting attrs)");
                }
            }
        }
    }

    // Patch event handlers and listeners.
    new.event_handler_manager.attach_listeners(
        old_el_ws.clone(),
        Some(&mut old.event_handler_manager),
        mailbox,
    );

    // Patch style.
    if old.style != new.style {
        // We can't patch each part of style; rewrite the whole attribute.
        set_style(old_el_ws, &new.style);
    }
}

/// Some elements have order-sensitive attributes.
///
/// See the [example](https://github.com/seed-rs/seed/issues/335) of such element.
#[allow(clippy::match_same_arms)]
fn fix_attrs_order(attrs: &mut Attrs) {
    attrs.vals.sort_by(|at_a, _, at_b, _| {
        // Move `At::Value` at the end.
        match (at_a, at_b) {
            (At::Value, At::Value) => Ordering::Equal,
            (At::Value, _) => Ordering::Greater,
            (_, At::Value) => Ordering::Less,
            _ => Ordering::Equal,
        }
    });
}

#[allow(clippy::too_many_lines)]
impl<Ms> From<&web_sys::Element> for El<Ms> {
    /// Create a vdom node from a `web_sys::Element`. Used in creating elements from html
    /// and markdown strings. Includes children, recursively added.
    #[allow(clippy::too_many_lines)]
    fn from(ws_el: &web_sys::Element) -> Self {
        let namespace = ws_el.namespace_uri().map(Namespace::from);
        let mut el = match namespace {
            // tag_name returns all caps for HTML, but Tag::from uses lowercase names for HTML
            Some(Namespace::Html) => El::empty(ws_el.tag_name().to_lowercase().into()),
            _ => El::empty(ws_el.tag_name().into()),
        };

        // Populate attributes
        let mut attrs = Attrs::empty();
        ws_el
            .get_attribute_names()
            .for_each(&mut |attr_name, _, _| {
                let attr_name = attr_name
                    .as_string()
                    .expect("problem converting attr to string");
                if let Some(attr_val) = ws_el.get_attribute(&attr_name) {
                    attrs.add(attr_name.into(), &attr_val);
                }
            });
        el.attrs = attrs;

        // todo This is the same list in `shortcuts::element_svg!`.
        // todo: Fix this repetition: Use `/scripts/populate_tags.rs`
        // todo to consolodate these lists.
        let svg_tags = [
            "line",
            "rect",
            "circle",
            "ellipse",
            "polygon",
            "polyline",
            "mesh",
            "path",
            "defs",
            "g",
            "marker",
            "mask",
            "pattern",
            "svg",
            "switch",
            "symbol",
            "unknown",
            "linearGradient",
            "radialGradient",
            "meshGradient",
            "stop",
            "image",
            "use",
            "altGlyph",
            "altGlyphDef",
            "altGlyphItem",
            "glyph",
            "glyphRef",
            "textPath",
            "text",
            "tref",
            "tspan",
            "clipPath",
            "cursor",
            "filter",
            "foreignObject",
            "hathpath",
            "meshPatch",
            "meshRow",
            "view",
            "colorProfile",
            "animate",
            "animateColor",
            "animateMotion",
            "animateTransform",
            "discard",
            "mpath",
            "set",
            "desc",
            "metadata",
            "title",
            "feBlend",
            "feColorMatrix",
            "feComponentTransfer",
            "feComposite",
            "feConvolveMatrix",
            "feDiffuseLighting",
            "feDisplacementMap",
            "feDropShadow",
            "feFlood",
            "feFuncA",
            "feFuncB",
            "feFuncG",
            "feFuncR",
            "feGaussianBlur",
            "feImage",
            "feMerge",
            "feMergeNode",
            "feMorphology",
            "feOffset",
            "feSpecularLighting",
            "feTile",
            "feTurbulence",
            "font",
            "hkern",
            "vkern",
            "hatch",
            "solidcolor",
        ];

        if svg_tags.contains(&ws_el.tag_name().as_str()) {
            el.namespace = Some(Namespace::Svg);
        }

        if let Some(ref ns) = namespace {
            // Prevent attaching a `xlmns` attribute to normal HTML elements.
            if ns != &Namespace::Html {
                el.namespace = namespace;
            }
        }

        let children = ws_el.child_nodes();
        for i in 0..children.length() {
            let child = children
                .get(i)
                .expect("Can't find child in raw html element.");

            if let Some(child_vdom) = node_from_ws(&child) {
                el.children.push(child_vdom);
            }
        }
        el
    }
}
impl<Ms> From<&web_sys::Element> for Node<Ms> {
    fn from(ws_el: &web_sys::Element) -> Node<Ms> {
        Node::Element(ws_el.into())
    }
}

/// Create a vdom node from a `web_sys::Node`. Used in creating elements from html
/// and markdown strings. Includes children, recursively added.
pub fn node_from_ws<Ms>(node: &web_sys::Node) -> Option<Node<Ms>> {
    match node.node_type() {
        web_sys::Node::ELEMENT_NODE => {
            // Element node
            let ws_el = node
                .dyn_ref::<web_sys::Element>()
                .expect("Problem casting Node as Element");

            // Create the Element
            Some(ws_el.into())
        }
        web_sys::Node::TEXT_NODE => Some(Node::new_text(
            node.text_content().expect("Can't find text"),
        )),
        web_sys::Node::COMMENT_NODE => None,
        node_type => {
            crate::error(format!(
                "HTML node type {} is not supported by Seed",
                node_type
            ));
            None
        }
    }
}

/// Insert a new node into the specified part of the DOM tree.
pub(crate) fn insert_node(
    node: &web_sys::Node,
    parent: &web_sys::Node,
    next: Option<web_sys::Node>,
) {
    match next {
        Some(n) => {
            parent
                .insert_before(node, Some(&n))
                .expect("Problem inserting node");
        }
        None => {
            parent.append_child(node).expect("Problem inserting node");
        }
    };
}

pub(crate) fn remove_node(node: &web_sys::Node, parent: &web_sys::Node) {
    parent
        .remove_child(node)
        .expect("Problem removing old el_ws when updating to empty");
}

pub(crate) fn replace_child(new: &web_sys::Node, old: &web_sys::Node, parent: &web_sys::Node) {
    parent
        .replace_child(new, old)
        .expect("Problem replacing element");
}
