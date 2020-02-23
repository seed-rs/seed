use super::{Attrs, El, ElRef, EventHandler, Node, Style, Tag, Text};

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

impl<Ms> UpdateEl<El<Ms>> for EventHandler<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.event_handler_manager.add_event_handlers(vec![self])
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

impl<Ms> UpdateEl<El<Ms>> for Node<Ms> {
    fn update(self, el: &mut El<Ms>) {
        el.children.push(self)
    }
}

/// This is intended only to be used for the custom! element macro.
impl<Ms> UpdateEl<El<Ms>> for Tag {
    fn update(self, el: &mut El<Ms>) {
        el.tag = self;
    }
}

impl<Ms, E: Clone> UpdateEl<El<Ms>> for ElRef<E> {
    fn update(self, el: &mut El<Ms>) {
        el.refs.push(self.shared_node_ws);
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

impl<Ms, I, U, F, II> UpdateEl<El<Ms>> for std::iter::FlatMap<I, II, F>
where
    I: Iterator,
    U: UpdateEl<El<Ms>>,
    II: IntoIterator<Item = U>,
    F: FnMut(I::Item) -> II,
{
    fn update(self, el: &mut El<Ms>) {
        self.for_each(|item| item.update(el));
    }
}

// ----- Containers ------

impl<Ms, T> UpdateEl<El<Ms>> for Option<T>
where
    T: UpdateEl<El<Ms>>,
{
    fn update(self, el: &mut El<Ms>) {
        if let Some(val) = self {
            val.update(el);
        }
    }
}

impl<Ms, T, E> UpdateEl<El<Ms>> for Result<T, E>
where
    T: UpdateEl<El<Ms>>,
    E: UpdateEl<El<Ms>>,
{
    fn update(self, el: &mut El<Ms>) {
        match self {
            Ok(val) => val.update(el),
            Err(err) => err.update(el),
        }
    }
}

impl<Ms, T> UpdateEl<El<Ms>> for Vec<T>
where
    T: UpdateEl<El<Ms>>,
{
    fn update(self, el: &mut El<Ms>) {
        for item in self {
            item.update(el);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn vec_with_update_el() {
        use crate::prelude::*;

        let children: Vec<Node<()>> = nodes![h1!["Page title"], div!["body"]];

        let _ = div![
            // Vec<Attrs>
            vec![attrs![ At::Title => "Title" ], attrs![ At::Width => 22.0 ],],
            // Vec<Style>
            vec![
                style![ St::Color => "#fff" ],
                style![ St::Display => "flex" ]
            ],
            children,
        ];
    }

    #[wasm_bindgen_test]
    fn option_with_update_el() {
        let val: Option<Node<()>> = if true { Some(div!["Some value"]) } else { None };

        let _ = div![val];
    }

    #[wasm_bindgen_test]
    fn result_with_update_el() {
        let val: Result<Node<()>, Node<()>> = if true {
            Ok(div!["Sent successfully"])
        } else {
            Err(h2!["Error, invalid input"])
        };

        let _ = div![val];
    }
}
