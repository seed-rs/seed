//! This module contains code related to patching the VDOM. It can be considered
//! a subset of the `vdom` module.

use crate::{
    dom_types::{self, AtValue, El, Node, View},
    events::{self, Listener},
    vdom::{App, Mailbox},
    websys_bridge,
};
use wasm_bindgen::JsCast;
use web_sys::{Document, Window};

/// Recursively attach all event-listeners. Run this after creating elements.
/// The associated `web_sys` nodes must be assigned prior to running this.
pub(crate) fn attach_listeners<Ms>(el: &mut El<Ms>, mailbox: &Mailbox<Ms>) {
    if let Some(el_ws) = el.node_ws.as_ref() {
        for listener in &mut el.listeners {
            listener.attach(el_ws, mailbox.clone());
        }
    }
    for child in &mut el.children {
        if let Node::Element(child_el) = child {
            attach_listeners(child_el, mailbox);
        }
    }
}

/// Recursively detach event-listeners. Run this before patching.
pub(crate) fn detach_listeners<Ms>(el: &mut El<Ms>) {
    if let Some(el_ws) = el.node_ws.as_ref() {
        for listener in &mut el.listeners {
            listener.detach(el_ws);
        }
    }
    for child in &mut el.children {
        if let Node::Element(child_el) = child {
            detach_listeners(child_el);
        }
    }
}

/// We reattach all listeners, as with normal Els, since we have no
/// way of diffing them.
pub(crate) fn setup_window_listeners<Ms>(
    window: &Window,
    old: &mut Vec<Listener<Ms>>,
    new: &mut Vec<Listener<Ms>>,
    mailbox: &Mailbox<Ms>,
) {
    for listener in old {
        listener.detach(window);
    }

    for listener in new {
        listener.attach(window, mailbox.clone());
    }
}

/// Remove a node from the vdom and `web_sys` DOM.
pub(crate) fn remove_node<Ms>(node: &web_sys::Node, parent: &web_sys::Node, el_vdom: &mut El<Ms>) {
    websys_bridge::remove_node(node, parent);

    if let Some(unmount_actions) = &mut el_vdom.hooks.will_unmount {
        (unmount_actions.actions)(node);
        //                if let Some(message) = unmount_actions.message.clone() {
        //                    app.update(message);
        //                }
    }
}

/// Set up controlled components: Input, Select, and `TextArea` elements must stay in sync with the
/// model; don't let them get out of sync from typing or other events, which can occur if a change
/// doesn't trigger a re-render, or if something else modifies them using a side effect.
/// Handle controlled inputs: Ie force sync with the model.
fn setup_input_listener<Ms>(el: &mut El<Ms>)
where
    Ms: 'static,
{
    if el.tag == dom_types::Tag::Input
        || el.tag == dom_types::Tag::Select
        || el.tag == dom_types::Tag::TextArea
    {
        let listener = if let Some(checked) = el.attrs.vals.get(&dom_types::At::Checked) {
            events::Listener::new_control_check(match checked {
                AtValue::Some(_) => true,
                _ => false,
            })
        } else if let Some(control_val) = el.attrs.vals.get(&dom_types::At::Value) {
            events::Listener::new_control(match control_val {
                AtValue::Some(value) => value.clone(),
                _ => "".into(),
            })
        } else {
            // If Value is not specified, force the field to be blank.
            events::Listener::new_control("".to_string())
        };
        el.listeners.push(listener); // Add to the El, so we can deattach later.
    }
}

/// Recursively sets up input listeners
pub(crate) fn setup_input_listeners<Ms>(el_vdom: &mut El<Ms>)
where
    Ms: 'static,
{
    setup_input_listener(el_vdom);
    for child in &mut el_vdom.children {
        if let Node::Element(child_el) = child {
            setup_input_listener(child_el);
        }
    }
}

fn patch_el<'a, Ms, Mdl, ElC: View<Ms>, GMs>(
    document: &Document,
    mut old: El<Ms>,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, ElC, GMs>,
) -> Option<&'a web_sys::Node> {
    if old != *new {
        // At this step, we already assume we have the right element - either
        // by entering this func directly for the top-level, or recursively after
        // analyzing children

        // If the tag's different, we must redraw the element and its children; there's
        // no way to patch one element type into another.
        // TODO: forcing a rerender for differnet listeners is inefficient
        // TODO:, but I'm not sure how to patch them.

        // Assume all listeners have been removed from the old el_ws (if any), and the
        // old el vdom's elements are still attached.

        // Namespaces can't be patched, since they involve create_element_ns instead of create_element.
        // Something about this element itself is different: patch it.
        if old.tag != new.tag || old.namespace != new.namespace {
            let old_el_ws = old.node_ws.as_ref().expect("Missing websys el");

            // We don't use assign_nodes directly here, since we only have access to
            // the El, not wrapping node.
            new.node_ws = Some(websys_bridge::make_websys_el(new, document));
            for mut child in &mut new.children {
                websys_bridge::assign_ws_nodes(document, &mut child);
            }
            if let Some(unmount_actions) = &mut old.hooks.will_unmount {
                let old_ws = old.node_ws.as_ref().expect("Missing websys el");
                (unmount_actions.actions)(old_ws);
            }

            websys_bridge::attach_el_and_children(new, parent);

            let new_ws = new.node_ws.as_ref().expect("Missing websys el");
            websys_bridge::replace_child(new_ws, old_el_ws, parent);

            attach_listeners(new, mailbox);
            // We've re-rendered this child and all children; we're done with this recursion.
            return new.node_ws.as_ref();
        } else {
            // Patch parts of the Element.
            let old_el_ws = old
                .node_ws
                .as_ref()
                .expect("missing old el_ws when patching non-empty el")
                .clone();
            websys_bridge::patch_el_details(&mut old, new, &old_el_ws);
        }
    }

    let old_el_ws = old.node_ws.take().unwrap();

    // Before running patch, assume we've removed all listeners from the old element.
    // Perform this attachment after we've verified we can patch this element, ie
    // it has the same tag - otherwise  we'd have to detach after the parent.remove_child step.
    // Note that unlike the attach_listeners function, this only attaches for the current
    // element.
    for listener in &mut new.listeners {
        listener.attach(&old_el_ws, mailbox.clone());
    }

    let num_children_in_both = old.children.len().min(new.children.len());
    let mut old_children_iter = old.children.into_iter();
    let mut new_children_iter = new.children.iter_mut();

    let mut last_visited_node: Option<web_sys::Node> = None;

    // TODO: Lines below commented out, because they were breaking `lifecycle_hooks` test
    //       - did_update was called 2x instead of 1x after 2nd call_patch
    //
    //  if let Some(update_actions) = &mut new.hooks.did_update {
    //      (update_actions.actions)(&old_el_ws) // todo
    //  }

    // Not using .zip() here to make sure we don't miss any of the children when one array is
    // longer than the other.
    for _i in 0..num_children_in_both {
        let child_old = old_children_iter.next().unwrap();
        let child_new = new_children_iter.next().unwrap();

        // Don't compare equality here; we do that at the top of this function
        // in the recursion.
        if let Some(new_el_ws) = patch(
            document,
            child_old,
            child_new,
            &old_el_ws,
            match last_visited_node.as_ref() {
                Some(node) => node.next_sibling(),
                None => old_el_ws.first_child(),
            },
            mailbox,
            app,
        ) {
            last_visited_node = Some(new_el_ws.clone());
        }
    }

    // Now one of the iterators is entirely consumed, and any items left in one iterator
    // don't have any matching items in the other.
    // We ran out of old children to patch; create new ones.
    for child_new in new_children_iter {
        websys_bridge::assign_ws_nodes(document, child_new);

        match child_new {
            Node::Element(child_new_el) => {
                websys_bridge::attach_el_and_children(child_new_el, &old_el_ws);
                attach_listeners(child_new_el, mailbox);
            }
            Node::Text(child_new_text) => {
                websys_bridge::attach_text_node(child_new_text, &old_el_ws);
            }
            Node::Empty => (),
        }
    }

    // Now purge any existing no-longer-needed children; they're not part of the new vdom.
    // while let Some(mut child) = old_children_iter.next() {
    for child in old_children_iter {
        match child {
            Node::Element(mut child_el) => {
                let child_ws = child_el.node_ws.take().expect("Missing child el_ws");
                remove_node(&child_ws, &old_el_ws, &mut child_el);
                child_el.node_ws.replace(child_ws);
            }
            Node::Text(mut child_text) => {
                let child_ws = child_text.node_ws.take().expect("Missing child node_ws");
                websys_bridge::remove_node(&child_ws, &old_el_ws);
                child_text.node_ws.replace(child_ws);
            }
            Node::Empty => (),
        }
    }

    new.node_ws = Some(old_el_ws);
    new.node_ws.as_ref()
}

// Reduces code repetition
fn add_el_helper<Ms>(
    new: &mut El<Ms>,
    parent: &web_sys::Node,
    next_node: Option<web_sys::Node>,
    mailbox: &Mailbox<Ms>,
) {
    websys_bridge::attach_children(new);
    let new_ws = new
        .node_ws
        .take()
        .expect("Missing websys el when patching Text to Element");
    websys_bridge::insert_node(&new_ws, parent, next_node);

    if let Some(mount_actions) = &mut new.hooks.did_mount {
        (mount_actions.actions)(&new_ws);
    }

    new.node_ws.replace(new_ws);
    // Make sure to attach after we've replaced node_ws.
    attach_listeners(new, mailbox);
}

/// Routes patching through different channels, depending on the Node variant
/// of old and new.
pub(crate) fn patch<'a, Ms, Mdl, ElC: View<Ms>, GMs>(
    document: &Document,
    old: Node<Ms>,
    new: &'a mut Node<Ms>,
    parent: &web_sys::Node,
    next_node: Option<web_sys::Node>,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, ElC, GMs>,
) -> Option<&'a web_sys::Node> {
    // Old_el_ws is what we're patching, with items from the new vDOM el; or replacing.
    // We go through each combination of new and old variants to determine how to patch.
    // We return the resulting web_sys node for assistance in inserting subsequent
    // ones in the right place.

    // We assume that when we run this, the new vdom doesn't have assigned `web_sys::Node`s -
    // assign them here when we create them.
    match old {
        Node::Element(mut old_el) => {
            match new {
                Node::Element(new_el) => patch_el(document, old_el, new_el, parent, mailbox, app),
                Node::Text(new_text) => {
                    // Can't just use assign_ws_nodes; borrow-checker issues.
                    new_text.node_ws = Some(
                        document
                            .create_text_node(&new_text.text)
                            .dyn_into::<web_sys::Node>()
                            .expect("Problem casting Text as Node."),
                    );

                    let old_node_ws = old_el
                        .node_ws
                        .take()
                        .expect("old el_ws missing when replacing with text node");
                    let new_node_ws = new_text
                        .node_ws
                        .as_ref()
                        .expect("old el_ws missing when replacing with text node");

                    websys_bridge::replace_child(new_node_ws, &old_node_ws, parent);
                    new_text.node_ws.as_ref()
                }
                Node::Empty => {
                    let old_el_ws = old_el
                        .node_ws
                        .take()
                        .expect("old el_ws missing when patching Element to Empty");
                    remove_node(&old_el_ws, parent, &mut old_el);
                    None
                }
            }
        }
        Node::Empty => {
            // If the old node's empty, assign and attach web_sys nodes.
            websys_bridge::assign_ws_nodes(document, new);
            match new {
                Node::Element(new_el) => {
                    add_el_helper(new_el, parent, next_node, mailbox);
                    new_el.node_ws.as_ref()
                }
                Node::Text(new_text) => {
                    let new_node_ws = new_text
                        .node_ws
                        .as_ref()
                        .expect("new_node_ws missing when patching Empty to Text");
                    websys_bridge::insert_node(new_node_ws, parent, next_node);
                    new_text.node_ws.as_ref()
                }
                // If new and old are empty, we don't need to do anything.
                Node::Empty => None,
            }
        }
        Node::Text(mut old_text) => {
            websys_bridge::assign_ws_nodes(document, new);
            match new {
                Node::Element(new_el) => {
                    websys_bridge::remove_node(
                        &old_text.node_ws.expect("Can't find node from Text"),
                        parent,
                    );

                    // Passing next_node here instead of `None` causes
                    // panics.
                    add_el_helper(new_el, parent, None, mailbox);
                    new_el.node_ws.as_ref()
                }
                Node::Empty => {
                    websys_bridge::remove_node(
                        &old_text.node_ws.expect("Can't find old text"),
                        parent,
                    );
                    None
                }
                Node::Text(new_text) => {
                    let old_node_ws = old_text
                        .node_ws
                        .take()
                        .expect("old_node_ws missing when changing text");

                    if new_text != &old_text {
                        old_node_ws.set_text_content(Some(&new_text.text));
                    }
                    new_text.node_ws.replace(old_node_ws);
                    new_text.node_ws.as_ref()
                }
            }
        }
    }
}
