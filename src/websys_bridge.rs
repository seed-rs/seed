//! This file contains interactions with web_sys.
use wasm_bindgen::JsCast;

use crate::dom_types;
use crate::dom_types::El;
use crate::vdom::App;

/// Reduces DRY
/// todo can't find a suitable trait for this. Seems like set_autofocus is
/// implemented individually for each of these el types.
//fn autofocus_helper<E: JsCast>(el: E) {
//    match val {
//        "true" => {
//            el.set_autofocus(true);
//            set_special = true;
//        },
//        "false" => {
//            el.set_autofocus(false);
//            set_special = true;
//        },
//        _ => ()
//    }
//}

/// Add a shim to make check logic more natural than the DOM handles it.
fn set_attr_shim(el_ws: &web_sys::Node, at: &dom_types::At, val: &str) {
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
    //todo needs to work for other type sof input!
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

    if !set_special {
        match el_ws.node_type() {
            1 => el_ws
                .dyn_ref::<web_sys::Element>()
                .expect("Problem casting Node as Element while setting an attribute")
                .set_attribute(at, val)
                .expect("Problem setting an atrribute."),
            3 => crate::log("Trying to set attr on text node. Bug?"),
            _ => crate::log("Found non el/text node."),
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

/// Create and return a web_sys Element from our virtual-dom El. The web_sys
/// Element is a close analog to JS/DOM elements.
/// web-sys reference: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
/// Mozilla reference: https://developer.mozilla.org/en-US/docs/Web/HTML/Element\
/// See also: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html
pub fn make_websys_el<Ms: Clone>(
    el_vdom: &mut El<Ms>,
    document: &web_sys::Document,
) -> web_sys::Node {
    // A simple text node.
    if let Some(text) = &el_vdom.text {
        return document.create_text_node(&text).into();
    }

    // Create the DOM-analog element; it won't render until attached to something.
    let tag = el_vdom.tag.as_str();

    let el_ws = match el_vdom.namespace {
        Some(ref ns) => document
            .create_element_ns(Some(ns.as_str()), tag)
            .expect("Problem creating web-sys El"),
        None => document
            .create_element(tag)
            .expect("Problem creating web-sys El"),
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

    // Don't attach listeners here,
    el_ws.into()
}

/// Similar to attach_el_and_children, but assumes we've already attached the parent.
pub fn attach_children<Ms: Clone, Mdl>(el_vdom: &mut El<Ms>, app: &App<Ms, Mdl>) {
    let el_ws = el_vdom
        .el_ws
        .take()
        .expect("Missing websys el in attach children");

    for child in &mut el_vdom.children {
        attach_el_and_children(child, &el_ws, app)
    }

    el_vdom.el_ws.replace(el_ws);
}

/// Attaches the element, and all children, recursively. Only run this when creating a fresh vdom node, since
/// it performs a rerender of the el and all children; eg a potentially-expensive op.
/// This is where rendering occurs.
pub fn attach_el_and_children<Ms: Clone, Mdl>(
    el_vdom: &mut El<Ms>,
    parent: &web_sys::Node,
    app: &App<Ms, Mdl>,
) {
    // No parent means we're operating on the top-level element; append it to the main div.
    // This is how we call this function externally, ie not through recursion.

    // Don't render if we're dealing with an empty element.
    if el_vdom.empty {
        return;
    }

    let el_ws = el_vdom.el_ws.take().expect("Missing websys el");

    // Append the element

    // todo: This can occur with raw html elements, but am unsur eof the cause.
    match parent.append_child(&el_ws) {
        Ok(_) => {}
        Err(_) => {
            crate::log("Minor problem with html element (append)");
        }
    }

    // appending the its children to the el_ws
    for child in &mut el_vdom.children {
        // Raise the active level once per recursion.
        attach_el_and_children(child, &el_ws, app)
    }

    // Perform side-effects specified for mounting.
    if let Some(mount_actions) = &mut el_vdom.hooks.did_mount {
        (mount_actions.actions)(&el_ws);
        if let Some(message) = mount_actions.message.clone() {
            //            app.update(message);
        }
    }

    // Replace the web_sys el
    el_vdom.el_ws.replace(el_ws);
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
                        set_attr_shim(&old_el_ws, key, new_val);
                    }
                }
                None => set_attr_shim(&old_el_ws, key, new_val),
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
                    None => crate::log("Minor error on html element (setting attrs)"),
                }

                //                old_el_ws
                //                    .dyn_ref::<web_sys::Element>()
                //                    .expect("Problem casting Node as Element while removing an attribute")
                //                    .remove_attribute(name.as_str())
                //                    .expect("Removing an attribute");
            }
        }
    }

    // Patch style.
    if old.style != new.style {
        // We can't patch each part of style; rewrite the whole attribute.
        set_style(&old_el_ws, &new.style)
    }

    // Patch text
    if old.text != new.text {
        // We need to change from Option<String> to Option<&str>
        match new.text.clone() {
            Some(text) => old_el_ws.set_text_content(Some(&text)),
            None => old_el_ws.set_text_content(None),
        }
    }
}

/// Convenience function used in event handling: Convert an event target
/// to an input element; eg so you can take its value.
pub fn to_input(target: &web_sys::EventTarget) -> &web_sys::HtmlInputElement {
    target
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("Unable to cast as an input element")
}

/// See to_input
pub fn to_textarea(target: &web_sys::EventTarget) -> &web_sys::HtmlTextAreaElement {
    target
        .dyn_ref::<web_sys::HtmlTextAreaElement>()
        .expect("Unable to cast as a textarea element")
}

/// See to_input
pub fn to_select(target: &web_sys::EventTarget) -> &web_sys::HtmlSelectElement {
    target
        .dyn_ref::<web_sys::HtmlSelectElement>()
        .expect("Unable to cast as a select element")
}

/// See to_input
pub fn to_html_el(target: &web_sys::EventTarget) -> &web_sys::HtmlElement {
    target
        .dyn_ref::<web_sys::HtmlElement>()
        .expect("Unable to cast as an HTML element")
}

/// Convert a web_sys::Event to a web_sys::KeyboardEvent. Useful for extracting
/// info like which key has been pressed, which is not available with normal Events.
pub fn to_kbevent(event: &web_sys::Event) -> &web_sys::KeyboardEvent {
    event
        .dyn_ref::<web_sys::KeyboardEvent>()
        .expect("Unable to cast as a keyboard event")
}

/// See to_kbevent
pub fn to_mouse_event(event: &web_sys::Event) -> &web_sys::MouseEvent {
    event
        .dyn_ref::<web_sys::MouseEvent>()
        .expect("Unable to cast as a mouse event")
}

/// Create a vdom node from a web_sys::Element. Used in creating elements from html
/// and markdown strings. Includes children, recursively added.
pub fn el_from_ws<Ms>(node: &web_sys::Node) -> Option<El<Ms>> {
    match node.node_type() {
        1 => {
            // Element node
            let el_ws = node
                .dyn_ref::<web_sys::Element>()
                .expect("Problem casting Node as Element");

            // Result of tag_name is all caps, but tag From<String> expects lower.
            // Probably is more pure to match by xlmns attribute instead.
            let mut result = match el_ws.tag_name().to_lowercase().as_ref() {
                "svg" => El::empty_svg(el_ws.tag_name().to_lowercase().into()),
                _ => El::empty(el_ws.tag_name().to_lowercase().into()),
            };

            // Populate attributes
            let mut attrs = dom_types::Attrs::empty();
            el_ws
                .get_attribute_names()
                .for_each(&mut |attr_name, _, _| {
                    let attr_name2 = attr_name
                        .as_string()
                        .expect("problem converting attr to string");
                    if let Some(attr_val) = el_ws.get_attribute(&attr_name2) {
                        attrs.add(attr_name2.into(), &attr_val);
                    }
                });
            result.attrs = attrs;

            if let Some(ns) = el_ws.namespace_uri() {
                result.namespace = Some(ns.into());
            }

            let children = el_ws.child_nodes();
            for i in 0..children.length() {
                let child = children
                    .get(i)
                    .expect("Can't find child in raw html element.");

                if let Some(child_vdom) = el_from_ws(&child) {
                    result.children.push(child_vdom);
                }
            }
            Some(result)
        }
        3 => {
            // Text node
            //            None
            //            match &node.text_content() {
            //                Some(t) => {
            //                    if t.chars().count() < 0{
            //                        None
            //                    } else {
            //                        None
            ////                        Some(El::new_text(t))
            //                    }
            //                },
            //                None => None,
            //            }

            Some(El::new_text(&node.text_content().expect("Can't find text")))
        }
        _ => {
            crate::log("Unexpected node type found from raw html");
            None
        }
    }
}
