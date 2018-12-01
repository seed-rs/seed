use std::{cell::RefCell, rc::Rc, boxed::Box};
use std::borrow::Cow;

use wasm_bindgen::{prelude::*, JsCast};

//use js_sys;

use crate::dom_types::{El, Events, Event};
use crate::{mailbox, subscription, node, element}; // todo temp?

fn test<M>(model: M) {

}

fn add_event<T>(el_ws: &web_sys::Element, event: &Event, handler: T)
    where
        T: 'static + FnMut(web_sys::Event),
{

//    let closure: Closure<FnMut()> = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    // todo is el_et necessary? Seems like we can just add the event listener to el_ws directly.
    let el_et: web_sys::EventTarget = el_ws.clone().into();  // todo clone??

//            el_ws.add_event_listener_with_callback(event.as_str(), closure.as_ref().unchecked_ref()).unwrap();
    el_et.add_event_listener_with_callback(event.as_str(), closure.as_ref().unchecked_ref()).unwrap();

    // Comment below taken from bindgen, verbatim:
    // The instances of `Closure` that we created will invalidate their
    // corresponding JS callback whenever they're dropped, so if we were to
    // normally return from `run` then both of our registered closures will
    // raise exceptions when invoked.
    //
    // Normally we'd store these handles to later get dropped at an appropriate
    // time but for now we want these to be global handlers so we use the
    // `forget` method to drop them without invalidating the closure. Note that
    // this is leaking memory in Rust, so this should be done judiciously!
    closure.forget();
}


/// Experimental struct to hold state-related parts of app, and methods.
struct App<Ms, Mdl> {
    document: web_sys::Document,
    main_div: web_sys::Element,
    model: Rc<RefCell<Mdl>>,
//    update: Box<Fn(&Ms, Rc<Mdl>) -> Mdl>,
    update: fn(&Ms, Rc<RefCell<Mdl>>) -> Mdl,
    top_component: fn(&Mdl) -> El<Ms>,
//    top_component: Box<Fn(&Mdl) -> El<Ms>>,
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetative sequences of parameters.
impl<Ms, Mdl> App<Ms, Mdl> {
    // todo don't make the user wrap the model in rc<refcell; do it here.
//    pub fn new(model: Mdl, update: Box<Fn(&Ms, Rc<Mdl>) -> Mdl>,
//               top_component: Box<Fn(&Mdl) -> El<Ms>>, parent_div_id: &str) -> Self {

    pub fn new(model: Mdl, update: fn(&Ms, Rc<RefCell<Mdl>>) -> Mdl,
        top_component: fn(&Mdl) -> El<Ms>, parent_div_id: &str) -> Self {

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let main_div = document.get_element_by_id(parent_div_id).unwrap();

        Self {
            document,
            main_div,
            model: Rc::new(RefCell::new(model)),
            update,
            top_component,
        }
    }

    pub fn test(&mut self) {

    }

//    pub fn update_(&self) -> &Box<Fn(&Ms, Rc<Mdl>) -> Mdl> {
//        &self.update
//    }
//
//    pub fn top_component_(&self) -> &Box<Fn(&Mdl) -> El<Ms>> {
//        &self.top_component
//    }


    /// Convert from an element in our own virtual dom, to a web_sys element.  Note that
/// we're unable to implmement this directly as impl From in web_sys::Element, due to issues
/// converting between web_sys::Node and web_sys::Element.
    fn el_vdom_to_websys(&mut self, el_vdom: &El<Ms>) -> web_sys::Element {
        let el_ws = self.document.create_element(el_vdom.tag.as_str()).unwrap();
        for (name, val) in &el_vdom.attrs.vals {
            el_ws.set_attribute(name, val).unwrap();
        }

        // Style is just an attribute in the actual Dom, but is handled specially in our vdom;
        // merge the different parts of style here.
        if &el_vdom.style.vals.keys().len() > &0 {
            el_ws.set_attribute("style", &el_vdom.style.as_str()).unwrap();
        }

        // We store text as Option<String>, but set_text_content uses Option<&str>.
        // A naive match Some(t) => Some(&t) does not work.
        // See https://stackoverflow.com/questions/31233938/converting-from-optionstring-to-optionstr
        el_ws.set_text_content(el_vdom.text.as_ref().map(String::as_ref));

        // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html

        for (vdom_event, message) in &el_vdom.events.vals {
            let cb = move |ws_event: web_sys::Event| {
                &ws_event.prevent_default();
                web_sys::console::log_1(&ws_event.into());
//              self.model = self.model.clone();
//                test(&self.model);
//                self.model.borrow();
//                self.test();
            };

            add_event(&el_ws, vdom_event, cb);
        }

        el_ws
    }

    /// Create a web_sys dom tree from one using our own format, by recursively iterating
    /// through nodes.
    fn ws_dom_from_vdom(&mut self, active_el_vdom: &El<Ms>, active_el_ws: web_sys::Element) {
        let el_ws = &mut self.el_vdom_to_websys(active_el_vdom);
        active_el_ws.append_child(&el_ws).unwrap();

        for child in &active_el_vdom.children {
            self.ws_dom_from_vdom(&child, el_ws.clone());
        }

    }

    /// This runs whenever the state is changed, ie the user-written update function is called.
    /// It updates the state, and any DOM elements affected by this change.
    /// todo this is where we need to compare against differences and only update nodes affected
    /// by teh state change.
    fn update_dom(&mut self, parent_div: web_sys::Element) {
        let top_el = (self.top_component)(&self.model.borrow());

        self.ws_dom_from_vdom(&top_el, parent_div.clone());
    }

    /// Mount our top-level component to an existing div in the HTML file.

    pub fn mount(&mut self) -> Result<(), JsValue> {
        let body = self.document.body().expect("document should have a body");
        body.append_child(&self.main_div)?;

        self.update_dom(self.main_div.clone());

        Ok(())
    }

}

// The entry point for user apps; exposed in the prelude.
pub fn run<Ms, Mdl>(model: Mdl, update: fn(&Ms, Rc<RefCell<Mdl>>) -> Mdl,
        top_component: fn(&Mdl) -> El<Ms>, parent_div_id: &str) -> Result<(), JsValue> {
    let mut app = App::new(model, update, top_component, parent_div_id);
    app.mount()
}