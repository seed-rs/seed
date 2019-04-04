//! See readme for details.

//#![deny(missing_docs)]

pub use crate::{
    fetch::{spawn_local, Method, Request},
    routing::{push_path, push_route, Url},
    util::{document, error, log, window},
    vdom::App, // todo remove App once new update system in place?
    websys_bridge::{to_html_el, to_input, to_kbevent, to_mouse_event, to_select, to_textarea},
};
use wasm_bindgen::{closure::Closure, JsCast};

#[macro_use]
pub mod shortcuts;

pub mod dom_types;
pub mod fetch;
pub mod routing;
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

/// Create an element flagged in a way that it will not be rendered. Useful
/// in ternary operations.
pub fn empty<Ms>() -> dom_types::El<Ms> {
    // The tag doesn't matter here, but this seems semantically appropriate.
    let mut el = dom_types::El::empty(dom_types::Tag::Del);
    el.empty = true;
    el
}

/// A high-level wrapper for web_sys::window.set_interval_with_callback_and_timeout_and_arguments_0:
///
/// # References
/// * [WASM bindgen closures](https://rustwasm.github.io/wasm-bindgen/examples/closures.html)
/// * [web_sys Window](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html)
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/setInterval)
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

/// See [set_interval](fn.set_interval.html)
///
///
/// # References
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Wemb/API/WindowOrWorkerGlobalScope/setTimeout)
pub fn set_timeout(handler: Box<Fn()>, timeout: i32) {
    let callback = Closure::wrap(handler as Box<dyn Fn()>);
    let window = web_sys::window().expect("Can't find window");
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            timeout,
        )
        .expect("Problem setting timeout");
    callback.forget();
}

/// Introduce El and Tag into the global namespace for convenience (El will be repeated
/// often in the output type of components), and UpdateEl, which is required
/// for element-creation macros, input event constructors, and the History struct.
/// Expose the wasm_bindgen prelude, and lifecycle hooks.
pub mod prelude {
    pub use crate::{
        dom_types::{
            did_mount, did_update, input_ev, keyboard_ev, mouse_ev, pointer_ev, raw_ev, simple_ev,
            will_unmount, At, El, Ev, Optimize::Key, Tag, UpdateEl,
        },
        shortcuts::*, // appears not to work.
        vdom::{ShouldRender, ShouldRender::*, Update},
    };
    pub use std::collections::HashMap;

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
            dom_types::{mouse_ev, El, UpdateEl},
            vdom::Update,
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

        fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
            match msg {
                Msg::Increment => {
                    model.val += 1;
                    Render.into()
                }
            }
        }

        fn view(_model: &Model) -> El<Msg> {
            div!["Hello world"]
        }

        fn window_events(_model: &Model) -> Vec<seed::dom_types::Listener<Msg>> {
            vec![mouse_ev("mousemove", |_| Msg::Increment)]
        }

        fn routes(_url: &seed::Url) -> Msg {
            Msg::Increment
        }

        #[wasm_bindgen]
        pub fn render() {
            seed::App::build(Model::default(), update, view)
                .mount("body")
                .routes(routes)
                .window_events(window_events)
                .finish()
                .run();
        }
    }

}
