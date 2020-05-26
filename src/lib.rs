//! Visit the [website](https://seed-rs.org/)
//!
//! See the [github Readme](https://github.com/seed-rs/seed) for details
//!
//!
//! ## Counter Example
//! ```
//! use seed::{prelude::*, *};
//!
//! // `init` describes what should happen when your app started.
//! fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
//!     Model::default()
//! }
//!
//! // `Model` describes our app state.
//! type Model = i32;
//!
//! // `Msg` describes the different events you can modify state with.
//! enum Msg {
//!     Increment,
//! }
//!
//! // `update` describes how to handle each `Msg`.
//! fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
//!     match msg {
//!         Msg::Increment => *model += 1,
//!     }
//! }
//!
//! // `view` describes what to display.
//! fn view(model: &Model) -> Node<Msg> {
//!     div![
//!         "This is a counter: ",
//!         C!["counter"],
//!         button![
//!             model,
//!             ev(Ev::Click, |_| Msg::Increment),
//!         ],
//!     ]
//! }
//!
//! #[wasm_bindgen(start)]
//! pub fn start() {
//!     // Mount the `app` to the element with the `id` "app".
//!     App::start("app", init, update, view);
//! }
//! ```

//#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![allow(
    clippy::use_self,
    clippy::single_match_else,
    clippy::must_use_candidate
)]
#![allow(deprecated)] // @TODO delete once `seed::update` and related things are removed

// @TODO Refactor once `optin_builtin_traits` or `negative_impls`
// @TODO is stable (https://github.com/seed-rs/seed/issues/391).
// --
// @TODO Remove `'static` bound from all `MsU`s once `optin_builtin_traits`, `negative_impls`
// @TODO or https://github.com/rust-lang/rust/issues/41875 is stable.
macro_rules! map_callback_return_to_option_ms {
    ($cb_type:ty, $callback:expr, $panic_text:literal, $output_type:tt) => {{
        let t_type = std::any::TypeId::of::<MsU>();
        if t_type == std::any::TypeId::of::<Ms>() {
            $output_type::new(move |value| {
                (&mut Some($callback(value)) as &mut dyn std::any::Any)
                    .downcast_mut::<Option<Ms>>()
                    .and_then(Option::take)
            })
        } else if t_type == std::any::TypeId::of::<Option<Ms>>() {
            $output_type::new(move |value| {
                (&mut $callback(value) as &mut dyn std::any::Any)
                    .downcast_mut::<Option<Ms>>()
                    .and_then(Option::take)
            })
        } else if t_type == std::any::TypeId::of::<()>() {
            $output_type::new(move |value| {
                $callback(value);
                None
            }) as $output_type<$cb_type>
        } else {
            panic!($panic_text);
        }
    }};
}
// @TODO move to prelude (?)
pub use crate::{
    app::App,
    browser::dom::cast::{
        to_drag_event, to_html_el, to_input, to_keyboard_event, to_mouse_event, to_select,
        to_textarea, to_touch_event,
    },
    browser::fetch,
    browser::url::Url,
    browser::util::{
        self, body, canvas, canvas_context_2d, cookies, document, error, history, html_document,
        log, window,
    },
    virtual_dom::{Attrs, EventHandler, Style},
};
pub use futures::future::{FutureExt, TryFutureExt};
use wasm_bindgen::{closure::Closure, JsCast};

#[macro_use]
pub mod shortcuts;
pub mod app;
pub mod browser;
pub mod dom_entity_names;
pub mod helpers;
pub mod virtual_dom;

/// Create an element flagged in a way that it will not be rendered. Useful
/// in ternary operations.
pub const fn empty<Ms>() -> virtual_dom::Node<Ms> {
    virtual_dom::Node::Empty
}

#[deprecated(
    since = "0.7.0",
    note = "use [`Orders::stream`](app/orders/trait.Orders.html#method.stream) instead"
)]
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

#[deprecated(
    since = "0.7.0",
    note = "use [`Orders::stream`](app/orders/trait.Orders.html#method.stream) instead"
)]
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
/// Expose the `wasm_bindgen` prelude.
pub mod prelude {
    pub use crate::{
        app::{
            cmds, streams, subs, App, CmdHandle, GetElement, MessageMapper, Orders, RenderInfo,
            StreamHandle, SubHandle,
        },
        browser::dom::css_units::*,
        browser::dom::event_handler::{
            drag_ev, ev, input_ev, keyboard_ev, mouse_ev, pointer_ev, raw_ev, simple_ev, touch_ev,
        },
        browser::fetch::{self, fetch, FetchError, Header, Method, Request, Response, Status},
        browser::util::{
            request_animation_frame, ClosureNew, RequestAnimationFrameHandle,
            RequestAnimationFrameTime,
        },
        browser::web_socket::{self, CloseEvent, WebSocket, WebSocketError, WebSocketMessage},
        browser::web_storage::{self, LocalStorage, SessionStorage, WebStorage},
        browser::{Url, UrlSearch},
        helpers::not,
        // macros are exported in crate root
        // https://github.com/rust-lang-nursery/reference/blob/master/src/macros-by-example.md
        shortcuts::*,
        virtual_dom::{
            el_key, el_ref::el_ref, AsAtValue, At, AtValue, CSSValue, El, ElRef, Ev, EventHandler,
            IntoNodes, Node, St, Tag, ToClasses, UpdateEl, UpdateElForIterator, View,
        },
    };
    pub use indexmap::IndexMap; // for attrs and style to work.
    pub use js_sys;
    pub use wasm_bindgen::{self, prelude::*, JsCast};
    pub use web_sys;
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
    pub(crate) fn app_builds() {
        use crate as seed; // required for macros to work.
        use crate::app::Orders;
        use crate::browser::dom::event_handler::mouse_ev;
        use crate::prelude::*;
        use crate::virtual_dom::{EventHandler, Node};

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

        fn window_events(_model: &Model) -> Vec<EventHandler<Msg>> {
            vec![mouse_ev("mousemove", |_| Msg::Increment)]
        }

        fn routes(_url: seed::Url) -> Option<Msg> {
            Some(Msg::Increment)
        }

        #[wasm_bindgen]
        pub fn render() {
            seed::App::start("render test app", |_, _| Model::default(), update, view);
        }
    }
}
