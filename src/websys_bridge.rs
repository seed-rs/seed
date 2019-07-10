//! This file contains interactions with `web_sys`.
use crate::dom_types;
use crate::dom_types::{El, Node, Text};

use wasm_bindgen::JsCast;
use web_sys::Document;

/// Add a shim to make check logic more natural than the DOM handles it.
fn set_attr_shim(el_ws: &web_sys::Node, at: &dom_types::At, val: &str) {
    // set_special means we don't set the attribute normally.
    let mut set_special = false;
    let at = at.as_str();

    if at == "checked" {
        let input_el = el_ws.dyn_ref::<web_sys::HtmlInputElement>();
        if let Some(el) = input_el {
            match val {
                "true" => {
                    el.set_checked(true);
                }
                "false" => {
                    el.set_checked(false);
                }
                _ => (),
            }
            set_special = true;
        }
    }
    // todo DRY! Massive dry between checked and auto, and in autofocus.
    // https://www.w3schools.com/tags/att_autofocus.asp
    //todo needs to work for other types of input!
    else if at == "autofocus" {
        if let Some(input) = el_ws.dyn_ref::<web_sys::HtmlInputElement>() {
            //            autofocus_helper(input)
            match val {
                "true" => {
                    input.set_autofocus(true);
                }
                "false" => {
                    input.set_autofocus(false);
                }
                _ => (),
            }
            set_special = true;
        }
        if let Some(input) = el_ws.dyn_ref::<web_sys::HtmlTextAreaElement>() {
            //             autofocus_helper(input)
            match val {
                "true" => {
                    input.set_autofocus(true);
                }
                "false" => {
                    input.set_autofocus(false);
                }
                _ => (),
            }
            set_special = true;
        }
        if let Some(input) = el_ws.dyn_ref::<web_sys::HtmlSelectElement>() {
            //             autofocus_helper(input)
            match val {
                "true" => {
                    input.set_autofocus(true);
                }
                "false" => {
                    input.set_autofocus(false);
                }
                _ => (),
            }
            set_special = true;
        }
        if let Some(input) = el_ws.dyn_ref::<web_sys::HtmlButtonElement>() {
            //             autofocus_helper(input)
            match val {
                "true" => {
                    input.set_autofocus(true);
                }
                "false" => {
                    input.set_autofocus(false);
                }
                _ => (),
            }
            set_special = true;
        }
    }
    // A disabled value of anything, including "", means disabled. To make not disabled,
    // the disabled attr can't be present.
    // Without this shim, setting At::Disabled => false still disables the field.
    else if at == "disabled" && val == "false" {
        match el_ws.node_type() {
            // https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType#Node_type_constants
            1 => el_ws
                .dyn_ref::<web_sys::Element>()
                .expect("Problem casting Node as Element while removing the attribute `disabled`")
                .remove_attribute(at)
                .expect("Problem removing the atrribute `disabled`."),
            _ => crate::error("Found non el node while removing attribute `disabled`."),
        }
        set_special = true;
    }

    if !set_special {
        match el_ws.node_type() {
            // https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType#Node_type_constants
            1 => el_ws
                .dyn_ref::<web_sys::Element>()
                .expect("Problem casting Node as Element while setting an attribute")
                .set_attribute(at, val)
                .expect("Problem setting an atrribute."),
            3 => crate::error("Trying to set attr on text node. Bug?"),
            _ => crate::error("Found non el/text node."),
        }
    }
}

/// Convenience function to reduce repetition
fn set_style(el_ws: &web_sys::Node, style: &dom_types::Style) {
    el_ws
        .dyn_ref::<web_sys::Element>()
        .expect("Problem casting Node as Element while setting style")
        .set_attribute("style", &style.to_string())
        .expect("Problem setting style");
}

/// Recursively create `web_sys::Node`s, and place them in the vdom Nodes' fields.
pub(crate) fn assign_ws_nodes<Ms>(document: &Document, node: &mut Node<Ms>)
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

/// Create and return a `web_sys` Element from our virtual-dom `El`. The `web_sys`
/// Element is a close analog to JS/DOM elements.
///
/// # References
/// * [`web_sys` Element](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html)
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element\)
/// * See also: [`web_sys` Node](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html)
pub(crate) fn make_websys_el<Ms>(
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

    for (at, val) in &el_vdom.attrs.vals {
        set_attr_shim(&el_ws, at, val);
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
pub fn attach_children<Ms>(el_vdom: &mut El<Ms>) {
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
pub fn attach_el_and_children<Ms>(el_vdom: &mut El<Ms>, parent: &web_sys::Node) {
    // No parent means we're operating on the top-level element; append it to the main div.
    // This is how we call this function externally, ie not through recursion.

    let el_ws = el_vdom
        .node_ws
        .as_ref()
        .expect("Missing websys el in attach_el_and_children");

    // Append the element

    // todo: This can occur with raw html elements, but am unsur eof the cause.
    match parent.append_child(el_ws) {
        Ok(_) => {}
        Err(_) => {
            crate::log("Minor problem with html element (append)");
        }
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

    // Perform side-effects specified for mounting.
    if let Some(mount_actions) = &mut el_vdom.hooks.did_mount {
        (mount_actions.actions)(el_ws);
        //        if let Some(message) = mount_actions.message.clone() {
        //            app.update(message);
        //        }
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
pub fn patch_el_details<Ms>(old: &mut El<Ms>, new: &mut El<Ms>, old_el_ws: &web_sys::Node) {
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
                        set_attr_shim(old_el_ws, key, new_val);
                    }
                }
                None => set_attr_shim(old_el_ws, key, new_val),
            }
            // We handle value in the vdom using attributes, but the DOM needs
            // to use set_value.
            if key == &dom_types::At::Value {
                crate::util::set_value(old_el_ws, new_val);
            }
        }
        // Remove attributes that aren't in the new vdom.
        for name in old.attrs.vals.keys() {
            if new.attrs.vals.get(name).is_none() {
                // todo get to the bottom of this
                match old_el_ws.dyn_ref::<web_sys::Element>() {
                    Some(el) => el
                        .remove_attribute(name.as_str())
                        .expect("Removing an attribute"),
                    None => crate::error("Minor error on html element (setting attrs)"),
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
pub fn node_from_ws<Ms>(node: &web_sys::Node) -> Option<Node<Ms>> {
    match node.node_type() {
        1 => {
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
                        attrs.add(attr_name2.into(), attr_val);
                    }
                });
            el.attrs = attrs;

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
        3 => Some(Node::Text(Text::new(
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
