//! This file contains interactions with web_sys.
use wasm_bindgen::JsCast;

use crate::dom_types;

/// Add a shim to make check logic more natural than the DOM handles it.
fn set_attr_shim(el_ws: &web_sys::Element, name: &str, val: &str) {
    let mut set_check = false;

    if name == "checked" {
        let input_el = el_ws.dyn_ref::<web_sys::HtmlInputElement>();
        if let Some(el) = input_el {
            match val {
                "true" => {
                    el.set_checked(true);
                    set_check = true;
                },
                "false" => {
                    el.set_checked(false);
                    set_check = true;
                },
                _ => ()
            }
        }
    }
    if !set_check {
        el_ws.set_attribute(name, val).expect("Problem setting an atrribute.");
    }
}

/// Create and return a web_sys Element from our virtual-dom El. The web_sys
/// Element is a close analog to JS/DOM elements.
/// web-sys reference: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
/// Mozilla reference: https://developer.mozilla.org/en-US/docs/Web/HTML/Element\
/// See also: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html
pub fn make_websys_el<Ms: Clone>(el_vdom: &mut dom_types::El<Ms>, document: &web_sys::Document) -> web_sys::Element {
    // Create the DOM-analog element; it won't render until attached to something.
    let el_ws = document.create_element(&el_vdom.tag.as_str()).expect("Problem creating web-sys El");

    for (name, val) in &el_vdom.attrs.vals {
        set_attr_shim(&el_ws, name, val);
    }

    // Style is just an attribute in the actual Dom, but is handled specially in our vdom;
    // merge the different parts of style here.
    if el_vdom.style.vals.keys().len() > 0 {
        el_ws.set_attribute("style", &el_vdom.style.as_str()).expect("Problem setting style");
    }

    // We store text as Option<String>, but set_text_content uses Option<&str>.
    // A naive match Some(t) => Some(&t) does not work.
    // See https://stackoverflow.com/questions/31233938/converting-from-optionstring-to-optionstr
    el_ws.set_text_content(el_vdom.text.as_ref().map(String::as_ref));

    // Don't attach listeners here: It'll cause a conflict when you try to
    // attach a second time. ?
    // todo delete these lines once events are sorted
//    for listener in &mut el_vdom.listeners {
//        listener.attach(&el_ws, mailbox.clone());
//    }

    el_ws
}

/// Attaches the element, and all children, recursively. Only run this when creating a fresh vdom node, since
/// it performs a rerender of the el and all children; eg a potentially-expensive op.
/// This is where rendering occurs.
pub fn attach_els<Ms: Clone>(el_vdom: &mut dom_types::El<Ms>, parent: &web_sys::Element) {
    // No parent means we're operating on the top-level element; append it to the main div.
    // This is how we call this function externally, ie not through recursion.

    // Don't render if we're dealing with a dummy element.
    // todo get this working. it produes panics
//    if el_vdom.is_dummy() == true { return }

    let el_ws = el_vdom.el_ws.take().expect("Missing websys el");

//    crate::log("Rendering element in attach");
    // Purge existing children.
//    parent.remove_child(&el_ws).expect("Missing el_ws");
    // Append its child while it's out of its element.
    parent.append_child(&el_ws).unwrap();

    // todo: It seesm like if text is present along with children, it'll bbe
    // todo shown before them instead of after. Fix this.

    for child in &mut el_vdom.children {
        // Raise the active level once per recursion.
        attach_els(child, &el_ws)
    }

    // Replace the web_sys el... Indiana-Jones-style.
    el_vdom.el_ws.replace(el_ws);
}

/// Recursively remove all children.
pub fn remove_children(el: &web_sys::Element) {
    while let Some(child) = el.last_child() {
        el.remove_child(&child).unwrap();
    }
}

/// Update the attributes, style, text, and events of an element. Does not
/// process children, and assumes the tag is the same. Assume we've identfied
/// the most-correct pairing between new and old.
pub fn patch_el_details<Ms: Clone>(old: &mut dom_types::El<Ms>, new: &mut dom_types::El<Ms>,
           old_el_ws: &web_sys::Element, document: &web_sys::Document) {

    if old.attrs != new.attrs {
        for (key, new_val) in &new.attrs.vals {
            match old.attrs.vals.get(key) {
                Some(old_val) => {
                    // The value's different
                    if old_val != new_val {
                        set_attr_shim(&old_el_ws, key, new_val);
                    }
                },
                None => old_el_ws.set_attribute(key, new_val).expect("Adding a new attribute")
            }
        }
        // Remove attributes that aren't in the new vdom.
        for name in old.attrs.vals.keys() {
            if new.attrs.vals.get(name).is_none() {
                old_el_ws.remove_attribute(name).expect("Removing an attribute");
            }
        }
    }

    // Patch style.
    if old.style != new.style {
        // We can't patch each part of style; rewrite the whole attribute.
        old_el_ws.set_attribute("style", &new.style.as_str())
            .expect("Setting style");
    }


    // Patch text
    if old.text != new.text {
        // This is not as straightforward as it looks: There can be multiple text nodes
        // in the DOM, even though our API only allows for 1 per element. If we
        // naively run set_text_content(), all child nodes will be removed.
        // Text is stored in special Text nodes that don't have a direct-relation to
        // the vdom.

        let text = new.text.clone().unwrap_or_default();

        if old.text.is_none() {
            // There's no old node to find: Add it.
            let new_next_node = document.create_text_node(&text);
            old_el_ws.append_child(&new_next_node).unwrap();
        } else {
            // Iterating over a NodeList, unfortunately, is not as clean as you might expect.
            let children = old_el_ws.child_nodes();
            for i in 0..children.length() {
                let node = children.item(i).unwrap();
                // We've found it; there will be not more than 1 text node.
                if node.node_type() == 3 {
                    node.set_text_content(Some(&text));
                    break;
                }
            }
        }
    }
}
