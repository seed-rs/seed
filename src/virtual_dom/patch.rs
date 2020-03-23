//! This module contains code related to patching the VDOM. It can be considered
//! a subset of the `vdom` module.

use super::{El, IntoNodes, Mailbox, Node};
use crate::app::App;
use crate::browser::dom::virtual_dom_bridge;
use wasm_bindgen::JsCast;
use web_sys::Document;

fn patch_el<'a, Ms, Mdl, INodes: IntoNodes<Ms>, GMs>(
    document: &Document,
    mut old: El<Ms>,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, INodes, GMs>,
) -> Option<&'a web_sys::Node> {
    // At this step, we already assume we have the right element - either
    // by entering this func directly for the top-level, or recursively after
    // analyzing children

    // If the tag's different, we must redraw the element and its children; there's
    // no way to patch one element type into another.

    // Assume old el vdom's elements are still attached.

    // Namespaces can't be patched, since they involve create_element_ns instead of create_element.
    // Custom elements can't be patched, because we need to reinit them (Issue #325). (@TODO is there a better way?)
    // Something about this element itself is different: patch it.
    if old.tag != new.tag || old.namespace != new.namespace || old.is_custom() {
        let old_el_ws = old.node_ws.as_ref().expect("Missing websys el");

        // We don't use assign_nodes directly here, since we only have access to
        // the El, not wrapping node.
        let new_node_ws = virtual_dom_bridge::make_websys_el(new, document);
        for ref_ in &mut new.refs {
            ref_.set(new_node_ws.clone());
        }
        new.node_ws = Some(new_node_ws);
        for mut child in &mut new.children {
            virtual_dom_bridge::assign_ws_nodes(document, &mut child);
        }
        virtual_dom_bridge::attach_el_and_children(new, parent, mailbox);

        let new_ws = new.node_ws.as_ref().expect("Missing websys el");
        virtual_dom_bridge::replace_child(new_ws, old_el_ws, parent);
    } else {
        // Patch parts of the Element.
        let old_el_ws = old
            .node_ws
            .as_ref()
            .expect("missing old el_ws when patching non-empty el")
            .clone();
        virtual_dom_bridge::patch_el_details(&mut old, new, &old_el_ws, mailbox);

        for ref_ in &mut new.refs {
            ref_.set(old_el_ws.clone());
        }

        let old_children_iter = old.children.into_iter();
        let new_children_iter = new.children.iter_mut();

        patch_els(
            document,
            mailbox,
            app,
            &old_el_ws,
            old_children_iter,
            new_children_iter,
        );
        new.node_ws = Some(old_el_ws);
    }
    new.node_ws.as_ref()
}

pub(crate) fn patch_els<'a, Ms, Mdl, INodes, GMs, OI, NI>(
    document: &Document,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, INodes, GMs>,
    old_el_ws: &web_sys::Node,
    old_children_iter: OI,
    new_children_iter: NI,
) where
    INodes: IntoNodes<Ms>,
    OI: ExactSizeIterator<Item = Node<Ms>>,
    NI: ExactSizeIterator<Item = &'a mut Node<Ms>>,
{
    let mut old_children_iter = old_children_iter.peekable();
    let mut new_children_iter = new_children_iter.peekable();
    let mut last_visited_node: Option<web_sys::Node> = None;

    // Not using .zip() here to make sure we don't miss any of the children when one array is
    // longer than the other.
    while let (Some(_), Some(_)) = (old_children_iter.peek(), new_children_iter.peek()) {
        let child_old = old_children_iter.next().unwrap();
        let child_new = new_children_iter.next().unwrap();

        // Don't compare equality here; we do that at the top of this function
        // in the recursion.
        if let Some(new_el_ws) = patch(
            document,
            child_old,
            child_new,
            old_el_ws,
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
        virtual_dom_bridge::assign_ws_nodes(document, child_new);

        match child_new {
            Node::Element(child_new_el) => {
                virtual_dom_bridge::attach_el_and_children(child_new_el, old_el_ws, mailbox);
            }
            Node::Text(child_new_text) => {
                virtual_dom_bridge::attach_text_node(child_new_text, old_el_ws);
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
                virtual_dom_bridge::remove_node(&child_ws, old_el_ws);
                child_el.node_ws.replace(child_ws);
            }
            Node::Text(mut child_text) => {
                let child_ws = child_text.node_ws.take().expect("Missing child node_ws");
                virtual_dom_bridge::remove_node(&child_ws, old_el_ws);
                child_text.node_ws.replace(child_ws);
            }
            Node::Empty => (),
        }
    }
}

// Reduces code repetition
fn add_el_helper<Ms>(
    new: &mut El<Ms>,
    parent: &web_sys::Node,
    next_node: Option<web_sys::Node>,
    mailbox: &Mailbox<Ms>,
) {
    virtual_dom_bridge::attach_children(new, mailbox);
    let new_ws = new
        .node_ws
        .take()
        .expect("Missing websys el when patching Text to Element");
    virtual_dom_bridge::insert_node(&new_ws, parent, next_node);

    for ref_ in &mut new.refs {
        ref_.set(new_ws.clone());
    }

    new.event_handler_manager
        .attach_listeners(new_ws.clone(), None, mailbox);

    new.node_ws.replace(new_ws);
}

/// Routes patching through different channels, depending on the Node variant
/// of old and new.
pub(crate) fn patch<'a, Ms, Mdl, INodes: IntoNodes<Ms>, GMs>(
    document: &Document,
    old: Node<Ms>,
    new: &'a mut Node<Ms>,
    parent: &web_sys::Node,
    next_node: Option<web_sys::Node>,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, INodes, GMs>,
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

                    virtual_dom_bridge::replace_child(new_node_ws, &old_node_ws, parent);
                    new_text.node_ws.as_ref()
                }
                Node::Empty => {
                    let old_el_ws = old_el
                        .node_ws
                        .take()
                        .expect("old el_ws missing when patching Element to Empty");
                    virtual_dom_bridge::remove_node(&old_el_ws, parent);
                    None
                }
            }
        }
        Node::Empty => {
            // If the old node's empty, assign and attach web_sys nodes.
            virtual_dom_bridge::assign_ws_nodes(document, new);
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
                    virtual_dom_bridge::insert_node(new_node_ws, parent, next_node);
                    new_text.node_ws.as_ref()
                }
                // If new and old are empty, we don't need to do anything.
                Node::Empty => None,
            }
        }
        Node::Text(mut old_text) => {
            virtual_dom_bridge::assign_ws_nodes(document, new);
            match new {
                Node::Element(new_el) => {
                    add_el_helper(new_el, parent, next_node, mailbox);

                    virtual_dom_bridge::remove_node(
                        &old_text.node_ws.expect("Can't find node from Text"),
                        parent,
                    );
                    new_el.node_ws.as_ref()
                }
                Node::Empty => {
                    virtual_dom_bridge::remove_node(
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
