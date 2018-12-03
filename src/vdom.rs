use std::{cell::RefCell, rc::Rc, boxed::Box};

use crate::dom_types::{El, Events, Event, Tag};
use crate::mailbox; // todo temp?

use self::mailbox::Mailbox; // todo temp


/// Used as part of an interior-mutability pattern, ie Rc<RefCell<>>
struct Inner<Ms: Sized + 'static , Mdl: Sized + 'static> {
    document: web_sys::Document,
    main_div: web_sys::Element,
    model: RefCell<Mdl>,
    update: fn(&Ms, &Mdl) -> Mdl,
    top_component: fn(&Mdl) -> El<Ms>,

    el_ws: RefCell<web_sys::Element>,
    el_vdom: RefCell<El<Ms>>,

    queue: RefCell<Vec<Ms>>,
    is_updating: RefCell<bool>,
}

pub struct App<Ms: Sized + 'static , Mdl: Sized + 'static> {
    inner: Rc<Inner<Ms, Mdl>>
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetative sequences of parameters.
impl<Ms: Sized + 'static, Mdl: Sized + 'static> App<Ms, Mdl> {
    pub fn new(model: Mdl, update: fn(&Ms, &Mdl) -> Mdl,
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

    /// This runs whenever the state is changed, ie the user-written update function is called.
    /// It updates the state, and any DOM elements affected by this change.
    /// todo this is where we need to compare against differences and only update nodes affected
    /// by teh state change.
    fn update_dom(&self) {
        let mut top_el = (self.inner.top_component)(&self.inner.model.borrow());
//        self.process_children(&top_el, self.inner.main_div.clone());


        let el_ws = top_el.make_websys_el(&self.inner.document, self.mailbox());


        // todo no diffing / patching algo atm; just replace everything.
        self.inner.main_div.set_inner_html("");
        self.inner.main_div.append_child(&el_ws).unwrap();

        self.inner.el_ws.replace(el_ws);
    }

    fn send(&self, message: Ms) {
        if *self.inner.is_updating.borrow() {
            self.inner.queue.borrow_mut().push(message);
            return;
        }
        self.inner.is_updating.replace(true);
        let updated_model = (self.inner.update)(&message, &self.inner.model.borrow());
        self.inner.model.replace(updated_model);
        while !self.inner.queue.borrow().is_empty() {
            let message = self.inner.queue.borrow_mut().remove(0);
            let updated_model = (self.inner.update)(&message, &self.inner.model.borrow());
            self.inner.model.replace(updated_model);
        }
        self.inner.is_updating.replace(false);

        self.update_dom();
    }

    fn mailbox(&self) -> Mailbox<Ms> {
        let cloned = self.clone();
        Mailbox::new(move |message| {
            cloned.send(message);
        })
    }
}

impl<Ms: Sized + 'static , Mdl: Sized + 'static> std::clone::Clone for App<Ms, Mdl> {
    fn clone(&self) -> Self {
        App {
            inner: Rc::clone(&self.inner),
        }
    }
}

// The entry point for user apps; exposed in the prelude.
//pub fn run<Ms: Sized + 'static, Mdl: Sized + 'static>(model: Mdl, update: fn(&Ms, RefCell<Mdl>) -> Mdl,
pub fn run<Ms: Sized + 'static, Mdl: Sized + 'static>(model: Mdl, update: fn(&Ms, &Mdl) -> Mdl,
        top_component: fn(&Mdl) -> El<Ms>, parent_div_id: &str) {
    let app = App::new(model, update, top_component, parent_div_id);
    app.update_dom();
}

