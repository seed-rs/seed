use super::{IntoNodes, Node};

#[deprecated(
    since = "0.7.0",
    note = "Use [`IntoNodes`](../node/into_nodes/trait.IntoNodes.html) instead."
)]
pub trait View<Ms>: IntoNodes<Ms> {
    fn els(self) -> Vec<Node<Ms>>;
}

impl<Ms> View<Ms> for Node<Ms> {
    fn els(self) -> Vec<Node<Ms>> {
        vec![self]
    }
}

impl<Ms> View<Ms> for Vec<Node<Ms>> {
    fn els(self) -> Vec<Node<Ms>> {
        self
    }
}
