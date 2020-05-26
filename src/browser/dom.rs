//! This module contains structs and enums that represent dom types, and their parts.
//! These are the types used internally by our virtual dom.

pub mod cast;
pub mod css_units;
pub mod event_handler;
pub mod namespace;
pub mod virtual_dom_bridge;

pub use namespace::Namespace;

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use crate as seed;
    use crate::virtual_dom::{patch, At, CSSValue, El, Mailbox, Node, St, Style, UpdateEl};
    use indexmap::IndexMap;
    use std::collections::HashSet;
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::Element;

    #[derive(Clone, Debug)]
    enum Msg {}

    struct TestModel;

    fn test_init(
        _: seed::browser::url::Url,
        _: &mut impl seed::app::orders::Orders<Msg>,
    ) -> TestModel {
        TestModel
    }

    fn create_app() -> seed::App<Msg, TestModel, Node<Msg>> {
        seed::App::start("output", self::test_init, |_, _, _| (), |_| seed::empty())
    }

    fn el_to_websys(mut node: Node<Msg>) -> web_sys::Node {
        let document = crate::util::document();
        let parent = document.create_element("div").unwrap();
        let app = create_app();

        patch::patch(
            &document,
            seed::empty(),
            &mut node,
            &parent,
            None,
            &Mailbox::new(|_: Option<Msg>| {}),
            &app,
        );

        if let Node::Element(el) = node {
            el.node_ws.unwrap()
        } else {
            panic!("not an El node")
        }
    }

    /// Assumes Node is an Element
    fn get_node_html(node: &web_sys::Node) -> String {
        node.dyn_ref::<Element>().unwrap().outer_html()
    }

    /// Assumes Node is an Element
    fn get_node_attrs(node: &web_sys::Node) -> IndexMap<String, String> {
        let element = node.dyn_ref::<Element>().unwrap();
        element
            .get_attribute_names()
            .values()
            .into_iter()
            .map(|item_res| {
                item_res.map(|item| {
                    let name = item.as_string().unwrap();
                    let value = element.get_attribute(&name).unwrap();
                    (name, value)
                })
            })
            .collect::<Result<IndexMap<String, String>, JsValue>>()
            .unwrap()
    }

    #[wasm_bindgen_test]
    pub fn single_div() {
        let expected = "<div>test</div>";

        let node = el_to_websys(div!["test"]);

        assert_eq!(expected, get_node_html(&node));
    }

    #[wasm_bindgen_test]
    pub fn nested_divs() {
        let expected = "<section><div><div><h1>huge success</h1></div><p>\
                        I'm making a note here</p></div><span>This is a triumph</span></section>";

        let node = el_to_websys(section![
            div![div![h1!["huge success"]], p!["I'm making a note here"]],
            span!["This is a triumph"]
        ]);

        assert_eq!(expected, get_node_html(&node));
    }

    #[wasm_bindgen_test]
    pub fn attrs_work() {
        let expected = "<section src=\"https://seed-rs.org\" class=\"biochemistry\">ok</section>";
        let expected2 = "<section class=\"biochemistry\" src=\"https://seed-rs.org\">ok</section>";

        let node = el_to_websys(section![
            attrs! {"class" => "biochemistry"; "src" => "https://seed-rs.org"},
            "ok"
        ]);

        let actual_html = get_node_html(&node);
        assert!(expected == actual_html || expected2 == actual_html);
    }

    /// Tests that multiple attribute sections with unconflicting attributes are handled correctly
    #[wasm_bindgen_test]
    pub fn merge_different_attrs() {
        let node = el_to_websys(a![
            id! {"my_id"},
            style!["background-color" => "red"],
            class!["my_class1"],
            attrs! {
                At::Href => "#my_ref";
            },
            attrs! {
                At::Name => "whatever";
            },
        ]);

        let mut expected = IndexMap::new();
        expected.insert("id".to_string(), "my_id".to_string());
        expected.insert("style".to_string(), "background-color:red".to_string());
        expected.insert("class".to_string(), "my_class1".to_string());
        expected.insert("href".to_string(), "#my_ref".to_string());
        expected.insert("name".to_string(), "whatever".to_string());
        assert_eq!(expected, get_node_attrs(&node));
    }

    /// Tests that multiple class attributes are handled correctly
    #[wasm_bindgen_test]
    pub fn merge_classes() {
        let mut e = a![
            class!["", "cls_1", "cls_2"],
            class!["cls_3", "", ""],
            attrs![
                At::Class => "cls_4 cls_5";
            ],
            class![
                "cls_6"
                "cls_7" => false
                "cls_8" => 1 == 1
            ]
        ];
        e.add_class("cls_9");
        let node = el_to_websys(e);

        let mut expected = IndexMap::new();
        expected.insert(
            "class".to_string(),
            "cls_1 cls_2 cls_3 cls_4 cls_5 cls_6 cls_8 cls_9".to_string(),
        );
        assert_eq!(expected, get_node_attrs(&node));
    }

    /// Tests that multiple style sections are handled correctly
    #[wasm_bindgen_test]
    pub fn merge_styles() {
        let node = el_to_websys(a![
            style!["border-top" => "1px"; "border-bottom" => "red"],
            style!["background-color" => "blue"],
        ]);

        let attrs = get_node_attrs(&node);
        let actual_styles = attrs["style"]
            .split(";")
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        let mut expected = HashSet::new();
        expected.insert("border-top:1px".to_string());
        expected.insert("border-bottom:red".to_string());
        expected.insert("background-color:blue".to_string());
        assert_eq!(expected, actual_styles);
    }

    /// Tests that multiple id attributes are handled correctly (the last ID should override the
    /// previous values)
    #[wasm_bindgen_test]
    pub fn merge_id() {
        let node = el_to_websys(a![
            id!("my_id1"),
            attrs! {
                At::Id => "my_id2";
            }
        ]);

        let mut expected = IndexMap::new();
        expected.insert("id".to_string(), "my_id2".to_string());
        assert_eq!(expected, get_node_attrs(&node));
    }

    /// Tests that method `replace_text` removes all text nodes and then adds a new one
    #[wasm_bindgen_test]
    pub fn replace_text() {
        let expected = "<div><span>bbb</span>xxx</div>";

        let mut e = div!["aaa", span!["bbb"], plain!["ccc"], "ddd"];
        e.replace_text("xxx");
        let node = el_to_websys(e);

        assert_eq!(expected, get_node_html(&node));
    }

    /// Test that `style!` macro accept types that have `to_css_value()` function
    #[wasm_bindgen_test]
    pub fn to_css_value_in_style() {
        let display: &str = "flex";
        let direction: String = "column".to_string();
        let order: Option<u32> = None;
        let gap: Option<&str> = Some("8px");

        let style = style![
            St::Display => display,
            St::FlexDirection => direction,
            St::Order => order,
            St::Gap => gap,
        ];

        let mut result_style = Style::empty();
        result_style.add(St::Display, CSSValue::Some("flex".into()));
        result_style.add(St::FlexDirection, CSSValue::Some("column".into()));
        result_style.add(St::Order, CSSValue::Ignored);
        result_style.add(St::Gap, CSSValue::Some("8px".into()));

        assert_eq!(style, result_style)
    }
}
