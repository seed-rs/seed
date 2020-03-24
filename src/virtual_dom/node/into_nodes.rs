use super::Node;

/// Items that implement `IntoNodes`:
/// - Can be used in `nodes!`.
/// - Can be returned from `view`.
pub trait IntoNodes<Ms> {
    /// Converts item or items to `Vec<Node<Ms>`.
    fn into_nodes(self) -> Vec<Node<Ms>>;
}

impl<Ms> IntoNodes<Ms> for Node<Ms> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        vec![self]
    }
}

impl<Ms> IntoNodes<Ms> for Option<Node<Ms>> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        self.map(IntoNodes::into_nodes).unwrap_or_default()
    }
}

impl<Ms> IntoNodes<Ms> for Vec<Node<Ms>> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        self
    }
}

impl<Ms> IntoNodes<Ms> for Option<Vec<Node<Ms>>> {
    fn into_nodes(self) -> Vec<Node<Ms>> {
        self.unwrap_or_default()
    }
}
