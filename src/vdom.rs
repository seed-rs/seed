use std::{cell::RefCell, rc::Rc, boxed::Box};
use futures::Future;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::dom_types::{El, Tag};


// todo: Get rid of the clone assiated with MS everywhere if you can!



pub struct Mailbox<Message: 'static> {
    func: Rc<Fn(Message)>,
}

impl<Ms: 'static> Mailbox<Ms> {
    pub fn new(func: impl Fn(Ms) + 'static) -> Self {
        Mailbox {
            func: Rc::new(func),
        }
    }

    pub fn send(&self, message: Ms) {
        (self.func)(message)
    }
}

impl<Ms> Clone for Mailbox<Ms> {
    fn clone(&self) -> Self {
        Mailbox {
            func: self.func.clone(),
        }
    }
}

/// Used as part of an interior-mutability pattern, ie Rc<RefCell<>>
struct Data<Ms: Clone + Sized + 'static , Mdl: Sized + 'static> {
    document: web_sys::Document,
    main_div: web_sys::Element,
    model: RefCell<Mdl>,
    update: fn(&Ms, &Mdl) -> Mdl,
    main_component: fn(&Mdl) -> El<Ms>,

    main_el_vdom: RefCell<El<Ms>>,

    ids: Vec<u32>
}

pub struct App<Ms: Clone + Sized + 'static , Mdl: Sized + 'static> {
    inner: Rc<Data<Ms, Mdl>>
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetative sequences of parameters.
impl<Ms: Clone + Sized + 'static, Mdl: Sized + 'static> App<Ms, Mdl> {
    pub fn new(model: Mdl, update: fn(&Ms, &Mdl) -> Mdl,
        top_component: fn(&Mdl) -> El<Ms>, parent_div_id: &str) -> Self {

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let main_div = document.get_element_by_id(parent_div_id).unwrap();

        Self {
            inner: Rc::new(Data {
                document,
                main_div: main_div.clone(),
                model: RefCell::new(model),
                update,
                main_component: top_component,

                main_el_vdom: RefCell::new(div ! []),
                ids: Vec::new(),
            })
        }
    }

    /// This runs whenever the state is changed, ie the user-written update function is called.
    /// It updates the state, and any DOM elements affected by this change.
    /// todo this is where we need to compare against differences and only update nodes affected
    /// by the state change.
    ///
    /// We re-create the whole virtual dom each time (Is there a way around this? Probably not without
    /// knowing what vars the model holds ahead of time), but only edit the rendered, web_sys dom
    /// for things that have been changed.
//    fn update_dom(&self, old_state: &Mdl) {
    fn update_dom(&self, message: Ms) {
        // Note that we re-render the virtual DOM on every change, but (attempt to) only change
        // the actual DOM, via web_sys, when we need.
        // The model storred in inner is the old model; updated_model is a newly-calculated one.
        // todo we should cach top_el when creating it, as inner.top_el
        let updated_model = (self.inner.update)(&message, &self.inner.model.borrow());

        let mut topel_new_vdom = (self.inner.main_component)(&updated_model);
        populate_nest_levels(&mut topel_new_vdom, 0);

        self.inner.model.replace(updated_model);

        self.patch(&mut self.inner.main_el_vdom.borrow_mut(), &mut topel_new_vdom);

        // todo these 4 lines shold go away once you get patch working.
//        let el_ws_new = topel_new_vdom.make_websys_el(&self.inner.document, &self.inner.ids, self.mailbox());
//        self.inner.main_div.set_inner_html("");
//        self.inner.main_div.append_child(&el_ws_new).unwrap();
        // The websys el will now "live" with its vdom el, to make make patching easier.
//        topel_new_vdom.el_ws = Some(el_ws_new);

        self.inner.main_el_vdom.replace(topel_new_vdom);
    }

    fn mailbox(&self) -> Mailbox<Ms> {
        let cloned = self.clone();
        Mailbox::new(move |message| {
            cloned.update_dom(message);
        })
    }

    // Note: Attached to the struct due to use of mailbox method.
    fn patch(&self, old: &mut El<Ms>, new: &mut El<Ms>) {
        // Todo: Current sceme is that if the parent changes, redraw all children...
        // todo fix this later.
        // We make an assumption that most of the page is not dramatically changed
        // by each event, to optimize.
        // todo: There are a lot of ways you could make this more sophisticated.

        // todo only redraw teh whole subtree if children are diff; if it's
        // todo just text or attrs etc, patch them.

        let mut el_ws_to_patch = old.el_ws.expect("Old node is None.");

        if old != new {
            // Something about this node itself is different: patch it.

            if old.tag != new.tag {
                // You can't change the tag in the DOM directly; need to create a new element.
                let new_el_ws = new.make_websys_el(&self.inner.document, &self.inner.ids, self.mailbox());
                &el_ws_to_patch.parent_node().unwrap().replace_child(
                    &new_el_ws, &el_ws_to_patch
                ).unwrap();
                new.el_ws = Some(new_el_ws);
            }

            // todo clean out old attrs that aren't in new
            if old.attrs != new.attrs {
                for (key, new_val) in &new.attrs.vals {
                    match old.attrs.vals.get(key) {
                        Some(old_val) => {
                            // The value's different
                            if old_val != new_val {
                                el_ws_to_patch.set_attribute(key, new_val).unwrap();
                            }
                        },

                        None => el_ws_to_patch.set_attribute(key, new_val).unwrap()
                    }
                }
                // todo style and events.
                }


            if old.children == new.children {
                // We've pached the el itself, and its children match; we're done.
                return
            }
        }
        // If we didn't return due to the children equality check, we need to recursively
        // run this function for the children.

        // The element itself appears to be the same, but its children may have changed,
        // or we've picked a spoofer element. (If we did, we'll probably have to rerender
        // all the children... Try to be smarter about picking the right one. Count children
        // or analyze tag of children to assist?

        // For each of the children of the new vdom element, find the first "matching"
        // el of the old vdom, and assume it's right for this iteration of the recursion.
//        for child_new in &mut new.children {
//            let mut found_match = false;
//
//            // We've found a good-enough match; treat it as the equiv element that
//            // we pass in, to check its children.
//            for child_old in &old.children {
//                if child_new == child_old {
//                    found_match = true;
//                    self.patch(child_old, child_new);
//                    break;
//                }
//            }
//
//            // The child was not present on the old version; create it.
//            if found_match == false {
//                // We need to create a new child element.
//                let new_el_ws = child_new.make_websys_el(&self.inner.document, &self.inner.ids, self.mailbox());
//                el_ws_to_patch.append_child(&new_el_ws);
//                child_new.el_ws = new_el_ws
//
//                // calling make_websys_el creates all children for this, so no need
//                // to enter the recursion again here.
//
//                // todo delete extra children!
//            }
//
//
//        }
//
    }

}

impl<Ms: Clone + Sized + 'static , Mdl: Sized + 'static> std::clone::Clone for App<Ms, Mdl> {
    fn clone(&self) -> Self {
        App {
            inner: Rc::clone(&self.inner),
        }
    }
}



// The entry point for user apps; exposed in the prelude.
//pub fn run<Ms: Sized + 'static, Mdl: Sized + 'static>(model: Mdl, update: fn(&Ms, RefCell<Mdl>) -> Mdl,
pub fn run<Ms: Clone + Sized + 'static, Mdl: Sized + 'static>(model: Mdl, update: fn(&Ms, &Mdl) -> Mdl,
        main_component: fn(&Mdl) -> El<Ms>, main_div_id: &str) {
    let app = App::new(model, update, main_component, main_div_id);


    // Our initial render. Can't initialize in new due to mailbox() requiring self.
    let mut main_el_vdom = (app.inner.main_component)(&app.inner.model.borrow());
    populate_nest_levels(&mut main_el_vdom, 0);
    let main_el_ws = main_el_vdom.make_websys_el(&app.inner.document, &app.inner.ids, app.mailbox());


    app.inner.main_div.set_inner_html("");

    app.inner.main_div.append_child(&main_el_ws).unwrap();
    // The websys el will now "live" with its vdom el, to make make patching easier.
    main_el_vdom.el_ws = Some(main_el_ws);

    app.inner.main_el_vdom.replace(main_el_vdom);
}


fn add_id(ids: Vec<u32>) -> Vec<u32> {
    let new_id = ids.last().unwrap() + 1;
    let mut result = ids;
    result.push(new_id);
    result
}

fn populate_nest_levels<Ms: Clone>(el_vdom: &mut El<Ms>, active_level: u32) {
    // Todo perhaps we populate this while making el_wses, to
    // todo avoid teh duplicate recursion.
    el_vdom.nest_level = Some(active_level);
    for child in &mut el_vdom.children {
        populate_nest_levels(child, active_level + 1)
    }
}
