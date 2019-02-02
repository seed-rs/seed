//! See readme for details.

#![allow(unused_macros)]
//#![feature(custom_attribute)]  // For testing

pub use crate::{
    dom_types::Listener,
    fetch::{Method, Request, spawn_local},
    routing::{push_route, Url},
    util::{document, window},
    vdom::App,
    websys_bridge::{to_html_el, to_input, to_kbevent, to_mouse_event, to_select, to_textarea},
};
use wasm_bindgen::{closure::Closure, JsCast};

pub mod dom_types;
pub mod fetch;
pub mod routing;
pub mod shortcuts;
pub mod storage;
mod util;
mod vdom;
mod websys_bridge;

// todo: Why does this work without importing web_sys??

//// todos:
// todo Give 'components' their own message type/update fn. Could help efficient rendering,
// todo and code organization.
// todo dynamic routing
// todo local storage
// todo High-level css-grid and flex api?
// todo keyed elements?

/// Create an element flagged in a way that it will not be rendered. Useful
/// in ternary operations.
pub fn empty<Ms>() -> dom_types::El<Ms> {
    // The tag doesn't matter here, but this seems semantically appropriate.
    let mut el = dom_types::El::empty(dom_types::Tag::Del);
    el.add_attr("dummy-element".into(), "true".into());
    el
}

/// A convenience function for logging to the web browser's console.  See also
/// the log! macro, which is more flexible.
pub fn log<S: ToString>(text: S) {
    web_sys::console::log_1(&text.to_string().into());
}

/// A high-level wrapper for web_sys::window.set_interval_with_callback_and_timeout_and_arguments_0:
/// https://rustwasm.github.io/wasm-bindgen/examples/closures.html
/// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html
pub fn set_interval(handler: Box<Fn()>, timeout: i32) {
    let callback = Closure::wrap(handler as Box<dyn Fn()>);
    let window = web_sys::window().expect("Can't find window");
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        )
        .expect("Problem setting interval");
    callback.forget();
}

// todo: Perhaps put did_mount etc here so we call with seed:: instead of in prelude.
// todo or maybe not, for consistency with events.

/// Introduce El and Tag into the global namespace for convenience (El will be repeated
/// often in the output type of components), and UpdateEl, which is required
/// for element-creation macros, input event constructors, and the History struct.
/// Expose the wasm_bindgen prelude, and lifecycle hooks.
pub mod prelude {
    pub use std::collections::HashMap;
    pub use crate::{
        dom_types::{
            did_mount, did_update, El, input_ev, keyboard_ev, mouse_ev, raw_ev,
            simple_ev, Tag, UpdateEl, will_unmount, At, Ev
        },
        shortcuts::*, // appears not to work.
        vdom::{Update, Update::Render, Update::Skip},
    };

    pub use wasm_bindgen::prelude::*;

    //    pub use proc_macros::seed_update;

    //    pub use wasm_bindgen_macro::wasm_bindgen;

}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    use wasm_bindgen_test::*;

    /// This is a minimal app, that should build. Will fail if there's a breaking
    /// change.
    #[wasm_bindgen_test]
    pub fn app_builds() {
        use crate as seed; // required for macros to work.
        use crate::{
            div,
            routes,
            dom_types::{El, At, UpdateEl, mouse_ev},
            vdom::Update,
        };
        use crate::prelude::*;

        #[derive(Clone)]
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

        fn update(msg: Msg, model: Model) -> Update<Model> {
            match msg {
                Msg::Increment => Update::Render(Model { val: model.val + 1 }),
            }
        }

        fn view(_state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
            div!["Hello world"]
        }

        fn window_events(model: Model) -> Vec<seed::Listener<Msg>> {
            vec![
                mouse_ev("mousemove", |_| Msg::Increment)
            ]
        }

        #[wasm_bindgen]
        pub fn render() {
            let routes = routes! {
                "page1" => Msg::Increment,
                "page2" => Msg::Increment,
            };

            seed::App::build(Model::default(), update, view)
                .mount("body")
                .routes(routes)
                .window_events(window_events)
                .finish()
                .run();
        }
    }

}
