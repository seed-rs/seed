use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc, boxed::Box};

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

    ids: Vec<u32>,

    // todo recell required?
    els_ws: RefCell<HashMap<u32, web_sys::Element>>
}

pub struct App<Ms: Clone + Sized + 'static , Mdl: Sized + 'static> {
    data: Rc<Data<Ms, Mdl>>
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
            data: Rc::new(Data {
                document,
                main_div: main_div.clone(),
                model: RefCell::new(model),
                update,
                main_component: top_component,

                main_el_vdom: RefCell::new(El::empty(Tag::Div)),
                ids: Vec::new(),

                els_ws: RefCell::new(HashMap::new()),
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
    /// We re-render the virtual DOM on every change, but (attempt to) only change
    /// the actual DOM, via web_sys, when we need.
    /// The model storred in inner is the old model; updated_model is a newly-calculated one.
    fn update_dom(&self, message: Ms) {
        // data.model is the old model; pass it to the update function created in the app,
        // which outputs an updated model.
        let updated_model = (self.data.update)(&message, &self.data.model.borrow());

        // Create a new vdom: The top element, and all its children. Does not yet
        // have ids, nest levels, or associated web_sys elements.
        let mut topel_new_vdom = (self.data.main_component)(&updated_model);

        // We're now done with this updated model; store it for use as the old
        // model for the next update.
        self.data.model.replace(updated_model);

        // We setup the vdom (which populates web_sys els through it, but don't
        // render them with attach_children; we try to do it cleverly via patch().
        self.setup_vdom(&mut topel_new_vdom, 0, 0);
//        self.attach(&mut topel_new_vdom, &self.data.main_div);

        // We haven't updated data.main_el_vdom, so we use it as our old (previous) state.
        self.patch(&mut self.data.main_el_vdom.borrow_mut(), &mut topel_new_vdom, &self.data.main_div);

        // Now that we've re-rendered, replace our stored El with the new one;
        // it will be used as the old El next update.
        self.data.main_el_vdom.replace(topel_new_vdom);
    }

    fn mailbox(&self) -> Mailbox<Ms> {
        let cloned = self.clone();
        Mailbox::new(move |message| {
            cloned.update_dom(message);
        })
    }

    fn find_next_id(&self) -> u32 {
        let el = self.data.main_el_vdom.borrow();
        let mut highest_id = el.id.expect("Missing id");
        for child in &el.children {
            if child.id.expect("Missing id") > highest_id { highest_id += 1; }
        }
        highest_id
    }

    // Note: Attached to the struct due to use of mailbox method.
    fn patch(&self, old: &mut El<Ms>, new: &mut El<Ms>, parent: &web_sys::Element) {
        // Todo: Current sceme is that if the parent changes, redraw all children...
        // todo fix this later.
        // We make an assumption that most of the page is not dramatically changed
        // by each event, to optimize.
        // todo: There are a lot of ways you could make this more sophisticated.

        // Assume setup_vdom has been run on the new el, but only the old vdom's nodes are attached.

        // todo only redraw teh whole subtree if children are diff; if it's
        // todo just text or attrs etc, patch them.

        // take removes the interior value from the Option; otherwise we run into problems
        // about not being able to remove from borrowed content.
        // We remove it from the old el_vodom now, and at the end... add it to the new one.
        // We don't run attach_childre() when patching, hence this approach.
        let old_el_ws = old.el_ws.take().expect("No old elws");

        if old != new {
            // Something about this element itself is different: patch it.
            // At this step, we already assume we have the right element - either
            // by entering this func directly for the top-level, or recursively after
            // analyzing children

            if old.tag != new.tag {
                // You can't change the tag in the DOM directly; need to create a new element.
                // If this changed, we probably couldn't find a suitable match to begin with, and
                // will likely have to re-render all children.
                let new_el_ws = new.el_ws.take().expect("No new elws");
                old_el_ws.parent_node().unwrap().replace_child(
                    &new_el_ws, &old_el_ws
                ).unwrap();
                new.el_ws.replace(new_el_ws);

                self.attach(new, parent);

                // We've re-rendered this child and all children; we're done with this recursion.
                return
            }

            // Patch attributes.
            patch_el_details(old, new, &old_el_ws);
        }

        // If there are the same number of children, assume there's a 1-to-1 mapping,
        // where we will not add or remove any; but patch as needed.
        if old.children.len() == new.children.len() {
            crate::log("Same children len");

            // There's probably a more sophisticated way, but for now, this should be OK.

            let mut avail_old_children = &mut old.children;
            for child_new in &mut new.children {
                let mut scores: Vec<(u32, f32)> = avail_old_children.iter()
                    .map(|c| (c.id.unwrap(), match_score(c, child_new))).collect();
                // should put highest score at the end.
                scores.sort_by(|b, a| b.1.partial_cmp(&a.1).unwrap());

                // Sorting children vice picking the best one makes this easier to handle
                // without irking the borrow checker, despite appearing less counter-intuitive,
                // due to the convenient pop method.
                avail_old_children.sort_by(|b, a| {
                    scores.iter().find(|s| s.0 == b.id.unwrap()).unwrap().1.partial_cmp(
                    &scores.iter().find(|s| s.0 == a.id.unwrap()).unwrap().1
                    ).unwrap()
                });

                let mut best_match = avail_old_children.pop().expect("Probably popping");
                crate::log(&format!("Best: {:?} {:?}, {:?}", &child_new.text, &best_match.text, &best_match.id.unwrap()));

//                crate::log(&avail_old_children.len().to_string());

                self.patch(&mut best_match, child_new, &old_el_ws); // todo old vs new for par

            }


        } else {
            crate::log("Diff child lens");
            crate::log(&format!("{}, {}", old.children.len(), new.children.len()));
            let mut children_accounted_for = Vec::new();

//            crate::log("Before children loop match");
            for child_new in &mut new.children {
                let mut found_match = false;

                // We've found a good-enough match; treat it as the equiv element that
                // we pass in, to check its children.
                for child_old in &mut old.children {
                    if child_new == child_old { // todo replace this line with a similar check.?
                        crate::log("matched child");
                        found_match = true;
                        children_accounted_for.push(child_old.id.unwrap());
                        // We've found a child that matches in terms of tag, attrs, style etc,
                        // so treat it as the right one, and recursively patch its children.
//                    crate::log("Pre recursion");
//                        let old_ws = old.el_ws.take().expect("Missing el_ws on old");
                        self.patch(child_old, child_new, &old_el_ws);  // todo old vs new for par
//                        old.el_ws.replace(old_ws);
//                    crate::log("Post recursion");
                        break;
                    }
                }

                // The child was not present on the old version; create it.
                // todo QC this. Didn't we already take this??
//            let el_ws_to_patch = &old.el_ws.take().expect("No old node");
                crate::log("Before found match");
                if found_match == false {
                    // We need to create a new child element.
                    let new_el_ws = child_new.make_websys_el(&self.data.document, &self.data.ids, self.mailbox());
                    crate::log("after new_el_ws");
                    old_el_ws.append_child(&new_el_ws);
                    crate::log("after child appended");

                    // calling setup_vdom creates all children for this, so no need
                    // to enter the recursion again here.
                    crate::log("Before setup vdom");
                    self.setup_vdom(child_new, child_new.nest_level.expect("Missing nest level"),
                                    self.find_next_id());
                    crate::log("after setup vdom");
                    // todo attach children.
//                let mut future_parent = &mut new.quick_clone();
//                self.attach_children(child_new, Some(&mut future_parent));
                }
            }
        }
//        for child_old in &old.children {
//            if !children_accounted_for.contains(&child_old.id.unwrap()) {
//                // todo convert from el to node
//                let node: web_sys::Node = child_old.quick_clone().el_ws.unwrap().into();
//                el_ws_to_patch.remove_child(&node).expect("Error removing child)");
//            }
//        }


        // Apply the el, which was previously bound to the old vdom el, to the new one.
        new.el_ws = Some(old_el_ws);
    }


    /// Populate the attached web_sys elements, ids, and nest-levels. Run this after creating a vdom, but before
    /// using it to process the web_sys dom. Does not attach children in the DOM. Run this on the top-level element.
    fn setup_vdom(&self, el_vdom: &mut El<Ms>, active_level: u32, active_id: u32) {
        // Active id iterates once per item; active-level once per nesting level.
        let mut id = active_id;
        el_vdom.id = Some(id);
        id += 1;  // Raise the id after each element we process.
        el_vdom.nest_level = Some(active_level);

        // Create the web_sys element; add it to the working tree; store it in
        // its corresponding vdom El.
        let el_ws = el_vdom.make_websys_el(&self.data.document, &self.data.ids, self.mailbox());
        el_vdom.el_ws = Some(el_ws);
        for child in &mut el_vdom.children {
            // Raise the active level once per recursion.
            self.setup_vdom(child, active_level + 1, id);
            id += 1;
        }
    }

    // Attaches the element, and all children, recursively. Only run this when creating a fresh vdom node, since
    // it performs a rerender of the el and all children; eg a potentially-expensive op.
    fn attach(&self, el_vdom: &mut El<Ms>, parent: &web_sys::Element) {
        // No parent means we're operating on the top-level element; append it to the main div.
        // This is how we call this function externally, ie not through recursion.
        let el_ws = el_vdom.el_ws.take().expect("Missing websys el");

        // Append its child while it's out of its element.
        crate::log("Rendering child in attach");
        parent.append_child(&el_ws);

        for child in &mut el_vdom.children {
            // Raise the active level once per recursion.
            self.attach(child, &el_ws)
        }

        // Replace the web_sys el... Indiana-Jones-style.
        el_vdom.el_ws.replace(el_ws);
    }

}

impl<Ms: Clone + Sized + 'static , Mdl: Sized + 'static> std::clone::Clone for App<Ms, Mdl> {
    fn clone(&self) -> Self {
        App {
            data: Rc::clone(&self.data),
        }
    }
}

fn add_id(ids: Vec<u32>) -> Vec<u32> {
    let new_id = ids.last().unwrap() + 1;
    let mut result = ids;
    result.push(new_id);
    result
}




/// Compare two elements. Rank based on how similar they are, using subjective criteria.
///
fn match_score<Ms: Clone>(old: &El<Ms>, new: &El<Ms>) -> f32 {
    // todo: No magic numbers
    let mut score = 0.;

    // Tags are not likely to change! Good indicator of it being the wrong element.
    if old.tag == new.tag { score += 0.3 };
    // Attrs are not likely to change.
    // todo: Compare attrs more directly.
    if old.attrs == new.attrs { score += 0.15 };
    // Style is likely to change.
    if old.style == new.style { score += 0.05 };
    // Text is likely to change.
    if old.text == new.text { score += 0.05 };

    // todo nest level?

    // For children length, don't do it based on the difference, since children that actually change in
    // len may have very large changes. But having identical length is a sanity check.
    if old.children.len() == new.children.len() { score += 0.1 };

    // Same id implies it may have been added in the same order.
    if old.id.expect("Missing id") == new.id.expect("Missing id") { score += 0.15 };

    // todo check children a level or two down.
    // todo check types of children
    let mut old_tags: Vec<&Tag> = old.children.iter().map(|c| &c.tag).collect();
    let mut new_tags: Vec<&Tag> = new.children.iter().map(|c| &c.tag).collect();

    score
}

// The entry point for user apps; exposed in the prelude.
pub fn run<Ms: Clone + Sized + 'static, Mdl: Sized + 'static>(model: Mdl, update: fn(&Ms, &Mdl) -> Mdl,
        main_component: fn(&Mdl) -> El<Ms>, main_div_id: &str) {
    let app = App::new(model, update, main_component, main_div_id);

    // Our initial render. Can't initialize in new due to mailbox() requiring self.
    let mut main_el_vdom = (app.data.main_component)(&app.data.model.borrow());
    app.setup_vdom(&mut main_el_vdom, 0, 0);
    // Attach all children: This is where our initial render occurs.
    app.attach(&mut main_el_vdom, &app.data.main_div);

    app.data.main_el_vdom.replace(main_el_vdom);
}


/// Update the attributes, style, text, and events of an element. Does not
/// process children, and assumes the tag is the same. Assume we've identfied
/// the most-correct pairing between new and old.
pub fn patch_el_details<Ms: Clone>(old: &mut El<Ms>, new: &mut El<Ms>, old_el_ws: &web_sys::Element) {
    if old.attrs != new.attrs {
        for (key, new_val) in &new.attrs.vals {
            match old.attrs.vals.get(key) {
                Some(old_val) => {
                    // The value's different
                    if old_val != new_val {
                        old_el_ws.set_attribute(key, new_val).expect("Replacing attribute");
                    }
                },
                None => old_el_ws.set_attribute(key, new_val).expect("Adding a new attribute")
            }
        }
        // Remove attributes that aren't in the new vdom.
        for (key, old_val) in &old.attrs.vals {
            if new.attrs.vals.get(key).is_none() {
                old_el_ws.remove_attribute(key).expect("Removing an attribute");
            }
        }
    }

    // Patch style.
    if old.style != new.style {
        // We can't patch each part of style; rewrite the whole attribute.
        old_el_ws.set_attribute("style", &new.style.as_str())
            .expect("Setting style");
    }

    // Patch text
    if old.text != new.text {
        // It appears that at this point, there is no way to manage Option comparison more directly.
        let text = new.text.clone().unwrap_or(String::new());
        old_el_ws.set_text_content(Some(&text));
    }

    // todo events.
}
