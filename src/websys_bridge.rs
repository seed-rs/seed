//! This file contains interactions with web_sys.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::vdom::Mailbox;
use crate::dom_types;

/// Add a shim to make check logic more natural than the DOM handles it.
pub fn set_attr_shim(el_ws: &web_sys::Element, name: &str, val: &str) {
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
    if set_check == false {
        el_ws.set_attribute(name, val).expect("Problem setting an atrribute.");
    }

}

/// Create and return a web_sys Element from our virtual-dom El. The web_sys
/// Element is a close analog to JS/DOM elements.
/// web-sys reference: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
/// Mozilla reference: https://developer.mozilla.org/en-US/docs/Web/HTML/Element\
/// See also: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Node.html
pub fn make_websys_el<Ms: Clone>(el_vdom: &mut dom_types::El<Ms>, document: &web_sys::Document,
                                 mailbox: Mailbox<Ms>) -> web_sys::Element {
    // Create the DOM-analog element; it won't render until attached to something.
    let el_ws = document.create_element(&el_vdom.tag.as_str()).unwrap();

    for (name, val) in &el_vdom.attrs.vals {
        set_attr_shim(&el_ws, name, val);
    }

    // Style is just an attribute in the actual Dom, but is handled specially in our vdom;
    // merge the different parts of style here.
    if el_vdom.style.vals.keys().len() > 0 {
        el_ws.set_attribute("style", &el_vdom.style.as_str()).unwrap();
    }

    // We store text as Option<String>, but set_text_content uses Option<&str>.
    // A naive match Some(t) => Some(&t) does not work.
    // See https://stackoverflow.com/questions/31233938/converting-from-optionstring-to-optionstr
    el_ws.set_text_content(el_vdom.text.as_ref().map(String::as_ref));

    for listener in &mut el_vdom.listeners {
        listener.attach(&el_ws, mailbox.clone());
    }

    el_ws
}

/// Attaches the element, and all children, recursively. Only run this when creating a fresh vdom node, since
/// it performs a rerender of the el and all children; eg a potentially-expensive op.
/// This is where rendering occurs.
pub fn attach<Ms: Clone>(el_vdom: &mut dom_types::El<Ms>, parent: &web_sys::Element) {
    // No parent means we're operating on the top-level element; append it to the main div.
    // This is how we call this function externally, ie not through recursion.

    // Don't render if we're dealing with a dummy element.
    // todo get this working. it produes panics
//    if el_vdom.is_dummy() == true { return }

    let el_ws = el_vdom.el_ws.take().expect("Missing websys el");

    crate::log("Rendering element in attach");
    // Purge existing children.
//    parent.remove_child(&el_ws).expect("Missing el_ws");
    // Append its child while it's out of its element.
    parent.append_child(&el_ws).unwrap();

    // todo: It seesm like if text is present along with children, it'll bbe
    // todo shown before them instead of after. Fix this.

    for child in &mut el_vdom.children {
        // Raise the active level once per recursion.
        attach(child, &el_ws)
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