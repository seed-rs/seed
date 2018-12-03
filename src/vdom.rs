use std::{cell::RefCell, rc::Rc, boxed::Box};
use std::borrow::Cow;

use wasm_bindgen::{prelude::*, JsCast};

//use js_sys;

use crate::dom_types::{El, Events, Event, Tag, Listener};
use crate::{mailbox, subscription, node, element}; // todo temp?

use self::mailbox::Mailbox; // todo temp

//pub trait App: Sized + 'static {
//    type Message;
//    type Mdl;
//
//    fn update(&mut self, _mailbox: &Mailbox<Self::Message>, _message: Self::Message) {}
////    fn render(&self) -> Node<Self::Message>;
//    fn render(&self) -> El<Self::Message>;
//}

fn add_event<T>(el_ws: &web_sys::Element, event: &Event, handler: T)
    where
        T: 'static + FnMut(web_sys::Event),
//        T: Sized + 'static
{

//    let closure: Closure<FnMut()> = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(web_sys::Event)>);

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
struct Inner<Ms: Sized + 'static , Mdl: Sized + 'static> {
    // todo do we store doc here, or call it from web_sys every time we create an el?
    document: web_sys::Document,
    main_div: web_sys::Element,
//    model: Rc<RefCell<Mdl>>,
    model: RefCell<Mdl>,
//    update: Box<Fn(&Ms, Rc<Mdl>) -> Mdl>,
//    update: fn(&Ms, Rc<RefCell<Mdl>>) -> Mdl,
//    update: fn(&Ms, RefCell<Mdl>) -> Mdl,
    update: fn(&Mailbox<Ms>, &Ms, &Mdl) -> Mdl,
    top_component: fn(&Mdl) -> El<Ms>,
//    top_component: Box<Fn(&Mdl) -> El<Ms>>,


    // todo temp?
//   vnode: RefCell<Node<A::Message>>,

    el_ws: RefCell<web_sys::Element>,
    el_vdom: RefCell<El<Ms>>,

    queue: RefCell<Vec<Ms>>,
    is_updating: RefCell<bool>,
}

pub struct Instance<Ms: Sized + 'static , Mdl: Sized + 'static> {
    inner: Rc<Inner<Ms, Mdl>>
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetative sequences of parameters.
impl<Ms: Sized + 'static, Mdl: Sized + 'static> Instance<Ms, Mdl> {
    // todo don't make the user wrap the model in rc<refcell; do it here.
//    pub fn new(model: Mdl, update: Box<Fn(&Ms, Rc<Mdl>) -> Mdl>,
//               top_component: Box<Fn(&Mdl) -> El<Ms>>, parent_div_id: &str) -> Self {

//    pub fn new(model: Mdl, update: fn(&Ms, RefCell<Mdl>) -> Mdl,
    pub fn new(model: Mdl, update: fn(&Mailbox<Ms>, &Ms, &Mdl) -> Mdl,
        top_component: fn(&Mdl) -> El<Ms>, parent_div_id: &str) -> Self {

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let main_div = document.get_element_by_id(parent_div_id).unwrap();

        Self {
            inner: Rc::new(Inner {
                document,
                main_div: main_div.clone(),
                model: RefCell::new(model),
                update,
                top_component,
                is_updating: RefCell::new(false), // todo temp
                queue: RefCell::new(Vec::new()), // todo temp

                el_vdom: RefCell::new(div ! []), // todo how should we init? Temp?
                el_ws: RefCell::new(main_div),
            })
        }
    }

    /// Convert from an element in our own virtual dom, to a web_sys element.  Note that
/// we're unable to implmement this directly as impl From in web_sys::Element, due to issues
/// converting between web_sys::Node and web_sys::Element.
    fn el_vdom_to_websys(&self, el_vdom: &El<Ms>) -> web_sys::Element {
        let el_ws = self.inner.document.create_element(el_vdom.tag.as_str()).unwrap();
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

//        for (vdom_event, message) in &el_vdom.events.vals {

//            let handler = move |ws_event: web_sys::Event| {
//                &ws_event.prevent_default();
//                web_sys::console::log_1(&ws_event.into());
//                // todo here magic must happen.
////                &self.inner.is_updating.replace(false);
//
//
//
//            };

//            add_event(&el_ws, vdom_event, handler);
//        }

        el_ws
    }

    /// Create a web_sys dom tree from one using our own format, by recursively iterating
    /// through nodes.
    /// &'stataic on El for use with name/cow
    /// //<N: Into<crate::S>
    fn process_children(&self, active_el_vdom: &El<Ms>, active_el_ws: web_sys::Element) {

        // todo may not be the place to add listeners.
//        let mut listeners: Vec<Listener<Ms>> = Vec::new();
//        for (vdom_event, message) in active_el_vdom.events.vals.into_iter() {
//
////            let hander = |_| web_sys::console::log_1(&"A modicum of success".into());
//            let handler : impl FnMut(web_sys::Event) -> Ms + 'static = |_| message;
////            let handler : impl FnMut(web_sys::Event) = |_| web_sys::console::log_1(&"A modicum of success".into());
//
//            let listener = Listener {
////                name: Cow::from(vdom_event.as_str()),
////                name: vdom_event.as_str(),
//                name: String::from(vdom_event.as_str()),
////                name: 'static: vdom_event.as_str().into(),
//                handler: Some(Box::new(handler)),
//                closure: None
//            };
//            listeners.push(listener)
//        }

        let el_ws = &self.el_vdom_to_websys(active_el_vdom);
        active_el_ws.append_child(&el_ws).unwrap();

        for child in &active_el_vdom.children {
            self.process_children(&child, el_ws.clone());
        }

    }

    /// This runs whenever the state is changed, ie the user-written update function is called.
    /// It updates the state, and any DOM elements affected by this change.
    /// todo this is where we need to compare against differences and only update nodes affected
    /// by teh state change.
    fn update_dom(&self) {
        let top_el = (self.inner.top_component)(&self.inner.model.borrow());
        self.process_children(&top_el, self.inner.main_div.clone());
    }

    fn send(&self, message: Ms) {
        web_sys::console::log_1(&"UPDATE2".into());
        if *self.inner.is_updating.borrow() {
            self.inner.queue.borrow_mut().push(message);
            return;
        }
        self.inner.is_updating.replace(true);
        let mailbox = self.mailbox();
        let updated_model = (self.inner.update)(&mailbox, &message, &self.inner.model.borrow());
        self.inner.model.replace(updated_model);
        while !self.inner.queue.borrow().is_empty() {
            let message = self.inner.queue.borrow_mut().remove(0);
            let updated_model = (self.inner.update)(&mailbox, &message, &self.inner.model.borrow());
            self.inner.model.replace(updated_model);
        }
        self.inner.is_updating.replace(false);
//        self.render();

        self.update_dom();
    }


//    fn render_(&self) {
//        let mut new_el_vdom = (self.inner.top_component)(&self.inner.model.borrow());
//        let new_el_ws = new_el_vdom.patch(&mut self.inner.el_vdom.borrow_mut(), self.mailbox());
//
////        let new_el_ws = el_vdom_to_websys(&self.inner.document, &new_el_vdom);
//        self.inner.el_vdom.replace(new_el_vdom);
//
////        self.inner.main_div.append_child(&new_el_ws);
//
//
////        self.update_dom();
//
//        self.inner.el_ws.replace(new_el_ws);
//
//    }

    fn mailbox(&self) -> Mailbox<Ms> {
        web_sys::console::log_1(&"UPDATE33".into());
        let cloned = self.clone();
        Mailbox::new(move |message| {
            web_sys::console::log_1(&"UPDATE".into());
            cloned.send(message);
        })
    }
}

impl<Ms: Sized + 'static , Mdl: Sized + 'static> std::clone::Clone for Instance<Ms, Mdl> {
    fn clone(&self) -> Self {
        Instance {
            inner: Rc::clone(&self.inner),
        }
    }
}

// The entry point for user apps; exposed in the prelude.
//pub fn run<Ms: Sized + 'static, Mdl: Sized + 'static>(model: Mdl, update: fn(&Ms, RefCell<Mdl>) -> Mdl,
pub fn run<Ms: Sized + 'static, Mdl: Sized + 'static>(model: Mdl, update: fn(&Mailbox<Ms>, &Ms, &Mdl) -> Mdl,
        top_component: fn(&Mdl) -> El<Ms>, parent_div_id: &str) -> Mailbox<Ms>{
    let instance = Instance::new(model, update, top_component, parent_div_id);

//    instance.render();

    instance.update_dom();
    instance.mailbox()

}

