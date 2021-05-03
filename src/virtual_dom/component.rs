use super::Node;

pub trait Component<Ms> {
    fn view(&self) -> Node<Ms>;
}

#[allow(clippy::needless_pass_by_value)]
pub fn instantiate<Ms, C: Component<Ms>>(component: C) -> Node<Ms> {
    // TODO: This is where we'd create a boundary node and a state container
    // that can then either be passed to `render` to be populated, or capture
    // hook calls indirectly like React does.
    //
    // The boundary node will own the state container and remember the component
    // configuration, so that it can do a local re-render when triggered by a
    // hook.
    component.view()
}
