use super::{Attrs, El, Node, Style, Tag, Text};
use crate::EventHandler;

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

impl<Ms> UpdateEl<El<Ms>> for EventHandler<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.event_handler_manager.add_event_handlers(vec![self])
    }
}

impl<Ms> UpdateEl<El<Ms>> for Vec<EventHandler<Ms>> {
    fn update(self, el: &mut El<Ms>) {
        el.event_handler_manager.add_event_handlers(self);
    }
}

impl<Ms> UpdateEl<El<Ms>> for &str {
    fn update(self, el: &mut El<Ms>) {
        el.children.push(Node::Text(Text::new(self.to_string())))
    }
}

// In the most cases `&str` is enough,
// but if we have, for instance, `Filter` iterator of `String`s -
// then the Rust type system can't coerce `String` to `&str`.
//
// However if we implement `UpdateEl` for `String`, code like `h1![model.title]` cannot be compiled,
// because Rust chooses `String` impl instead of `&str` and fails on moving value (`title`).
// @TODO How to resolve it? `&self`?
//
//impl<Ms> UpdateEl<El<Ms>> for String {
//    fn update(self, el: &mut El<Ms>) {
//        el.children.push(Node::Text(Text::new(self)))
//    }
//}

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

// ----- Iterators ------

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

impl<Ms, I, U, F> UpdateEl<El<Ms>> for std::iter::FilterMap<I, F>
where
    I: Iterator,
    U: UpdateEl<El<Ms>>,
    F: FnMut(I::Item) -> Option<U>,
{
    fn update(self, el: &mut El<Ms>) {
        self.for_each(|item| item.update(el));
    }
}

impl<Ms, I, U, P> UpdateEl<El<Ms>> for std::iter::Filter<I, P>
where
    U: UpdateEl<El<Ms>>,
    I: Iterator<Item = U>,
    P: FnMut(&I::Item) -> bool,
{
    fn update(self, el: &mut El<Ms>) {
        self.for_each(|item| item.update(el));
    }
}
