//! This file contains interactions with `web_sys`.
use crate::dom_types;
use crate::dom_types::{AtValue, El, Node, Text, Namespace};

use wasm_bindgen::JsCast;
use web_sys::Document;

/// Convenience function to reduce repetition
fn set_style(el_ws: &web_sys::Node, style: &dom_types::Style) {
    el_ws
        .dyn_ref::<web_sys::Element>()
        .expect("Problem casting Node as Element while setting style")
        .set_attribute("style", &style.to_string())
        .expect("Problem setting style");
}

/// Recursively create `web_sys::Node`s, and place them in the vdom Nodes' fields.
pub(crate) fn assign_ws_nodes<Ms: Clone>(document: &Document, node: &mut Node<Ms>)
where
    Ms: 'static,
{
    match node {
        Node::Element(el) => {
            el.node_ws = Some(make_websys_el(el, document));
            for mut child in &mut el.children {
                assign_ws_nodes(document, &mut child);
            }
        }
        Node::Text(text) => {
            text.node_ws = Some(
                document
                    .create_text_node(&text.text)
                    .dyn_into::<web_sys::Node>()
                    .expect("Problem casting Text as Node."),
            );
        }
        Node::Empty => (),
    }
}

fn node_to_element(el_ws: &web_sys::Node) -> Result<&web_sys::Element, &'static str> {
    if let web_sys::Node::ELEMENT_NODE = el_ws.node_type() {
        el_ws
            .dyn_ref::<web_sys::Element>()
            .ok_or("Problem casting Node as Element")
    } else {
        Err("Node isn't Element!")
    }
}

fn set_attr_value(el_ws: &web_sys::Node, at: &dom_types::At, at_value: &AtValue) {
    match at_value {
        AtValue::Some(value) => {
            node_to_element(el_ws)
                .and_then(|element| {
                    element
                        .set_attribute(at.as_str(), value)
                        .map_err(|_| "Problem setting an atrribute.")
                })
                .unwrap_or_else(|err| {
                    crate::error(err);
                });
        }
        AtValue::None => {
            node_to_element(el_ws)
                .and_then(|element| {
                    element
                        .set_attribute(at.as_str(), "")
                        .map_err(|_| "Problem setting an atrribute.")
                })
                .unwrap_or_else(|err| {
                    crate::error(err);
                });
        }
        AtValue::Ignored => {
            node_to_element(el_ws)
                .and_then(|element| {
                    element
                        .remove_attribute(at.as_str())
                        .map_err(|_| "Problem removing an atrribute.")
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
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element\)
/// * See also: [`web_sys` Node](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html)
pub(crate) fn make_websys_el<Ms: Clone>(
    el_vdom: &mut El<Ms>,
    document: &web_sys::Document,
) -> web_sys::Node {
    let tag = el_vdom.tag.as_str();

    let el_ws = match el_vdom.namespace {
        Some(ref ns) => document
            .create_element_ns(Some(ns.as_str()), tag)
            .expect("Problem creating web-sys element with namespace"),
        None => document
            .create_element(tag)
            .expect("Problem creating web-sys element"),
    };

    for (at, attr_value) in &el_vdom.attrs.vals {
        set_attr_value(&el_ws, at, attr_value);
    }
    if let Some(ns) = &el_vdom.namespace {
        el_ws
            .dyn_ref::<web_sys::Element>()
            .expect("Problem casting Node as Element while setting an attribute")
            .set_attribute("xmlns", ns.as_str())
            .expect("Problem setting xlmns attribute");
    }

    // Style is just an attribute in the actual Dom, but is handled specially in our vdom;
    // merge the different parts of style here.
    if el_vdom.style.vals.keys().len() > 0 {
        set_style(&el_ws, &el_vdom.style)
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
pub fn attach_children<Ms: Clone>(el_vdom: &mut El<Ms>) {
    let el_ws = el_vdom
        .node_ws
        .as_ref()
        .expect("Missing websys el in attach_children");
    // appending the its children to the el_ws
    for child in &mut el_vdom.children {
        match child {
            // Raise the active level once per recursion.
            Node::Element(child_el) => attach_el_and_children(child_el, el_ws),
            Node::Text(child_text) => attach_text_node(child_text, el_ws),
            Node::Empty => (),
        }
    }
}

/// Attaches the element, and all children, recursively. Only run this when creating a fresh vdom node, since
/// it performs a rerender of the el and all children; eg a potentially-expensive op.
/// This is where rendering occurs.
pub fn attach_el_and_children<Ms: Clone>(el_vdom: &mut El<Ms>, parent: &web_sys::Node) {
    // No parent means we're operating on the top-level element; append it to the main div.
    // This is how we call this function externally, ie not through recursion.
    let el_ws = el_vdom
        .node_ws
        .as_ref()
        .expect("Missing websys el in attach_el_and_children");

    // Append the element

    // todo: This error can occur with raw html elements, but am unsure of the cause.
    if parent.append_child(el_ws).is_err() {
        crate::error("Minor problem with html element (append)");
    }

    // appending the its children to the el_ws
    for child in &mut el_vdom.children {
        match child {
            // Raise the active level once per recursion.
            Node::Element(child_el) => attach_el_and_children(child_el, el_ws),
            Node::Text(child_text) => attach_text_node(child_text, el_ws),
            Node::Empty => (),
        }
    }

    // Note: Call `set_default_element_state` after child appending,
    // otherwise it breaks autofocus in Firefox
    set_default_element_state(el_ws, el_vdom);

    // Perform side-effects specified for mounting.
    if let Some(mount_actions) = &mut el_vdom.hooks.did_mount {
        (mount_actions.actions)(el_ws);
        //        if let Some(message) = mount_actions.message.clone() {
        //            app.update(message);
        //        }
    }
}

fn set_default_element_state<Ms: Clone>(el_ws: &web_sys::Node, el_vdom: &El<Ms>) {
    // @TODO handle also other Auto* attributes?
    // Set focus because of attribute "autofocus"
    if let Some(at_value) = el_vdom.attrs.vals.get(&dom_types::At::AutoFocus) {
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
        if let Some(AtValue::Some(value)) = el_vdom.attrs.vals.get(&dom_types::At::Value) {
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

/// Update the attributes, style, text, and events of an element. Does not
/// process children, and assumes the tag is the same. Assume we've identfied
/// the most-correct pairing between new and old.
pub fn patch_el_details<Ms: Clone>(old: &mut El<Ms>, new: &mut El<Ms>, old_el_ws: &web_sys::Node) {
    // Perform side-effects specified for updating
    if let Some(update_actions) = &mut new.hooks.did_update {
        (update_actions.actions)(old_el_ws) // todo
    }

    if old.attrs != new.attrs {
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
                dom_types::At::Value => match new_val {
                    AtValue::Some(new_val) => crate::util::set_value(old_el_ws, new_val),
                    AtValue::None | AtValue::Ignored => crate::util::set_value(old_el_ws, ""),
                },
                dom_types::At::Checked => match new_val {
                    AtValue::Some(_) | AtValue::None => crate::util::set_checked(old_el_ws, true),
                    AtValue::Ignored => crate::util::set_checked(old_el_ws, false),
                },
                _ => Ok(()),
            }
            .unwrap_or_else(|err| {
                crate::error(err);
            })
        }
        // Remove attributes that aren't in the new vdom.
        for name in old.attrs.vals.keys() {
            if new.attrs.vals.get(name).is_none() {
                // todo get to the bottom of this
                match old_el_ws.dyn_ref::<web_sys::Element>() {
                    Some(el) => el
                        .remove_attribute(name.as_str())
                        .expect("Removing an attribute"),
                    None => {
                        crate::error("Minor error on html element (setting attrs)");
                    }
                }
            }
        }
    }

    // Patch style.
    if old.style != new.style {
        // We can't patch each part of style; rewrite the whole attribute.
        set_style(old_el_ws, &new.style)
    }
}

/// Convenience function used in event handling: Convert an event target
/// to an input element; eg so you can take its value.
pub fn to_input(target: &web_sys::EventTarget) -> &web_sys::HtmlInputElement {
    target
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("Unable to cast as an input element")
}

/// See [`to_input`](fn.to_input.html)
pub fn to_textarea(target: &web_sys::EventTarget) -> &web_sys::HtmlTextAreaElement {
    target
        .dyn_ref::<web_sys::HtmlTextAreaElement>()
        .expect("Unable to cast as a textarea element")
}

/// See [`to_input`](fn.to_input.html)
pub fn to_select(target: &web_sys::EventTarget) -> &web_sys::HtmlSelectElement {
    target
        .dyn_ref::<web_sys::HtmlSelectElement>()
        .expect("Unable to cast as a select element")
}

/// See [`to_input`](fn.to_input.html)
pub fn to_html_el(target: &web_sys::EventTarget) -> &web_sys::HtmlElement {
    target
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("Unable to cast as an HTML element")
}

/// Convert a `web_sys::Event` to a `web_sys::KeyboardEvent`. Useful for extracting
/// info like which key has been pressed, which is not available with normal Events.
pub fn to_kbevent(event: &web_sys::Event) -> &web_sys::KeyboardEvent {
    event
        .dyn_ref::<web_sys::KeyboardEvent>()
        .expect("Unable to cast as a keyboard event")
}

/// See `to_kbevent`
pub fn to_mouse_event(event: &web_sys::Event) -> &web_sys::MouseEvent {
    event
        .dyn_ref::<web_sys::MouseEvent>()
        .expect("Unable to cast as a mouse event")
}

/// Create a vdom node from a `web_sys::Element`. Used in creating elements from html
/// and markdown strings. Includes children, recursively added.
pub fn node_from_ws<Ms: Clone>(node: &web_sys::Node) -> Option<Node<Ms>> {
    match node.node_type() {
        web_sys::Node::ELEMENT_NODE => {
            // Element node
            let node_ws = node
                .dyn_ref::<web_sys::Element>()
                .expect("Problem casting Node as Element");

            // Result of tag_name is all caps, but tag From<String> expects lower.
            // Probably is more pure to match by xlmns attribute instead.
            let mut el = match node_ws.tag_name().to_lowercase().as_ref() {
                "svg" => El::empty_svg(node_ws.tag_name().to_lowercase().into()),
                _ => El::empty(node_ws.tag_name().to_lowercase().into()),
            };

            // Populate attributes
            let mut attrs = dom_types::Attrs::empty();
            node_ws
                .get_attribute_names()
                .for_each(&mut |attr_name, _, _| {
                    let attr_name2 = attr_name
                        .as_string()
                        .expect("problem converting attr to string");
                    if let Some(attr_val) = node_ws.get_attribute(&attr_name2) {
                        attrs.add(attr_name2.into(), &attr_val);
                    }
                });
            el.attrs = attrs;

            // todo etc
            let svg_tags = vec![
                "svg", "circle", "line", "rect"
            ];
            let svg_tags: Vec<String> = svg_tags.into_iter().map(|t| t.to_string()).collect();

            crate::log(&node_ws.tag_name());
            if svg_tags.contains(&node_ws.tag_name().to_lowercase()) {
                crate::log(&format!("Found one!: {}", &node_ws.tag_name()));
                el.namespace = Some(Namespace::Svg);
            }

            if let Some(ns) = node_ws.namespace_uri() {
                // Prevent attaching a `xlmns` attribute to normal HTML elements.
                if ns != "http://www.w3.org/1999/xhtml" {
                    el.namespace = Some(ns.into());
                }
            }

            let children = node_ws.child_nodes();
            for i in 0..children.length() {
                let child = children
                    .get(i)
                    .expect("Can't find child in raw html element.");

                if let Some(child_vdom) = node_from_ws(&child) {
                    el.children.push(child_vdom);
                }
            }
            Some(Node::Element(el))
        }
        web_sys::Node::TEXT_NODE => Some(Node::Text(Text::new(
            node.text_content().expect("Can't find text"),
        ))),
        _ => {
            crate::error("Unexpected node type found from raw html");
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
