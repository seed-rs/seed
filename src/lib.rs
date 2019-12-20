//! See readme for details.

//#![deny(missing_docs)]
#![allow(
    clippy::use_self,
    clippy::single_match_else,
    clippy::must_use_candidate
)]
#![allow(deprecated)] // @TODO delete once `seed::update` and related things are removed

// @TODO move to prelude (?)
pub use crate::{
    app::{App, AppBuilder},
    browser::dom::cast::{
        to_html_el, to_input, to_kbevent, to_mouse_event, to_select, to_textarea,
    },
    browser::service::fetch::{Method, Request},
    browser::service::routing::push_route,
    browser::url::Url,
    browser::util::{
        self, body, canvas, canvas_context_2d, cookies, document, error, history, html_document,
        log, update, window,
    },
};
use wasm_bindgen::{closure::Closure, JsCast};

#[macro_use]
pub mod shortcuts;
pub mod app;
pub mod browser;
pub mod dom_entity_names;
pub mod virtual_dom;

/// Create an element flagged in a way that it will not be rendered. Useful
/// in ternary operations.
pub const fn empty<Ms>() -> virtual_dom::Node<Ms> {
    virtual_dom::Node::Empty
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
        app::{
            builder::init::Init, AfterMount, App, BeforeMount, MessageMapper, MountType, Orders,
            RenderTimestampDelta, UrlHandling,
        },
        browser::dom::css_units::*,
        browser::dom::event_handler::{
            input_ev, keyboard_ev, mouse_ev, pointer_ev, raw_ev, simple_ev, trigger_update_handler,
        },
        browser::dom::lifecycle_hooks::{did_mount, did_update, will_unmount},
        browser::util::{
            request_animation_frame, ClosureNew, RequestAnimationFrameHandle,
            RequestAnimationFrameTime,
        },
        browser::Url,
        // macros are exported in crate root
        // https://github.com/rust-lang-nursery/reference/blob/master/src/macros-by-example.md
        shortcuts::*,
        virtual_dom::{
            AsAtValue, At, AtValue, CSSValue, El, Ev, Listener, Node, St, Tag, UpdateEl, View,
        },
    };
    pub use indexmap::IndexMap; // for attrs and style to work.
    pub use wasm_bindgen::prelude::*;
    pub use web_sys::Event;
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
        use crate::app::{builder::init::Init, Orders};
        use crate::browser::dom::event_handler::mouse_ev;
        use crate::prelude::*;
        use crate::virtual_dom::{Listener, Node};

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

        fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
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
}
