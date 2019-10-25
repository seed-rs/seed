//! See readme for details.

//#![deny(missing_docs)]
#![allow(clippy::use_self, clippy::single_match_else)]
#![allow(deprecated)] // @TODO delete once `seed::update` and related things are removed

pub use crate::{
    fetch::{Method, Request},
    routing::{push_route, Url},
    util::{body, document, error, history, log, update, window},
    vdom::{App, AppBuilder},
    websys_bridge::{to_html_el, to_input, to_kbevent, to_mouse_event, to_select, to_textarea},
};
use wasm_bindgen::{closure::Closure, JsCast};

#[macro_use]
pub mod shortcuts;

pub mod dom_entity_names;

pub mod css_units;
pub mod dom_types;
pub mod events;
pub mod fetch;
mod next_tick;
pub mod orders;
mod patch;
pub mod routing;
pub mod storage;
mod util;
mod vdom;
mod websys_bridge;

/// Create an element flagged in a way that it will not be rendered. Useful
/// in ternary operations.
pub fn empty<Ms: Clone>() -> dom_types::Node<Ms> {
    dom_types::Node::Empty
}

// @TODO remove `set_interval` and `set_timeout`? Alternative from `gloo` should be used instead.

/// A high-level wrapper for `web_sys::window.set_interval_with_callback_and_timeout_and_arguments_0`:
///
/// # References
/// * [WASM bindgen closures](https://rustwasm.github.io/wasm-bindgen/examples/closures.html)
/// * [`web_sys` Window](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html)
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/setInterval)
pub fn set_interval(handler: Box<dyn Fn()>, timeout: i32) {
    let callback = Closure::wrap(handler as Box<dyn Fn()>);
    util::window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        )
        .expect("Problem setting interval");
    callback.forget();
}

/// See [`set_interval`](fn.set_interval.html)
///
///
/// # References
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Wemb/API/WindowOrWorkerGlobalScope/setTimeout)
pub fn set_timeout(handler: Box<dyn Fn()>, timeout: i32) {
    let callback = Closure::wrap(handler as Box<dyn Fn()>);
    util::window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        )
        .expect("Problem setting timeout");
    callback.forget();
}

/// Introduce `El` and `Tag` into the global namespace for convenience (`El` will be repeated
/// often in the output type of components), and `UpdateEl`, which is required
/// for element-creation macros, input event constructors, and the `History` struct.
/// Expose the `wasm_bindgen` prelude, and lifecycle hooks.
pub mod prelude {
    pub use crate::{
        css_units::*,
        dom_entity_names::styles::St,
        dom_types::{
            did_mount, did_update, will_unmount, AsAtValue, At, AtValue, CSSValue, El,
            MessageMapper, Node, Tag, UpdateEl, View,
        },
        events::{
            input_ev, keyboard_ev, mouse_ev, pointer_ev, raw_ev, simple_ev, trigger_update_handler,
            Ev,
        },
        orders::Orders,
        routing::Url,
        // macros are exported in crate root
        // https://github.com/rust-lang-nursery/reference/blob/master/src/macros-by-example.md
        shortcuts::*,
        util::{
            request_animation_frame, ClosureNew, RequestAnimationFrameHandle,
            RequestAnimationFrameTime,
        },
        vdom::{Init, UrlHandling},
    };
    pub use indexmap::IndexMap; // for attrs and style to work.
    pub use wasm_bindgen::prelude::*;
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    use wasm_bindgen_test::*;

    /// This is a minimal app, that should build. Will fail if there's a breaking
    /// change.
    #[wasm_bindgen_test]
    #[allow(dead_code)]
    pub fn app_builds() {
        use crate as seed; // required for macros to work.
        use crate::prelude::*;
        use crate::{
            dom_types::{El, UpdateEl},
            events::mouse_ev,
            orders::Orders,
        };

        struct Model {
            pub val: i32,
        }

        impl Default for Model {
            fn default() -> Self {
                Self { val: 0 }
            }
        }

        #[derive(Clone)]
        enum Msg {
            Increment,
        }

        fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
            match msg {
                Msg::Increment => model.val += 1,
            }
        }

        fn view(_model: &Model) -> Vec<Node<Msg>> {
            vec![div!["Hello world"]]
        }

        fn window_events(_model: &Model) -> Vec<seed::events::Listener<Msg>> {
            vec![mouse_ev("mousemove", |_| Msg::Increment)]
        }

        fn routes(_url: seed::Url) -> Option<Msg> {
            Some(Msg::Increment)
        }

        #[wasm_bindgen]
        pub fn render() {
            seed::App::build(|_, _| Init::new(Model::default()), update, view)
                .mount("body")
                .routes(routes)
                .window_events(window_events)
                .finish()
                .run();
        }
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn attr_disabled_initial() {
        use crate as seed; // required for macros to work.
        use crate::prelude::*;
        use crate::{
            util,
            dom_types::{El, UpdateEl},
            orders::Orders,
        };

        #[derive(Default)]
        struct Model { }

        #[derive(Clone)]
        enum Msg { }

        fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) { }

        fn view(_: &Model) -> Node<Msg> {
            div![
              button![attrs!{At::Disabled => false;}]
            ]
        }

        let doc = util::document();
        let app_el = doc.create_element("div").unwrap();
        app_el.set_attribute("id", "app").unwrap();
        util::body().append_child(&app_el).unwrap();

        seed::App::build(|_, _| Init::new(Model::default()), update, view).finish().run();
        let button = doc.get_elements_by_tag_name("button").item(0).unwrap();
        assert_eq!(button.get_attribute("disabled"),None);
    }
}
