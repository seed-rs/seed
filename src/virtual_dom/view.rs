use super::{El, Node};

pub trait View<Ms: 'static> {
    fn els(self) -> Vec<Node<Ms>>;
}

impl<Ms> View<Ms> for El<Ms> {
    fn els(self) -> Vec<Node<Ms>> {
        vec![Node::Element(self)]
    }
}

impl<Ms> View<Ms> for Vec<El<Ms>> {
    fn els(self) -> Vec<Node<Ms>> {
        self.into_iter().map(Node::Element).collect()
    }
}

impl<Ms: 'static> View<Ms> for Node<Ms> {
    fn els(self) -> Vec<Node<Ms>> {
        vec![self]
    }
}

impl<Ms: 'static> View<Ms> for Vec<Node<Ms>> {
    fn els(self) -> Vec<Node<Ms>> {
        self
    }
}
