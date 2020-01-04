use super::{Attrs, El, Listener, Node, Style, Tag, Text};
use crate::browser::dom::lifecycle_hooks::{DidMount, DidUpdate, WillUnmount};

/// `UpdateEl` is used to distinguish arguments in element-creation macros, and handle
/// each type appropriately.
pub trait UpdateEl<T> {
    // T is the type of thing we're updating; eg attrs, style, events etc.
    fn update(self, el: &mut T);
}

impl<Ms> UpdateEl<El<Ms>> for Attrs {
    fn update(self, el: &mut El<Ms>) {
        el.attrs.merge(self);
    }
}

impl<Ms> UpdateEl<El<Ms>> for &Attrs {
    fn update(self, el: &mut El<Ms>) {
        el.attrs.merge(self.clone());
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<Attrs> {
    fn update(self, el: &mut El<Ms>) {
        for at in self {
            el.attrs.merge(at);
        }
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<&Attrs> {
    fn update(self, el: &mut El<Ms>) {
        for at in self {
            el.attrs.merge(at.clone());
        }
    }
}

impl<Ms> UpdateEl<El<Ms>> for Style {
    fn update(self, el: &mut El<Ms>) {
        el.style.merge(self);
    }
}

impl<Ms> UpdateEl<El<Ms>> for &Style {
    fn update(self, el: &mut El<Ms>) {
        el.style.merge(self.clone());
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<Style> {
    fn update(self, el: &mut El<Ms>) {
        for st in self {
            el.style.merge(st);
        }
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<&Style> {
    fn update(self, el: &mut El<Ms>) {
        for st in self {
            el.style.merge(st.clone());
        }
    }
}

impl<Ms> UpdateEl<El<Ms>> for Listener<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.listeners.push(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<Listener<Ms>> {
    fn update(mut self, el: &mut El<Ms>) {
        el.listeners.append(&mut self);
    }
}

impl<Ms> UpdateEl<El<Ms>> for DidMount<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.hooks.did_mount = Some(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for DidUpdate<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.hooks.did_update = Some(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for WillUnmount<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.hooks.will_unmount = Some(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for &str {
    // This, or some other mechanism seems to work for String too... note sure why.
    fn update(self, el: &mut El<Ms>) {
        el.children.push(Node::Text(Text::new(self.to_string())))
    }
}

impl<Ms> UpdateEl<El<Ms>> for El<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.children.push(Node::Element(self))
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<El<Ms>> {
    fn update(self, el: &mut El<Ms>) {
        el.children
            .append(&mut self.into_iter().map(Node::Element).collect());
    }
}

impl<Ms> UpdateEl<El<Ms>> for Node<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.children.push(self)
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<Node<Ms>> {
    fn update(mut self, el: &mut El<Ms>) {
        el.children.append(&mut self);
    }
}

/// This is intended only to be used for the custom! element macro.
impl<Ms> UpdateEl<El<Ms>> for Tag {
    fn update(self, el: &mut El<Ms>) {
        el.tag = self;
    }
}

impl<Ms, I, U, F> UpdateEl<El<Ms>> for std::iter::Map<I, F>
where
    I: Iterator,
    U: UpdateEl<El<Ms>>,
    F: FnMut(I::Item) -> U,
{
    fn update(self, el: &mut El<Ms>) {
        self.for_each(|item| item.update(el));
    }
}

// impl<Ms, I, U, P> UpdateEl<El<Ms>> for std::iter::Filter<I, P>
// where
//     I: Iterator,
//     U: UpdateEl<El<Ms>>,
//     P: FnMut(&I::Item) -> bool,
// {
//     fn update(self, el: &mut El<Ms>) {
//         self.for_each(|item| item.update(el));
//     }
// }

//impl<Ms, I, U, F> UpdateEl<El<Ms>> for std::iter::Map<I, F>
//where
//    I: Iterator,
//    U: UpdateEl<Attrs>,
//    F: FnMut(I::Item) -> U,
//{
//    fn update(self, el: &mut El<Ms>) {
//        self.for_each(|item| item.update(el));
//    }
//}
//
//impl<Ms, I, U, F> UpdateEl<El<Ms>> for std::iter::Map<I, F>
//where
//    I: Iterator,
//    U: UpdateEl<&Attrs>,
//    F: FnMut(I::Item) -> U,
//{
//    fn update(self, el: &mut El<Ms>) {
//        self.for_each(|item| item.update(el));
//    }
//}
//
//impl<Ms, I, U, F> UpdateEl<El<Ms>> for std::iter::Map<I, F>
//where
//    I: Iterator,
//    U: UpdateEl<Style>,
//    F: FnMut(I::Item) -> U,
//{
//    fn update(self, el: &mut El<Ms>) {
//        self.for_each(|item| item.update(el));
//    }
//}
//
//impl<Ms, I, U, F> UpdateEl<El<Ms>> for std::iter::Map<I, F>
//where
//    I: Iterator,
//    U: UpdateEl<&Style>,
//    F: FnMut(I::Item) -> U,
//{
//    fn update(self, el: &mut El<Ms>) {
//        self.for_each(|item| item.update(el));
//    }
//}
