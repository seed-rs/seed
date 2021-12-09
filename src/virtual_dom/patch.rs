//! This module contains code related to patching the VDOM. It can be considered
//! a subset of the `vdom` module.

use super::{El, IntoNodes, Mailbox, Node, Text};
use crate::app::App;
use crate::browser::dom::virtual_dom_bridge;
use web_sys::Document;

mod patch_gen;
use patch_gen::{PatchCommand, PatchGen};

// We assume that when we run this, the new vdom doesn't have assigned `web_sys::Node`s -
// assign them here when we create them.
// @TODO: "Split" `Node` into 2 structs - one without native nodes and one with them (?).

fn append_el<'a, Ms>(
    document: &Document,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    mailbox: &Mailbox<Ms>,
) {
    virtual_dom_bridge::assign_ws_nodes_to_el(document, new);
    virtual_dom_bridge::attach_el_and_children(new, parent, mailbox);
}

fn append_text<'a>(document: &Document, new: &'a mut Text, parent: &web_sys::Node) {
    virtual_dom_bridge::assign_ws_nodes_to_text(document, new);
    virtual_dom_bridge::attach_text_node(new, parent);
}

fn insert_el<'a, Ms>(
    document: &Document,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    next_node: web_sys::Node,
    mailbox: &Mailbox<Ms>,
) {
    virtual_dom_bridge::assign_ws_nodes_to_el(document, new);
    virtual_dom_bridge::attach_children(new, mailbox);
    let new_node = new
        .node_ws
        .take()
        .expect("Missing websys el when patching Text to Element");
    virtual_dom_bridge::insert_node(&new_node, parent, Some(next_node));

    for ref_ in &mut new.refs {
        ref_.set(new_node.clone());
    }

    new.event_handler_manager
        .attach_listeners(new_node.clone(), None, mailbox);

    new.node_ws.replace(new_node);
}

fn insert_text<'a>(
    document: &Document,
    new: &'a mut Text,
    parent: &web_sys::Node,
    next_node: web_sys::Node,
) {
    virtual_dom_bridge::assign_ws_nodes_to_text(document, new);
    let new_node_ws = new
        .node_ws
        .as_ref()
        .expect("new_node_ws missing when patching Empty to Text");
    virtual_dom_bridge::insert_node(new_node_ws, parent, Some(next_node));
}

fn patch_el<'a, Ms, Mdl, INodes>(
    document: &Document,
    mut old: El<Ms>,
    new: &'a mut El<Ms>,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, INodes>,
) where
    INodes: IntoNodes<Ms>,
{
    // At this step, we already assume we have the right element with matching namespace, tag and
    // el_key - either by entering this func directly for the top-level, or recursively after
    // analyzing children.

    // Assume old el vdom's elements are still attached.
    // @TODO: "Split" `Node` into 2 structs - one without native nodes and one with them (?).

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

fn patch_text(mut old: Text, new: &mut Text) {
    let old_node_ws = old
        .node_ws
        .take()
        .expect("old_node_ws missing when changing text");

    if new != &old {
        old_node_ws.set_text_content(Some(&new.text));
    }
    new.node_ws.replace(old_node_ws);
}

fn replace_by_el<'a, Ms>(
    document: &Document,
    old_node: &web_sys::Node,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    mailbox: &Mailbox<Ms>,
) {
    let new_node = virtual_dom_bridge::make_websys_el(new, document);
    for ref_ in &mut new.refs {
        ref_.set(new_node.clone());
    }
    new.node_ws = Some(new_node);
    for child in &mut new.children {
        virtual_dom_bridge::assign_ws_nodes(document, child);
    }
    virtual_dom_bridge::attach_el_and_children(new, parent, mailbox);

    let new_ws = new.node_ws.as_ref().expect("Missing websys el");
    virtual_dom_bridge::replace_child(new_ws, old_node, parent);
}

fn replace_by_text<'a>(
    document: &Document,
    old_node: &web_sys::Node,
    new: &'a mut Text,
    parent: &web_sys::Node,
) {
    virtual_dom_bridge::assign_ws_nodes_to_text(document, new);
    let new_node_ws = new
        .node_ws
        .as_ref()
        .expect("old el_ws missing when replacing with text node");

    virtual_dom_bridge::replace_child(new_node_ws, old_node, parent);
}

fn replace_el_by_el<'a, Ms>(
    document: &Document,
    mut old: El<Ms>,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    mailbox: &Mailbox<Ms>,
) {
    let old_node = old
        .node_ws
        .take()
        .expect("old el_ws missing when replacing element with new element");
    replace_by_el(document, &old_node, new, parent, mailbox);
}

fn replace_el_by_text<'a, Ms>(
    document: &Document,
    mut old: El<Ms>,
    new: &'a mut Text,
    parent: &web_sys::Node,
) {
    let old_node = old
        .node_ws
        .take()
        .expect("old el_ws missing when replacing element with text node");
    replace_by_text(document, &old_node, new, parent);
}

fn replace_text_by_el<'a, Ms>(
    document: &Document,
    mut old: Text,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    mailbox: &Mailbox<Ms>,
) {
    let old_node = old
        .node_ws
        .take()
        .expect("old el_ws missing when replacing text node with element");
    replace_by_el(document, &old_node, new, parent, mailbox);
}

fn remove_el<Ms>(mut old: El<Ms>, parent: &web_sys::Node) {
    let old_node = old.node_ws.take().expect("Missing child node_ws");
    virtual_dom_bridge::remove_node(&old_node, parent);
    old.node_ws.replace(old_node);
}

fn remove_text(mut old: Text, parent: &web_sys::Node) {
    let old_node = old.node_ws.take().expect("Missing child node_ws");
    virtual_dom_bridge::remove_node(&old_node, parent);
    old.node_ws.replace(old_node);
}

pub(crate) fn patch_els<'a, Ms, Mdl, INodes, OI, NI>(
    document: &Document,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, INodes>,
    old_el_ws: &web_sys::Node,
    old_children_iter: OI,
    new_children_iter: NI,
) where
    INodes: IntoNodes<Ms>,
    OI: Iterator<Item = Node<Ms>>,
    NI: Iterator<Item = &'a mut Node<Ms>>,
{
    for command in PatchGen::new(old_children_iter, new_children_iter) {
        match command {
            PatchCommand::AppendEl { el_new } => append_el(document, el_new, old_el_ws, mailbox),
            PatchCommand::AppendText { text_new } => append_text(document, text_new, old_el_ws),
            PatchCommand::InsertEl { el_new, next_node } => {
                insert_el(document, el_new, old_el_ws, next_node, mailbox);
            }
            PatchCommand::InsertText {
                text_new,
                next_node,
            } => insert_text(document, text_new, old_el_ws, next_node),
            PatchCommand::PatchEl { el_old, el_new } => {
                patch_el(document, el_old, el_new, mailbox, app);
            }
            PatchCommand::PatchText { text_old, text_new } => patch_text(text_old, text_new),
            PatchCommand::ReplaceElByEl { el_old, el_new } => {
                replace_el_by_el(document, el_old, el_new, old_el_ws, mailbox);
            }
            PatchCommand::ReplaceTextByEl { text_old, el_new } => {
                replace_text_by_el(document, text_old, el_new, old_el_ws, mailbox);
            }
            PatchCommand::ReplaceElByText { el_old, text_new } => {
                replace_el_by_text(document, el_old, text_new, old_el_ws);
            }
            PatchCommand::RemoveEl { el_old } => remove_el(el_old, old_el_ws),
            PatchCommand::RemoveText { text_old } => remove_text(text_old, old_el_ws),
        };
    }
}

/// Routes patching through different channels, depending on the Node variant of old and new.
/// Tries to updates the `old` node to become the `new` one.
#[cfg(test)]
pub(crate) fn patch<'a, Ms, Mdl, INodes: IntoNodes<Ms>>(
    document: &Document,
    old: Node<Ms>,
    new: &'a mut Node<Ms>,
    parent: &web_sys::Node,
    next_node: Option<web_sys::Node>,
    mailbox: &Mailbox<Ms>,
    app: &App<Ms, Mdl, INodes>,
) -> Option<&'a web_sys::Node> {
    // Old_el_ws is what we're patching, with items from the new vDOM el; or replacing.
    // We go through each combination of new and old variants to determine how to patch.
    // We return the resulting web_sys node for assistance in inserting subsequent
    // ones in the right place.

    // We assume that when we run this, the new vdom doesn't have assigned `web_sys::Node`s -
    // assign them here when we create them.
    // @TODO: "Split" `Node` into 2 structs - one without native nodes and one with them (?).

    // @TODO Do we realy need this function? This function could be replaced by calling
    // `patch_els` with `std::iter::once` for old and new nodes.
    match old {
        Node::Element(old_el) => match new {
            Node::Element(new_el) => {
                if patch_gen::el_can_be_patched(&old_el, new_el) {
                    patch_el(document, old_el, new_el, mailbox, app)
                } else {
                    replace_el_by_el(document, old_el, new_el, parent, mailbox)
                }
            }
            Node::Text(new_text) => replace_el_by_text(document, old_el, new_text, parent),
            Node::Empty => remove_el(old_el, parent),
            Node::NoChange => {
                *new = Node::Element(old_el);
            }
        },
        Node::Empty => {
            match new {
                Node::Element(new_el) => {
                    if let Some(next) = next_node {
                        insert_el(document, new_el, parent, next, mailbox)
                    } else {
                        append_el(document, new_el, parent, mailbox)
                    }
                }
                Node::Text(new_text) => {
                    if let Some(next) = next_node {
                        insert_text(document, new_text, parent, next)
                    } else {
                        append_text(document, new_text, parent)
                    }
                }
                // If new and old are empty, we don't need to do anything.
                Node::Empty => (),
                Node::NoChange => {
                    *new = old;
                }
            }
        }
        Node::Text(old_text) => {
            virtual_dom_bridge::assign_ws_nodes(document, new);
            match new {
                Node::Element(new_el) => {
                    replace_text_by_el(document, old_text, new_el, parent, mailbox)
                }
                Node::Empty => remove_text(old_text, parent),
                Node::Text(new_text) => patch_text(old_text, new_text),
                Node::NoChange => {
                    *new = Node::Text(old_text);
                }
            }
        }
        Node::NoChange => panic!("Node::NoChange cannot be an old VDOM node!"),
    };
    new.node_ws()
}
