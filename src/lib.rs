//! See readme for details.

#![allow(unused_macros)]

pub mod dom_types;

mod dom_shortcuts;
//mod fetch;
mod vdom;


//// todos:
// todo router
// todo local storage
// todo vdom patching
// todo streamlined input handling for text etc
// todo maybe?? High-level css-grid and flex api?
// todo Async conflicts with events stepping on each other ?
// todo keyed elements??


/// The entry point for the app
pub fn run<Ms: Clone + Sized + 'static, Mdl: Sized + 'static>(model: Mdl, update: fn(&Ms, &Mdl) -> Mdl,
          view: fn(&Mdl) -> dom_types::El<Ms>, main_div_id: &str) {
    let app = vdom::App::new(model, update, view, main_div_id);

    // Our initial render. Can't initialize in new due to mailbox() requiring self.
    let mut main_el_vdom = (app.data.view)(&app.data.model.borrow());
    app.setup_vdom(&mut main_el_vdom, 0, 0);
    // Attach all children: This is where our initial render occurs.
    vdom::attach(&mut main_el_vdom, &app.data.main_div);

    // todo really: You shouldn't need to attach here. Will be handled by patch
// todo try it again. and maybe have a helper func to update the model, setup_vdom, run patch etc??
// todo or maybe i'm wrong
    app.data.main_el_vdom.replace(main_el_vdom);
}

/// Create an element flagged in a way that it will not be rendered. Useful
/// in ternary operations.
pub fn empty<Ms: Clone>() -> dom_types::El<Ms> {
    // The tag doesn't matter here, but this seems semantically appropriate.
    let mut el = dom_types::El::empty(dom_types::Tag::Del);
    el.add_attr("dummy-element".into(), "true".into());
    el
}

/// A convenience function for logging to the web browser's console.
pub fn log(text: &str) {
    web_sys::console::log_1(&text.into());
}

/// Introduce El into the global namespace for convenience (It will be repeated
/// often in the output type of components), and UpdateEl, which is required
/// for our element-creation macros.
pub mod prelude {
    pub use crate::dom_types::{El, UpdateEl, UpdateListener,
                               simple_ev, input_ev, keyboard_ev, raw_ev};
}
