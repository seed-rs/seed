use super::Node;

pub trait IntoNodes<Ms> {
    fn into_nodes(self) -> Vec<Node<Ms>>;
}

impl<Ms> IntoNodes<Ms> for Node<Ms> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        vec![self]
    }
}

impl<Ms> IntoNodes<Ms> for Vec<Node<Ms>> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        self
    }
}
