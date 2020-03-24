use super::{Attrs, El, ElRef, EventHandler, Node, Style, Tag, Text};

// ------ Traits ------

/// `UpdateEl` is used to distinguish arguments in element-creation macros, and handle
/// each type appropriately.
pub trait UpdateEl<Ms> {
    fn update_el(self, el: &mut El<Ms>);
}

/// Similar to `UpdateEl`, specialized for `Iterator`.
#[allow(clippy::module_name_repetitions)]
pub trait UpdateElForIterator<Ms> {
    fn update_el(self, el: &mut El<Ms>);
}

// ------ Implementations ------

impl<Ms, T: UpdateEl<Ms> + Clone> UpdateEl<Ms> for &T {
    fn update_el(self, el: &mut El<Ms>) {
        self.clone().update_el(el);
    }
}

// --- V-DOM entities ---

impl<Ms> UpdateEl<Ms> for Attrs {
    fn update_el(self, el: &mut El<Ms>) {
        el.attrs.merge(self);
    }
}

impl<Ms> UpdateEl<Ms> for Style {
    fn update_el(self, el: &mut El<Ms>) {
        el.style.merge(self);
    }
}

impl<Ms> UpdateEl<Ms> for EventHandler<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        el.event_handler_manager.add_event_handlers(vec![self])
    }
}

impl<Ms> UpdateEl<Ms> for El<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        el.children.push(Node::Element(self))
    }
}

impl<Ms> UpdateEl<Ms> for Node<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        el.children.push(self)
    }
}

/// This is intended only to be used for the `custom!` element macro.
impl<Ms> UpdateEl<Ms> for Tag {
    fn update_el(self, el: &mut El<Ms>) {
        el.tag = self;
    }
}

impl<Ms, E: Clone> UpdateEl<Ms> for ElRef<E> {
    fn update_el(self, el: &mut El<Ms>) {
        el.refs.push(self.shared_node_ws);
    }
}

// --- Texts ---

impl<Ms> UpdateEl<Ms> for String {
    fn update_el(self, el: &mut El<Ms>) {
        el.children.push(Node::Text(Text::new(self)))
    }
}

impl<Ms> UpdateEl<Ms> for &str {
    fn update_el(self, el: &mut El<Ms>) {
        el.children.push(Node::Text(Text::new(self.to_string())))
    }
}

// --- Numbers ---

impl<Ms> UpdateEl<Ms> for u32 {
    fn update_el(self, el: &mut El<Ms>) {
        self.to_string().update_el(el);
    }
}

impl<Ms> UpdateEl<Ms> for u64 {
    fn update_el(self, el: &mut El<Ms>) {
        self.to_string().update_el(el);
    }
}

impl<Ms> UpdateEl<Ms> for i32 {
    fn update_el(self, el: &mut El<Ms>) {
        self.to_string().update_el(el);
    }
}

impl<Ms> UpdateEl<Ms> for i64 {
    fn update_el(self, el: &mut El<Ms>) {
        self.to_string().update_el(el);
    }
}

impl<Ms> UpdateEl<Ms> for usize {
    fn update_el(self, el: &mut El<Ms>) {
        self.to_string().update_el(el);
    }
}

impl<Ms> UpdateEl<Ms> for f64 {
    fn update_el(self, el: &mut El<Ms>) {
        self.to_string().update_el(el);
    }
}

// --- Containers ---

impl<Ms, T: UpdateEl<Ms>, I: Iterator<Item = T>> UpdateElForIterator<Ms> for I {
    fn update_el(self, el: &mut El<Ms>) {
        for item in self {
            item.update_el(el);
        }
    }
}

impl<Ms, T: UpdateEl<Ms>> UpdateEl<Ms> for Option<T> {
    fn update_el(self, el: &mut El<Ms>) {
        self.into_iter().update_el(el)
    }
}

impl<Ms, T: UpdateEl<Ms>> UpdateEl<Ms> for Vec<T> {
    fn update_el(self, el: &mut El<Ms>) {
        self.into_iter().update_el(el)
    }
}

impl<Ms, T: UpdateEl<Ms> + Clone> UpdateEl<Ms> for &[T] {
    fn update_el(self, el: &mut El<Ms>) {
        self.iter().update_el(el)
    }
}

// ------ ------ Tests ------ ------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use wasm_bindgen_test::*;

    type Ms = ();

    // @TODO:
    // These tests only check types.
    // Verify also HTML once https://github.com/seed-rs/seed/issues/294 is resolved.
    // DRY

    // --- V-DOM entities ---

    #[wasm_bindgen_test]
    fn update_el_attrs() {
        let attrs: Attrs = attrs!(At::Href => "https://example.com");
        let _el: Node<Ms> = div![attrs];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_attrs() {
        let attrs: &Attrs = &attrs!(At::Href => "https://example.com");
        let _el: Node<Ms> = div![attrs];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_style() {
        let style: Style = style! {St::Left => px(5)};
        let _el: Node<Ms> = div![style];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_style() {
        let style: &Style = &style! {St::Left => px(5)};
        let _el: Node<Ms> = div![style];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_event_handler() {
        let event_handler: EventHandler<Ms> = ev(Ev::Click, |_| ());
        let _el: Node<Ms> = div![event_handler];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_event_handler() {
        let event_handler: &EventHandler<Ms> = &ev(Ev::Click, |_| ());
        let _el: Node<Ms> = div![event_handler];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_el() {
        let el: El<Ms> = El::empty(Tag::H2);
        let _el: Node<Ms> = div![el];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_el() {
        let el: &El<Ms> = &El::empty(Tag::H2);
        let _el: Node<Ms> = div![el];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_node() {
        let node: Node<Ms> = span![];
        let _el: Node<Ms> = div![node];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_node() {
        let node: &Node<Ms> = &span![];
        let _el: Node<Ms> = div![node];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_tag() {
        let tag: Tag = Tag::H1;
        let _el: Node<Ms> = div![tag];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_tag() {
        let tag: &Tag = &Tag::H1;
        let _el: Node<Ms> = div![tag];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_el_ref() {
        let el_ref: ElRef<web_sys::HtmlElement> = ElRef::default();
        let _el: Node<Ms> = div![el_ref];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_el_ref() {
        let el_ref: &ElRef<web_sys::HtmlElement> = &ElRef::default();
        let _el: Node<Ms> = div![el_ref];
        assert!(true);
    }

    // --- Texts ---

    #[wasm_bindgen_test]
    fn update_el_ref_str() {
        let text: &str = "foo";
        let _el: Node<Ms> = div![text];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_string() {
        let text: String = String::from("bar");
        let _el: Node<Ms> = div![text];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_string() {
        let text: &String = &String::from("ref_bar");
        let _el: Node<Ms> = div![text];
        assert!(true);
    }

    // --- Numbers ---

    #[wasm_bindgen_test]
    fn update_el_u32() {
        let number: u32 = 100;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_u32() {
        let number: &u32 = &1009;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_i32() {
        let number: i32 = -25;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_i32() {
        let number: &i32 = &-259;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_u64() {
        let number: u64 = 100;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_u64() {
        let number: &u64 = &1009;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_i64() {
        let number: i64 = -25;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_i64() {
        let number: &i64 = &-259;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_usize() {
        let number: usize = 1_012;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_usize() {
        let number: &usize = &10_129;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_f64() {
        let number: f64 = 3.14;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_f64() {
        let number: &f64 = &3.149;
        let _el: Node<Ms> = div![number];
        assert!(true);
    }

    // --- Containers ---

    #[wasm_bindgen_test]
    fn update_el_iterator_map() {
        let map_iterator = vec![3, 4].into_iter().map(|n| n * 2);
        let _el: Node<Ms> = div![map_iterator];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_iterator_filter() {
        let filter_iterator = vec![3, 4].into_iter().filter(|n| n % 2 == 1);
        let _el: Node<Ms> = div![filter_iterator];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_vec() {
        let vec: Vec<&str> = vec!["foo_1", "foo_2"];
        let _el: Node<Ms> = div![vec];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_vec() {
        let vec: &Vec<&str> = &vec!["foo_1", "foo_2"];
        let _el: Node<Ms> = div![vec];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_slice() {
        let slice: &[&str] = &["foo_1", "foo_2"];
        let _el: Node<Ms> = div![slice];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_option_some() {
        let option: Option<&str> = Some("foo_opt");
        let _el: Node<Ms> = div![option];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_ref_option_some() {
        let option: &Option<&str> = &Some("foo_opt");
        let _el: Node<Ms> = div![option];
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn update_el_option_none() {
        let option: Option<&str> = None;
        let _el: Node<Ms> = div![option];
        assert!(true);
    }
}
