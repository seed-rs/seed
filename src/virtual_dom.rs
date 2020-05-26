pub mod attrs;
pub mod el_ref;
pub mod event_handler_manager;
pub mod mailbox;
pub mod node;
pub mod patch;
pub mod style;
pub mod to_classes;
pub mod update_el;
pub mod values;
pub mod view;

pub use attrs::Attrs;
pub use el_ref::{el_ref, ElRef, SharedNodeWs};
pub use event_handler_manager::{EventHandler, EventHandlerManager, Listener};
pub use mailbox::Mailbox;
pub use node::{el_key, El, ElKey, IntoNodes, Node, Text};
pub use style::Style;
pub use to_classes::ToClasses;
pub use update_el::{UpdateEl, UpdateElForIterator};
pub use values::{AsAtValue, AtValue, CSSValue};
pub use view::View;

pub use crate::dom_entity_names::{At, Ev, St, Tag};

#[cfg(test)]
pub mod tests {

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    use web_sys::{self, Element};

    use crate as seed;
    use crate::{
        browser::{dom::virtual_dom_bridge, util},
        class,
        prelude::*,
        virtual_dom::{mailbox::Mailbox, patch},
    };

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, Debug)]
    enum Msg {}

    struct TestModel;

    fn test_init(
        _: seed::browser::url::Url,
        _: &mut impl seed::app::orders::Orders<Msg>,
    ) -> TestModel {
        TestModel
    }

    fn create_app() -> App<Msg, TestModel, Node<Msg>> {
        App::start("output", test_init, |_, _, _| (), |_| seed::empty())
    }

    fn call_patch(
        doc: &web_sys::Document,
        parent: &Element,
        mailbox: &Mailbox<Msg>,
        old_vdom: Node<Msg>,
        mut new_vdom: Node<Msg>,
        app: &App<Msg, TestModel, Node<Msg>>,
    ) -> Node<Msg> {
        patch::patch(&doc, old_vdom, &mut new_vdom, parent, None, mailbox, &app);
        new_vdom
    }

    fn iter_nodelist(list: web_sys::NodeList) -> impl Iterator<Item = web_sys::Node> {
        (0..list.length()).map(move |i| list.item(i).unwrap())
    }

    fn iter_child_nodes(node: &web_sys::Node) -> impl Iterator<Item = web_sys::Node> {
        iter_nodelist(node.child_nodes())
    }

    #[wasm_bindgen_test]
    fn el_added() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").expect("parent");

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().expect("node_ws").clone();
            parent.append_child(&old_ws).expect("successful appending");

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            vdom = call_patch(&doc, &parent, &mailbox, vdom, div!["text"], &app);
            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(old_ws.child_nodes().length(), 1);
            assert_eq!(
                old_ws
                    .first_child()
                    .expect("first_child")
                    .text_content()
                    .expect("first_child's text_content"),
                "text"
            );

            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div!["text", "more text", vec![li!["even more text"]]],
                &app,
            );

            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(old_ws.child_nodes().length(), 3);
            assert_eq!(
                old_ws
                    .child_nodes()
                    .item(0)
                    .expect("0. item")
                    .text_content()
                    .expect("0. item's text_content"),
                "text"
            );
            assert_eq!(
                old_ws
                    .child_nodes()
                    .item(1)
                    .expect("1. item")
                    .text_content()
                    .expect("1. item's text_content"),
                "more text"
            );
            let child3 = old_ws.child_nodes().item(2).expect("child3");
            assert_eq!(child3.node_name(), "LI");
            assert_eq!(
                child3.text_content().expect("child3's text_content"),
                "even more text"
            );
        } else {
            panic!("Node not Element")
        }
    }

    #[wasm_bindgen_test]
    fn el_removed() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            // First add some child nodes using the vdom
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div!["text", "more text", vec![li!["even more text"]]],
                &app,
            );

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 3);
            let old_child1 = old_ws.child_nodes().item(0).unwrap();

            // Now test that patch function removes the last 2 nodes
            call_patch(&doc, &parent, &mailbox, vdom, div!["text"], &app);

            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(old_ws.child_nodes().length(), 1);
            assert!(old_child1.is_same_node(old_ws.child_nodes().item(0).as_ref()));
        }
    }

    #[wasm_bindgen_test]
    fn el_changed() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(el) = vdom.clone() {
            let old_ws = el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            // First add some child nodes using the vdom
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![span!["hello"], ", ", span!["world"]],
                &app,
            );

            assert_eq!(parent.child_nodes().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 3);

            // Now add some attributes
            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![
                    span![class!["first"], "hello"],
                    ", ",
                    span![class!["second"], "world"],
                ],
                &app,
            );

            let child1 = old_ws
                .child_nodes()
                .item(0)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(child1.get_attribute("class"), Some("first".to_string()));
            let child3 = old_ws
                .child_nodes()
                .item(2)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(child3.get_attribute("class"), Some("second".to_string()));
        } else {
            panic!("Node not Element")
        }
    }

    #[wasm_bindgen_test]
    fn els_changed_correct_order() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = div![];
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(el) = vdom.clone() {
            let old_ws = el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            vdom = call_patch(&doc, &parent, &mailbox, vdom, div!["1", a!["2"]], &app);
            let html_result = old_ws.clone().dyn_into::<Element>().unwrap().inner_html();
            assert_eq!(html_result, "1<a>2</a>");

            call_patch(&doc, &parent, &mailbox, vdom, div![a!["A"], "B"], &app);
            let html_result = old_ws.dyn_into::<Element>().unwrap().inner_html();
            assert_eq!(html_result, "<a>A</a>B");
        } else {
            panic!("Node not Element")
        }
    }

    /// Test if attribute `disabled` is correctly added and then removed.
    #[wasm_bindgen_test]
    fn attr_disabled() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            // First add button without attribute `disabled`
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![button![attrs! { At::Disabled => false.as_at_value() }]],
                &app,
            );

            assert_eq!(parent.child_nodes().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 1);
            let button = old_ws
                .child_nodes()
                .item(0)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(button.has_attribute("disabled"), false);

            // Now add attribute `disabled`
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![button![attrs! { At::Disabled => true.as_at_value() }]],
                &app,
            );

            let button = old_ws
                .child_nodes()
                .item(0)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(
                button
                    .get_attribute("disabled")
                    .expect("button hasn't got attribute `disabled`!"),
                ""
            );

            // And remove attribute `disabled`
            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![button![attrs! { At::Disabled => false.as_at_value() }]],
                &app,
            );

            let button = old_ws
                .child_nodes()
                .item(0)
                .unwrap()
                .dyn_into::<Element>()
                .unwrap();
            assert_eq!(button.has_attribute("disabled"), false);
        } else {
            panic!("Node not El")
        }
    }

    /// Test that if the first child was a seed::empty() and it is changed to a non-empty El,
    /// then the new element is inserted at the correct position.
    #[wasm_bindgen_test]
    fn empty_changed_in_front() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![seed::empty(), "b", "c"],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&old_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["b", "c"],
            );

            call_patch(&doc, &parent, &mailbox, vdom, div!["a", "b", "c"], &app);

            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&old_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["a", "b", "c"],
            );
        } else {
            panic!("Not Element node")
        }
    }

    /// Test that if a middle child was a seed::empty() and it is changed to a non-empty El,
    /// then the new element is inserted at the correct position.
    #[wasm_bindgen_test]
    fn empty_changed_in_the_middle() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        if let Node::Element(vdom_el) = vdom.clone() {
            // clone so we can keep using it after vdom is modified
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div!["a", seed::empty(), "c"],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&old_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["a", "c"],
            );

            call_patch(&doc, &parent, &mailbox, vdom, div!["a", "b", "c"], &app);

            assert_eq!(parent.children().length(), 1);
            assert!(old_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&old_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["a", "b", "c"],
            );
        } else {
            panic!("Not Element node")
        }
    }

    /// Test that if the old_el passed to patch was itself an empty, it is correctly patched to a non-empty.
    #[wasm_bindgen_test]
    fn root_empty_changed() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = seed::empty();

        vdom = call_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div!["a", seed::empty(), "c"],
            &app,
        );
        assert_eq!(parent.children().length(), 1);
        if let Node::Element(vdom_el) = vdom {
            let el_ws = vdom_el.node_ws.as_ref().expect("el_ws missing");
            assert!(el_ws.is_same_node(parent.first_child().as_ref()));
            assert_eq!(
                iter_child_nodes(&el_ws)
                    .map(|node| node.text_content().unwrap())
                    .collect::<Vec<_>>(),
                &["a", "c"],
            );
        } else {
            panic!("Node not Element type")
        }
    }

    /// Test that an empty->empty transition is handled correctly.
    #[wasm_bindgen_test]
    fn root_empty_to_empty() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let old = seed::empty();
        call_patch(&doc, &parent, &mailbox, old, seed::empty(), &app);
        assert_eq!(parent.children().length(), 0);
    }

    /// Test that a text Node is correctly patched to an Element and vice versa.
    #[wasm_bindgen_test]
    fn text_to_element_to_text() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = seed::empty();
        vdom = call_patch(&doc, &parent, &mailbox, vdom, Node::new_text("abc"), &app);
        assert_eq!(parent.child_nodes().length(), 1);
        let text = parent
            .first_child()
            .unwrap()
            .dyn_ref::<web_sys::Text>()
            .expect("not a Text node")
            .clone();
        assert_eq!(text.text_content().unwrap(), "abc");

        // change to a span (that contains a text node and styling).
        // span was specifically chosen here because text Els are saved with the span tag.
        // (or at least they were when the test was written.)
        vdom = call_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            span![style!["color" => "red"], "def"],
            &app,
        );
        assert_eq!(parent.child_nodes().length(), 1);
        let element = parent
            .first_child()
            .unwrap()
            .dyn_ref::<Element>()
            .expect("not an Element node")
            .clone();
        assert_eq!(&element.tag_name().to_lowercase(), "span");

        // change back to a text node
        call_patch(&doc, &parent, &mailbox, vdom, Node::new_text("abc"), &app);
        assert_eq!(parent.child_nodes().length(), 1);
        let text = parent
            .first_child()
            .unwrap()
            .dyn_ref::<web_sys::Text>()
            .expect("not a Text node")
            .clone();
        assert_eq!(text.text_content().unwrap(), "abc");
    }

    /// Test that the element with the key inserted and removed in the middle correctly.
    #[wasm_bindgen_test]
    fn el_key_insert_remove() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            // First add two children nodes using the vdom
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![vec![li!["first"], li!["last"]]],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 2);

            let first = old_ws.child_nodes().item(0).unwrap();
            let last = old_ws.child_nodes().item(1).unwrap();

            // Then insert the child node in the middle
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![vec![
                    li!["first"],
                    li![el_key(&"middle"), "middle"],
                    li!["last"]
                ]],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 3);

            assert!(first.is_same_node(old_ws.child_nodes().item(0).as_ref()));
            assert!(last.is_same_node(old_ws.child_nodes().item(2).as_ref()));

            // And then remove the child node in the middle
            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![vec![li!["first"], li!["last"]]],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 2);

            assert!(first.is_same_node(old_ws.child_nodes().item(0).as_ref()));
            assert!(last.is_same_node(old_ws.child_nodes().item(1).as_ref()));
        } else {
            panic!("Node not Element")
        }
    }

    /// Test that the element with the key is replaced correctly.
    #[wasm_bindgen_test]
    fn el_key_replace() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            // First add three children nodes using the vdom
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![vec![
                    li![el_key(&"first"), "first"],
                    li![el_key(&"variant-A"), "middle"],
                    li![el_key(&"last"), "last"]
                ]],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 3);

            let first = old_ws.child_nodes().item(0).unwrap();
            let middle = old_ws.child_nodes().item(1).unwrap();
            let last = old_ws.child_nodes().item(2).unwrap();

            // Then replace the middle elements.
            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![vec![
                    li![el_key(&"first"), "first"],
                    li![el_key(&"variant-B"), "middle"],
                    li![el_key(&"last"), "last"],
                ]],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 3);

            assert!(first.is_same_node(old_ws.child_nodes().item(0).as_ref()));
            assert!(!middle.is_same_node(old_ws.child_nodes().item(1).as_ref()));
            assert!(last.is_same_node(old_ws.child_nodes().item(2).as_ref()));
        } else {
            panic!("Node not Element")
        }
    }

    /// Test that the elements with the keys are swaped correctly.
    #[wasm_bindgen_test]
    fn el_key_swap() {
        let app = create_app();
        let mailbox = Mailbox::new(|_msg: Option<Msg>| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = Node::Element(El::empty(Tag::Div));
        virtual_dom_bridge::assign_ws_nodes(&doc, &mut vdom);
        // clone so we can keep using it after vdom is modified
        if let Node::Element(vdom_el) = vdom.clone() {
            let old_ws = vdom_el.node_ws.as_ref().unwrap().clone();
            parent.append_child(&old_ws).unwrap();

            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 0);

            // First add two children nodes using the vdom
            vdom = call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![vec![
                    li![el_key(&"first"), "first"],
                    li![el_key(&"last"), "last"]
                ]],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 2);

            let first = old_ws.child_nodes().item(0).unwrap();
            let last = old_ws.child_nodes().item(1).unwrap();

            // Then swap the first and last elements.
            call_patch(
                &doc,
                &parent,
                &mailbox,
                vdom,
                div![vec![
                    li![el_key(&"last"), "last"],
                    li![el_key(&"first"), "first"]
                ]],
                &app,
            );
            assert_eq!(parent.children().length(), 1);
            assert_eq!(old_ws.child_nodes().length(), 2);

            assert!(
                first.is_same_node(old_ws.child_nodes().item(1).as_ref())
                    || last.is_same_node(old_ws.child_nodes().item(0).as_ref())
            );
        } else {
            panic!("Node not Element")
        }
    }

    /// Tests an update() function that repeatedly sends messages or performs commands.
    #[wasm_bindgen_test(async)]
    async fn update_promises() {
        // ARRANGE

        // when we call `test_value_sender.send(..)`, future `test_value_receiver` will be marked as resolved
        let (test_value_sender, test_value_receiver) =
            futures::channel::oneshot::channel::<Counters>();

        // big numbers because we want to test if it doesn't blow call-stack
        // Note: Firefox has bigger call stack then Chrome - see http://2ality.com/2014/04/call-stack-size.html
        const MESSAGES_TO_SEND: i32 = 5_000;
        const COMMANDS_TO_PERFORM: i32 = 4_000;

        #[derive(Default, Copy, Clone, Debug)]
        struct Counters {
            messages_sent: i32,
            commands_scheduled: i32,
            messages_received: i32,
            commands_performed: i32,
        }

        #[derive(Default)]
        struct Model {
            counters: Counters,
            test_value_sender: Option<futures::channel::oneshot::Sender<Counters>>,
        }
        #[derive(Clone)]
        enum Msg {
            MessageReceived,
            CommandPerformed,
            Start,
        }

        fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
            orders.skip();

            match msg {
                Msg::MessageReceived => model.counters.messages_received += 1,
                Msg::CommandPerformed => model.counters.commands_performed += 1,
                Msg::Start => (),
            }

            if model.counters.messages_sent < MESSAGES_TO_SEND {
                orders.send_msg(Msg::MessageReceived);
                model.counters.messages_sent += 1;
            }
            if model.counters.commands_scheduled < MESSAGES_TO_SEND {
                orders.perform_cmd(async { Msg::CommandPerformed });
                model.counters.commands_scheduled += 1;
            }

            if model.counters.messages_received == MESSAGES_TO_SEND
                && model.counters.commands_performed == COMMANDS_TO_PERFORM
            {
                model
                    .test_value_sender
                    .take()
                    .unwrap()
                    .send(model.counters)
                    .unwrap()
            }
        }

        let app = App::start(
            "output",
            |_, _| Model {
                test_value_sender: Some(test_value_sender),
                ..Default::default()
            },
            update,
            |_| seed::empty(),
        );

        // ACT
        app.update(Msg::Start);

        // ASSERT
        test_value_receiver
            .await
            .map(|counters| {
                assert_eq!(counters.messages_received, MESSAGES_TO_SEND);
                assert_eq!(counters.commands_performed, COMMANDS_TO_PERFORM);
            })
            .expect("test_value_sender.send probably wasn't called!");
    }
}
