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
//! Suppose we have old and new child nodes with `el_keys`:
//! ```
//! old: [a] [b] [c] [d] [e] [f] [g] [h]
//! new: [a] [d] [e] [b] [c] [x] [f] [y]
//! ```
//!
//! The algorithm starts by taking nodes form the source iterators one by one in turn (new and old)
//! and putting them in the corresponding queues:
//! ```
//! old: [a]
//! new: [a]
//! ```
//!
//! Now we have the matching key in queues. The algorithm takes nodes from the queues and yields
//! `patch [a] by [a]` command.
//!
//! But then nodes become diverse:
//! ```
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
//! ```
//! old: [c]
//! new: [c]
//! ```
//!
//! Then nodes again become diverse:
//! ```
//! old: [d] [e] [f]
//! new: [x] [f] [y]
//! ```
//!
//! The algorithm stops when finds the matching key `[f]` and yields three commands:
//! `replace [d] by [x]`, `remove [e]`, `patch [f] by [f]`.
//!
//! At this point the source iterator for the new nodes has been exhausted and the algorithm
//! continues to take only old nodes.
//! ```
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
/// `find_matching` stores these keys in `BTreeSet` to check is the key already seen.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PatchKey {
    Element {
        namespace: Option<Namespace>,
        tag: Tag,
        el_key: Option<ElKey>,
    },
    Text,
}

fn make_key<Ms: 'static>(node: &Node<Ms>) -> Option<PatchKey> {
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

    /// Decides how to produce the next command depending form the internal state.
    fn next_command(&mut self) -> Option<PatchCommand<'a, Ms>> {
        if self.keyed_mode {
            if self.matching_key.is_none()
                && (self.old_children_iter.peek().is_some()
                    || self.new_children_iter.peek().is_some())
            {
                self.matching_key = find_matching(
                    &mut self.old_children_iter,
                    &mut self.old_children,
                    &mut self.new_children_iter,
                    &mut self.new_children,
                );
            }
            if self.matching_key.is_some() {
                self.yield_keyed()
            } else {
                self.yield_remaining()
            }
        } else {
            self.yield_keyless()
        }
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
                if child_old.el_key().is_some() || child_new.el_key().is_some() {
                    // switch to to keyed mode
                    let key_old = make_key(&child_old);
                    let key_new = make_key(child_new);
                    if key_old == key_new {
                        self.matching_key = key_new;
                    }
                    if child_old.is_el() || child_old.is_text() {
                        self.old_children.push_back(child_old);
                    }
                    if child_new.is_el() || child_new.is_text() {
                        self.new_children.push_back(child_new);
                    }
                    self.keyed_mode = true;
                    self.next_command()
                } else {
                    self.patch_or_replace(child_old, child_new)
                }
            }
            (None, Some(child_new)) => self.append(child_new),
            (Some(child_old), None) => self.remove(child_old),
            (None, None) => None,
        }
    }

    /// Produces commands from children stored in the `old_children` and the `new_children` queues
    /// until child key is qual to `matching_key`, then returns the `PatchEl` or `ReplaceElByEl`
    /// command.
    fn yield_keyed(&mut self) -> Option<PatchCommand<'a, Ms>> {
        match (
            self.matching_child_old.as_ref(),
            self.matching_child_new.as_ref(),
        ) {
            // The matching key are reached for both the old and the new children.
            (Some(_), Some(_)) => {
                self.matching_key.take();
                let child_old = self.matching_child_old.take().unwrap();
                let child_new = self.matching_child_new.take().unwrap();
                self.patch_or_replace(child_old, child_new)
            }
            // The matching key is reached for the old children.
            (Some(child_old), None) => {
                let child_new = self
                    .new_children
                    .pop_back()
                    .expect("No node with a matching key");
                if make_key(child_new) == self.matching_key {
                    self.matching_child_new.replace(child_new);
                    self.yield_keyed()
                } else {
                    let next_node = child_old
                        .node_ws()
                        .expect("Old node not connected to web_sys node")
                        .clone();
                    self.insert(child_new, next_node)
                }
            }
            // The matching key is reached for the new children.
            (None, Some(_)) => {
                let child_old = self
                    .old_children
                    .pop_back()
                    .expect("No node with a matching key");
                if make_key(&child_old) == self.matching_key {
                    self.matching_child_old.replace(child_old);
                    self.yield_keyed()
                } else {
                    self.remove(child_old)
                }
            }
            // No matching keys is reached.
            (None, None) => {
                if let (Some(child_old), Some(child_new)) =
                    (self.old_children.pop_back(), self.new_children.pop_back())
                {
                    let key_old = make_key(&child_old);
                    let key_new = make_key(child_new);
                    if key_old == self.matching_key && key_new == self.matching_key {
                        self.matching_child_old.replace(child_old);
                        self.matching_child_new.replace(child_new);
                        self.yield_keyed()
                    } else if key_old == self.matching_key {
                        let next_node = child_old.node_ws().unwrap().clone();
                        self.matching_child_old.replace(child_old);
                        self.insert(child_new, next_node)
                    } else if key_new == self.matching_key {
                        self.matching_child_new.replace(child_new);
                        self.remove(child_old)
                    } else {
                        self.patch_or_replace(child_old, child_new)
                    }
                } else {
                    // if the matching key is set then both the old and the new children queues has
                    // node with this key.
                    unreachable!("No node with a matching key")
                }
            }
        }
    }

    /// Takes a pair of the remaining children stored in the queues and returns the command.
    fn yield_remaining(&mut self) -> Option<PatchCommand<'a, Ms>> {
        let maybe_child_old = self.old_children.pop_back();
        let maybe_child_new = self.new_children.pop_back();

        match (maybe_child_old, maybe_child_new) {
            (Some(child_old), Some(child_new)) => self.patch_or_replace(child_old, child_new),
            (Some(child_old), None) => self.remove(child_old),
            (None, Some(child_new)) => self.append(child_new),
            (None, None) => None,
        }
    }

    fn append(&mut self, child_new: &'a mut Node<Ms>) -> Option<PatchCommand<'a, Ms>> {
        match child_new {
            Node::Element(el_new) => Some(PatchCommand::AppendEl { el_new }),
            Node::Text(text_new) => Some(PatchCommand::AppendText { text_new }),
            Node::Empty => self.next_command(),
        }
    }

    fn insert(
        &mut self,
        child_new: &'a mut Node<Ms>,
        next_node: web_sys::Node,
    ) -> Option<PatchCommand<'a, Ms>> {
        match child_new {
            Node::Element(el_new) => Some(PatchCommand::InsertEl { el_new, next_node }),
            Node::Text(text_new) => Some(PatchCommand::InsertText {
                text_new,
                next_node,
            }),
            Node::Empty => self.next_command(),
        }
    }

    fn patch_or_replace(
        &mut self,
        child_old: Node<Ms>,
        child_new: &'a mut Node<Ms>,
    ) -> Option<PatchCommand<'a, Ms>> {
        match child_old {
            Node::Element(el_old) => match child_new {
                Node::Element(el_new) => {
                    if el_can_be_patched(&el_old, el_new) {
                        Some(PatchCommand::PatchEl { el_old, el_new })
                    } else {
                        Some(PatchCommand::ReplaceElByEl { el_old, el_new })
                    }
                }
                Node::Text(text_new) => Some(PatchCommand::ReplaceElByText { el_old, text_new }),
                Node::Empty => Some(PatchCommand::RemoveEl { el_old }),
            },
            Node::Text(text_old) => match child_new {
                Node::Element(el_new) => Some(PatchCommand::ReplaceTextByEl { text_old, el_new }),
                Node::Text(text_new) => Some(PatchCommand::PatchText { text_old, text_new }),
                Node::Empty => Some(PatchCommand::RemoveText { text_old }),
            },
            Node::Empty => match child_new {
                Node::Element(el_new) => {
                    if let Some(next_node) =
                        find_next_node_ws(&mut self.old_children_iter, &mut self.old_children)
                    {
                        Some(PatchCommand::InsertEl { el_new, next_node })
                    } else {
                        Some(PatchCommand::AppendEl { el_new })
                    }
                }
                Node::Text(text_new) => {
                    if let Some(next_node) =
                        find_next_node_ws(&mut self.old_children_iter, &mut self.old_children)
                    {
                        Some(PatchCommand::InsertText {
                            text_new,
                            next_node,
                        })
                    } else {
                        Some(PatchCommand::AppendText { text_new })
                    }
                }
                Node::Empty => self.next_command(),
            },
        }
    }

    fn remove(&mut self, child_old: Node<Ms>) -> Option<PatchCommand<'a, Ms>> {
        match child_old {
            Node::Element(el_old) => Some(PatchCommand::RemoveEl { el_old }),
            Node::Text(text_old) => Some(PatchCommand::RemoveText { text_old }),
            Node::Empty => self.next_command(),
        }
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

/// Takes children form the source iterators (new and old) and puts them in the
/// corresponding queues.
/// Stops when the key of the new child matches to any key of the previously seen old children
/// or the key of the old child matches to any key of the previously seen new children.
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
        .filter_map(|node| make_key(node.borrow()))
        .collect();
    //  And one for the new children.
    let mut seen_new_keys: BTreeSet<_> = new_children
        .iter()
        .filter_map(|node| make_key(node.borrow()))
        .collect();

    loop {
        // Fill the old/new children queues and keep the queue lenghts same.
        if old_children_iter.peek().is_some()
            && (new_children_iter.peek().is_none() || new_children.len() > old_children.len())
        {
            if let Some(key) = fetch_next_item(old_children_iter, old_children)
                .map(|child| make_key(child.borrow()))
                .flatten()
            {
                if seen_new_keys.contains(&key) {
                    // Stop and return matching key.
                    break Some(key);
                } else {
                    seen_old_keys.insert(key);
                }
            }
        } else if new_children_iter.peek().is_some() {
            if let Some(key) = fetch_next_item(new_children_iter, new_children)
                .map(|child| make_key(child.borrow()))
                .flatten()
            {
                if seen_old_keys.contains(&key) {
                    // Stop and return matching key.
                    break Some(key);
                } else {
                    seen_new_keys.insert(key);
                }
            }
        } else {
            // No matching keys.
            break None;
        }
    }
}

/// Searches for the next node with the bound `web_sys::Node` and returns a clone of that
/// `web_sys::Node` or None if there is no such node.
fn find_next_node_ws<I, N, Ms>(
    children_iter: &mut Peekable<I>,
    children: &mut VecDeque<N>,
) -> Option<web_sys::Node>
where
    I: Iterator<Item = N>,
    N: Borrow<Node<Ms>>,
    Ms: 'static,
{
    // Search in the stored children frist.
    children
        .iter()
        .find_map(|child| child.borrow().node_ws().cloned())
        // Consume source iterator if there is no stored child with the bound node.
        .or_else(|| loop {
            if let Some(child) =
                fetch_next_item(children_iter, children)
            {
                let maybe_ws = child.borrow().node_ws().cloned();
                if maybe_ws.is_some() {
                    break maybe_ws;
                }
            } else {
                break None;
            }
        })
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
