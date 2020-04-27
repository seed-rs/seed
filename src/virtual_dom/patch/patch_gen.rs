//!
//! This module decides how to update nodes.
//!
//! ### Keyless mode
//!
//! The patch algorithm starts with keyless mode.
//! In this mode, a pair of nodes is taken: old and new.
//! - if the node type, tag, and namespace are equal, the old node is updated with the new one.
//! - if the nodes are different, then the new node replaces the old one.
//! - If the old node is empty, the new node is inserted before the next non-empty old node.
//! - If the new node is empty, the old node is deleted.
//! - All remaining new nodes are added to the end.
//! - All remaining old nodes are deleted.
//!
//! As soon as the old or new node has a key, the algorithm switches to the key mode.
//!
//! ### Keyed mode
//!
//! Suppose we have old and new child nodes with `el_key`s:
//! ```text
//! old: [a] [b] [c] [d] [e] [f] [g] [h]
//! new: [a] [d] [e] [b] [c] [x] [f] [y]
//! ```
//!
//! The algorithm starts by taking nodes from the source iterators one by one in turn (new and old)
//! and putting them in the corresponding queues:
//! ```text
//! old: [a]
//! new: [a]
//! ```
//!
//! Now we have the matching key in queues. The algorithm takes nodes from the queues and yields
//! `patch [a] by [a]` command.
//!
//! But then nodes become diverse:
//! ```text
//! old: [b] [c]
//! new: [d] [e] [b]
//! ```
//!
//! As soon as the algorithm finds the matching key `[b]` it yields three commands:
//! `insert [d] before old [b]`, `insert [e] before old [b]` and then `patch [b] by [b]`.
//! Old node `[c]` remains in queue.
//!
//! The algorithm continues to fill queues and stops with matching nodes `[c]`. Then it issues `patch`
//! command:
//! ```text
//! old: [c]
//! new: [c]
//! ```
//!
//! Then nodes again become diverse:
//! ```text
//! old: [d] [e] [f]
//! new: [x] [f] [y]
//! ```
//!
//! The algorithm stops when finds the matching key `[f]` and yields three commands:
//! `replace [d] by [x]`, `remove [e]`, `patch [f] by [f]`.
//!
//! At this point the source iterator for the new nodes has been exhausted and the algorithm
//! continues to take only old nodes.
//! ```text
//! old: [g] [h]
//! new: [y]
//! ```
//!
//! At this point both source iterators are exhausted and the algorithm yields:
//! `replace [g] by [y]` and `remove [h]`.
//! The append command means append as a last children.
//!

use crate::browser::dom::Namespace;
use crate::virtual_dom::{El, ElKey, Node, Tag, Text};
use std::borrow::Borrow;
use std::collections::{BTreeSet, VecDeque};
use std::iter::Peekable;

#[allow(clippy::large_enum_variant)]
pub(crate) enum PatchCommand<'a, Ms: 'static> {
    AppendEl {
        el_new: &'a mut El<Ms>,
    },
    AppendText {
        text_new: &'a mut Text,
    },
    InsertEl {
        el_new: &'a mut El<Ms>,
        next_node: web_sys::Node,
    },
    InsertText {
        text_new: &'a mut Text,
        next_node: web_sys::Node,
    },
    PatchEl {
        el_old: El<Ms>,
        el_new: &'a mut El<Ms>,
    },
    PatchText {
        text_old: Text,
        text_new: &'a mut Text,
    },
    ReplaceElByEl {
        el_old: El<Ms>,
        el_new: &'a mut El<Ms>,
    },
    ReplaceElByText {
        el_old: El<Ms>,
        text_new: &'a mut Text,
    },
    ReplaceTextByEl {
        text_old: Text,
        el_new: &'a mut El<Ms>,
    },
    RemoveEl {
        el_old: El<Ms>,
    },
    RemoveText {
        text_old: Text,
    },
}

/// `PatchKey` used to compare nodes during patching.
///
/// A function `find_matching` stores these keys to check if the key has already been seen.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PatchKey {
    Element {
        namespace: Option<Namespace>,
        tag: Tag,
        el_key: Option<ElKey>,
    },
    Text,
}

impl PatchKey {
    fn new<Ms: 'static>(node: &Node<Ms>) -> Option<Self> {
        match node {
            Node::Element(el) => Some(PatchKey::Element {
                namespace: el.namespace.clone(),
                tag: el.tag.clone(),
                el_key: el.key.clone(),
            }),
            Node::Text(_) => Some(PatchKey::Text),
            Node::Empty => None,
        }
    }
}

/// This is a command generator.
/// See the module documenation for brief description of how this works.
pub(crate) struct PatchGen<'a, Ms, OI, NI>
where
    Ms: 'static,
    OI: Iterator<Item = Node<Ms>>,
    NI: Iterator<Item = &'a mut Node<Ms>>,
{
    old_children_iter: Peekable<OI>,
    new_children_iter: Peekable<NI>,
    old_children: VecDeque<Node<Ms>>,
    new_children: VecDeque<&'a mut Node<Ms>>,
    matching_child_old: Option<OI::Item>,
    matching_child_new: Option<NI::Item>,
    matching_key: Option<PatchKey>,
    keyed_mode: bool,
}

impl<'a, Ms, OI, NI> PatchGen<'a, Ms, OI, NI>
where
    Ms: 'static,
    OI: Iterator<Item = Node<Ms>>,
    NI: Iterator<Item = &'a mut Node<Ms>>,
{
    /// Creates the new `PatchGen` instance from the source iterators.
    pub fn new(old_children_iter: OI, new_children_iter: NI) -> Self {
        Self {
            old_children_iter: old_children_iter.peekable(),
            new_children_iter: new_children_iter.peekable(),
            old_children: VecDeque::new(),
            new_children: VecDeque::new(),
            matching_child_old: None,
            matching_child_new: None,
            matching_key: None,
            keyed_mode: false,
        }
    }

    /// Decides what command to produce according to the internal state.
    fn next_command(&mut self) -> Option<PatchCommand<'a, Ms>> {
        if !self.keyed_mode {
            return self.yield_keyless();
        }
        // Matching `PatchKey` has been already found.
        if self.matching_key.is_some() {
            return self.yield_keyed();
        }
        // Try to find matching key if at least one iterator has some nodes.
        if self.old_children_iter.peek().is_some() || self.new_children_iter.peek().is_some() {
            self.matching_key = find_matching(
                &mut self.old_children_iter,
                &mut self.old_children,
                &mut self.new_children_iter,
                &mut self.new_children,
            );
            // Matching key has been found.
            if self.matching_key.is_some() {
                return self.yield_keyed();
            }
        }
        self.yield_remaining()
    }

    /// Takes a pair of old and new children from source iterators and decides how to update the
    /// old child by the new one.
    /// Sets `keyed_mode` to true and calls `yield_keyed` as soon as any child has an element key.
    fn yield_keyless(&mut self) -> Option<PatchCommand<'a, Ms>> {
        // Take a pair of old/new children but skip if both are `Some(Node::Empty)`.
        let (child_old, child_new) = loop {
            // First consume the children stored in the queue.
            // When old child is `Empty` we call `find_next_node_ws` which
            // moves some children from the source iterator to the queue.
            let old = self
                .old_children
                .pop_back()
                .or_else(|| self.old_children_iter.next());
            let new = self.new_children_iter.next();
            // We should not issue any command if both the old and the new nodes are `Empty`.
            if let (Some(Node::Empty), Some(Node::Empty)) = (&old, &new) {
                continue;
            }
            break (old, new);
        };

        match (child_old, child_new) {
            (Some(child_old), Some(child_new)) => {
                if child_old.el_key().is_none() && child_new.el_key().is_none() {
                    return self.patch_or_replace(child_old, child_new);
                }

                // Permanent switch to keyed mode.
                self.keyed_mode = true;

                let key_old = PatchKey::new(&child_old);
                let key_new = PatchKey::new(child_new);
                if key_old == key_new {
                    self.matching_key = key_new;
                }
                if !child_old.is_empty() {
                    self.old_children.push_back(child_old);
                }
                if !child_new.is_empty() {
                    self.new_children.push_back(child_new);
                }
                self.next_command()
            }
            (None, Some(child_new)) => self.append(child_new),
            (Some(child_old), None) => self.remove(child_old),
            (None, None) => None,
        }
    }

    /// Produces commands from children stored in the `old_children` and the `new_children` queues
    /// until the child key is equal to `matching_key`, then returns the `PatchEl` or `ReplaceElByEl`
    /// command.
    ///
    /// `self.matching_key` has to be set before calling this method.
    fn yield_keyed(&mut self) -> Option<PatchCommand<'a, Ms>> {
        // `self.matching_child_old` and `self.matching_child_new` are set only in this method.
        // Therefore the first matching arm is always `(None, None)`.
        match (
            self.matching_child_old.as_ref(),
            self.matching_child_new.as_ref(),
        ) {
            // No nodes with the matching key have been found.
            (None, None) => {
                // If the matching key is set then both the old and the new children queues
                // have a node with this key.
                let child_old = self
                    .old_children
                    .pop_back()
                    .expect("old child from the queue");
                let child_new = self
                    .new_children
                    .pop_back()
                    .expect("new child from the queue");

                let key_old = PatchKey::new(&child_old);
                let key_new = PatchKey::new(child_new);

                if key_old == self.matching_key && key_new == self.matching_key {
                    self.matching_child_old = Some(child_old);
                    self.matching_child_new = Some(child_new);
                    return self.yield_keyed();
                }
                if key_old == self.matching_key {
                    let next_node = child_old.node_ws().unwrap().clone();
                    self.matching_child_old = Some(child_old);
                    return self.insert(child_new, next_node);
                }
                if key_new == self.matching_key {
                    self.matching_child_new = Some(child_new);
                    return self.remove(child_old);
                }
                self.patch_or_replace(child_old, child_new)
            }
            // An old node with the matching key has been found in the queue.
            (Some(child_old), None) => {
                let child_new = self
                    .new_children
                    .pop_back()
                    .expect("node with a matching key");

                if PatchKey::new(child_new) == self.matching_key {
                    self.matching_child_new = Some(child_new);
                    return self.yield_keyed();
                }
                let next_node = child_old
                    .node_ws()
                    .expect("old node connected to web_sys node")
                    .clone();
                self.insert(child_new, next_node)
            }
            // A new node with the matching key has been found in the queue.
            (None, Some(_)) => {
                let child_old = self
                    .old_children
                    .pop_back()
                    .expect("node with a matching key");

                if PatchKey::new(&child_old) == self.matching_key {
                    self.matching_child_old = Some(child_old);
                    return self.yield_keyed();
                }
                self.remove(child_old)
            }
            // An old and a new node with the matching key have been found in queues.
            (Some(_), Some(_)) => {
                // We have found the matching node pair, we no longer need the key.
                self.matching_key = None;
                let child_old = self.matching_child_old.take().unwrap();
                let child_new = self.matching_child_new.take().unwrap();
                self.patch_or_replace(child_old, child_new)
            }
        }
    }

    /// Takes a pair of the remaining children stored in the queues and returns the command.
    fn yield_remaining(&mut self) -> Option<PatchCommand<'a, Ms>> {
        match (self.old_children.pop_back(), self.new_children.pop_back()) {
            (Some(child_old), Some(child_new)) => self.patch_or_replace(child_old, child_new),
            (Some(child_old), None) => self.remove(child_old),
            (None, Some(child_new)) => self.append(child_new),
            (None, None) => None,
        }
    }

    fn append(&mut self, child_new: &'a mut Node<Ms>) -> Option<PatchCommand<'a, Ms>> {
        Some(match child_new {
            Node::Element(el_new) => PatchCommand::AppendEl { el_new },
            Node::Text(text_new) => PatchCommand::AppendText { text_new },
            Node::Empty => return self.next_command(),
        })
    }

    fn insert(
        &mut self,
        child_new: &'a mut Node<Ms>,
        next_node: web_sys::Node,
    ) -> Option<PatchCommand<'a, Ms>> {
        Some(match child_new {
            Node::Element(el_new) => PatchCommand::InsertEl { el_new, next_node },
            Node::Text(text_new) => PatchCommand::InsertText {
                text_new,
                next_node,
            },
            Node::Empty => return self.next_command(),
        })
    }

    fn patch_or_replace(
        &mut self,
        child_old: Node<Ms>,
        child_new: &'a mut Node<Ms>,
    ) -> Option<PatchCommand<'a, Ms>> {
        Some(match child_old {
            Node::Element(el_old) => match child_new {
                Node::Element(el_new) => {
                    if el_can_be_patched(&el_old, el_new) {
                        PatchCommand::PatchEl { el_old, el_new }
                    } else {
                        PatchCommand::ReplaceElByEl { el_old, el_new }
                    }
                }
                Node::Text(text_new) => PatchCommand::ReplaceElByText { el_old, text_new },
                Node::Empty => PatchCommand::RemoveEl { el_old },
            },
            Node::Text(text_old) => match child_new {
                Node::Element(el_new) => PatchCommand::ReplaceTextByEl { text_old, el_new },
                Node::Text(text_new) => PatchCommand::PatchText { text_old, text_new },
                Node::Empty => PatchCommand::RemoveText { text_old },
            },
            Node::Empty => match child_new {
                Node::Element(el_new) => {
                    if let Some(next_node) =
                        find_next_node_ws(&mut self.old_children_iter, &mut self.old_children)
                    {
                        PatchCommand::InsertEl { el_new, next_node }
                    } else {
                        PatchCommand::AppendEl { el_new }
                    }
                }
                Node::Text(text_new) => {
                    if let Some(next_node) =
                        find_next_node_ws(&mut self.old_children_iter, &mut self.old_children)
                    {
                        PatchCommand::InsertText {
                            text_new,
                            next_node,
                        }
                    } else {
                        PatchCommand::AppendText { text_new }
                    }
                }
                Node::Empty => return self.next_command(),
            },
        })
    }

    fn remove(&mut self, child_old: Node<Ms>) -> Option<PatchCommand<'a, Ms>> {
        Some(match child_old {
            Node::Element(el_old) => PatchCommand::RemoveEl { el_old },
            Node::Text(text_old) => PatchCommand::RemoveText { text_old },
            Node::Empty => return self.next_command(),
        })
    }
}

impl<'a, Ms, OI, NI> Iterator for PatchGen<'a, Ms, OI, NI>
where
    Ms: 'static,
    OI: Iterator<Item = Node<Ms>>,
    NI: Iterator<Item = &'a mut Node<Ms>>,
{
    type Item = PatchCommand<'a, Ms>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_command()
    }
}

/// Checks whether the old element can be updated with a new one.
pub(crate) fn el_can_be_patched<Ms>(el_old: &El<Ms>, el_new: &El<Ms>) -> bool {
    // Custom elements can't be patched, because we need to reinit them (Issue #325).
    // @TODO remove this check when #364 will be done.
    el_old.namespace == el_new.namespace
        && el_old.tag == el_new.tag
        && el_old.key == el_new.key
        && !el_new.is_custom()
}

/// Takes children from source iterators (new and old) and puts them in the
/// corresponding queues.
///
/// Stops when:
/// - The key of the new child matches to any key of the previously seen old children.
/// - The key of the old child matches to any key of the previously seen new children.
fn find_matching<OI, NI, ON, NN, Ms>(
    old_children_iter: &mut Peekable<OI>,
    old_children: &mut VecDeque<ON>,
    new_children_iter: &mut Peekable<NI>,
    new_children: &mut VecDeque<NN>,
) -> Option<PatchKey>
where
    OI: Iterator<Item = ON>,
    NI: Iterator<Item = NN>,
    ON: Borrow<Node<Ms>>,
    NN: Borrow<Node<Ms>>,
    Ms: 'static,
{
    // First store all seen keys to the sets.
    // One for the old children.
    let mut seen_old_keys: BTreeSet<_> = old_children
        .iter()
        .filter_map(|node| PatchKey::new(node.borrow()))
        .collect();
    // And one for the new children.
    let mut seen_new_keys: BTreeSet<_> = new_children
        .iter()
        .filter_map(|node| PatchKey::new(node.borrow()))
        .collect();

    while old_children_iter.peek().is_some() || new_children_iter.peek().is_some() {
        // Fill the old/new children queues and keep the same queue lengths.
        let should_pick_old_child = old_children_iter.peek().is_some()
            && (new_children_iter.peek().is_none() || new_children.len() > old_children.len());

        if should_pick_old_child {
            if let Some(key) = fetch_next_item(old_children_iter, old_children)
                .and_then(|child| PatchKey::new(child.borrow()))
            {
                if seen_new_keys.contains(&key) {
                    return Some(key);
                }
                seen_old_keys.insert(key);
            }
        } else if new_children_iter.peek().is_some() {
            if let Some(key) = fetch_next_item(new_children_iter, new_children)
                .and_then(|child| PatchKey::new(child.borrow()))
            {
                if seen_old_keys.contains(&key) {
                    return Some(key);
                }
                seen_new_keys.insert(key);
            }
        }
    }
    None
}

/// Searches for the next node with set `web_sys::Node` and returns a clone of that
/// `web_sys::Node` or `None` if there is no such node.
fn find_next_node_ws<I, N, Ms>(
    children_iter: &mut Peekable<I>,
    children: &mut VecDeque<N>,
) -> Option<web_sys::Node>
where
    I: Iterator<Item = N>,
    N: Borrow<Node<Ms>>,
    Ms: 'static,
{
    // Search in the stored children first.
    if let node_ws @ Some(_) = children.iter().find_map(|child| child.borrow().node_ws()) {
        return node_ws.cloned();
    }
    // Consume the source iterator if there is no stored child with the searched node.
    while let Some(child) = fetch_next_item(children_iter, children) {
        if let node_ws @ Some(_) = child.borrow().node_ws() {
            return node_ws.cloned();
        }
    }
    None
}

/// Fetches the next item from the `source_iter` iterator, pushes this item to the
/// `queue` and returns a reference to this item.
fn fetch_next_item<'a, I, T>(source_iter: &'a mut I, queue: &'a mut VecDeque<T>) -> Option<&'a T>
where
    I: Iterator<Item = T>,
{
    if let Some(item) = source_iter.next() {
        queue.push_front(item);
        queue.front()
    } else {
        None
    }
}
