use std::{cell::{Cell, RefCell}, rc::Rc};

use crate::dom_types;
use crate::dom_types::{El};
use crate::websys_bridge;


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

// todo: Examine what needs to be ref cells, rcs etc

/// Used as part of an interior-mutability pattern, ie Rc<RefCell<>>
pub struct Data<Ms: Clone + Sized + 'static , Mdl: Sized + 'static> {
    document: web_sys::Document,
    pub main_div: web_sys::Element,
    // Model is in a RefCell here so we can replace it in self.update_dom().
    pub model: RefCell<Mdl>,
    update: fn(Ms, &Mdl) -> Mdl,
    pub view: fn(Mdl) -> El<Ms>,
    pub main_el_vdom: RefCell<El<Ms>>,
}

pub struct App<Ms: Clone + Sized + 'static , Mdl: Sized + 'static> {
    pub data: Rc<Data<Ms, Mdl>>
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetative sequences of parameters.
impl<Ms: Clone + Sized + 'static, Mdl: Clone + Sized + 'static> App<Ms, Mdl> {
    pub fn new(model: Mdl, update: fn(Ms, &Mdl) -> Mdl,
               view: fn(Mdl) -> El<Ms>, parent_div_id: &str) -> Self {

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let main_div = document.get_element_by_id(parent_div_id).unwrap();


        Self {
            data: Rc::new(Data {
                document,
                main_div: main_div.clone(),
                model: RefCell::new(model),
                update,
                view,

                main_el_vdom: RefCell::new(El::empty(dom_types::Tag::Div)),
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
        let updated_model = (self.data.update)(message, &self.data.model.borrow());

        // Create a new vdom: The top element, and all its children. Does not yet
        // have ids, nest levels, or associated web_sys elements.
        // We accept cloning here, for the benefit of making data easier to work
        // with in the app.
        let mut topel_new_vdom = (self.data.view)(updated_model.clone());

        // We're now done with this updated model; store it for use as the old
        // model for the next update.
        // Note: It appears that this step is why we need data.model to be in a RefCell.
        self.data.model.replace(updated_model);

        // We setup the vdom (which populates web_sys els through it, but don't
        // render them with attach_children; we try to do it cleverly via patch().
        self.setup_vdom(&mut topel_new_vdom, 0, 0);

        // We haven't updated data.main_el_vdom, so we use it as our old (previous) state.
        self.patch(&mut self.data.main_el_vdom.borrow_mut(), &mut topel_new_vdom, &self.data.main_div);

        // Now that we've re-rendered, replace our stored El with the new one;
        // it will be used as the old El next (.
        self.data.main_el_vdom.replace(topel_new_vdom);
    }

    fn mailbox(&self) -> Mailbox<Ms> {
        let cloned = self.clone();
        Mailbox::new(move |message| {
            cloned.update_dom(message);
        })
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
        // We don't run attach_children() when patching, hence this approach.

//        if new.is_dummy() == true { return }

        let old_el_ws = old.el_ws.take().expect("No old elws");

        if old != new {
            // Something about this element itself is different: patch it.
            // At this step, we already assume we have the right element - either
            // by entering this func directly for the top-level, or recursively after
            // analyzing children
            if old.tag != new.tag {
                parent.remove_child(&old_el_ws).expect("Problem removing this element");
                websys_bridge::attach(new, parent);
                // We've re-rendered this child and all children; we're done with this recursion.
                return
            }

            // Patch attributes.
            patch_el_details(old, new, &old_el_ws, &self.data.document, self.mailbox());
        }

        // If there are the same number of children, assume there's a 1-to-1 mapping,
        // where we will not add or remove any; but patch as needed.

        // A more sophisticated approach would be to find the best match of every
        // combination of score of new vs old, then rank them somehow. (Eg even
        // if old id=2 is the best match for the first new, if it's only a marginal
        // winner, but a strong winner for the second, it makes sense to put it
        // in the second, but we are not allowing it this opporunity as-is.
        // One approach would be check all combinations, combine scores within each combo, and pick the one
        // with the highest total score, but this increases with the factorial of
        // child size!
        // todo: Look into this improvement sometime after the initial release.

        let avail_old_children = &mut old.children;
        for child_new in &mut new.children {
            if avail_old_children.is_empty() {
                // One or more new children has been added, or much content has
                // changed, or we've made a mistake: Attach new children.
                websys_bridge::attach(child_new, &old_el_ws);
            } else {

                // We still have old children to pick a match from. If we pick
                // incorrectly, or there is no "good" match, we'll have some
                // patching and/or attaching (rendering) to do in subsequent recursions.
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

                self.patch(&mut best_match, child_new, &old_el_ws); // todo old vs new for par
              }
        }

        // Now purge any existing children; they're not part of the new model.
        for child in avail_old_children {
            let child_el_ws = child.el_ws.take().expect("Missing child el_ws");
            old_el_ws.remove_child(&child_el_ws).expect("Problem removing child");
            child.el_ws.replace(child_el_ws);
        }

        new.el_ws = Some(old_el_ws);
    }

    /// Populate the attached web_sys elements, ids, and nest-levels. Run this after creating a vdom, but before
    /// using it to process the web_sys dom. Does not attach children in the DOM. Run this on the top-level element.
    pub fn setup_vdom(&self, el_vdom: &mut El<Ms>, active_level: u32, active_id: u32) {
        // id iterates once per item; active-level once per nesting level.
        let mut id = active_id;
        el_vdom.id = Some(id);
        id += 1;  // Raise the id after each element we process.
        el_vdom.nest_level = Some(active_level);

        // Create the web_sys element; add it to the working tree; store it in
        // its corresponding vdom El.
        let el_ws = websys_bridge::make_websys_el(el_vdom, &self.data.document, self.mailbox());
        el_vdom.el_ws = Some(el_ws);
        for child in &mut el_vdom.children {
            // Raise the active level once per recursion.
            self.setup_vdom(child, active_level + 1, id);
            id += 1;
        }
    }
}

impl<Ms: Clone + Sized + 'static , Mdl: Sized + 'static> std::clone::Clone for App<Ms, Mdl> {
    fn clone(&self) -> Self {
        App {
            data: Rc::clone(&self.data),
        }
    }
}

/// Compare two elements. Rank based on how similar they are, using subjective criteria.
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
    // Text is likely to change, but may still be a good indicator.
    if old.text == new.text { score += 0.05 };

    // todo nest level?
    // For children length, don't do it based on the difference, since children that actually change in
    // len may have very large changes. But having identical length is a sanity check.
    if old.children.len() == new.children.len() {
        score += 0.1
    } else if (old.children.len() as i16 - new.children.len() as i16).abs() == 1 {
        // Perhaps we've added or removed a child.
        score += 0.05
    }
    // Same id implies it may have been added in the same order.
    if old.id.expect("Missing id") == new.id.expect("Missing id") { score += 0.15 };

    // todo check children a level or two down.
    // todo check types of children
    let _old_tags: Vec<&dom_types::Tag> = old.children.iter().map(|c| &c.tag).collect();
    let _new_tags: Vec<&dom_types::Tag> = new.children.iter().map(|c| &c.tag).collect();

    // todo: Recursively (or shallowly?) score children? we really must think of the children
    // todo seriously; we'll make lots of mistakes otherwise
    score
}


/// Update the attributes, style, text, and events of an element. Does not
/// process children, and assumes the tag is the same. Assume we've identfied
/// the most-correct pairing between new and old.
pub fn patch_el_details<Ms: Clone>(old: &mut El<Ms>, new: &mut El<Ms>,
           old_el_ws: &web_sys::Element, document: &web_sys::Document, mailbox: Mailbox<Ms>) {

    if old.attrs != new.attrs {
        for (key, new_val) in &new.attrs.vals {
            match old.attrs.vals.get(key) {
                Some(old_val) => {
                    // The value's different
                    if old_val != new_val {
                        websys_bridge::set_attr_shim(&old_el_ws, key, new_val);
                    }
                },
                None => old_el_ws.set_attribute(key, new_val).expect("Adding a new attribute")
            }
        }
        // Remove attributes that aren't in the new vdom.
        for (name, old_val) in &old.attrs.vals {
            if new.attrs.vals.get(name).is_none() {
                old_el_ws.remove_attribute(name).expect("Removing an attribute");
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
        // This is not as straightforward as it looks: There can be multiple text nodes
        // in the DOM, even though our API only allows for 1 per element. If we
        // naively run set_text_content(), all child nodes will be removed.
        // Text is stored in special Text nodes that don't have a direct-relation to
        // the vdom.

        let text = new.text.clone().unwrap_or_default();

        if old.text.is_none() {
            // There's no old node to find: Add it.
            let new_next_node = document.create_text_node(&text);
            old_el_ws.append_child(&new_next_node).unwrap();
        } else {
            // Iterating over a NodeList, unfortunately, is not as clean as you might expect.
            let children = old_el_ws.child_nodes();
            for i in 0..children.length() {
                let node = children.item(i).unwrap();
                // We've found it; there will be not more than 1 text node.
                if node.node_type() == 3 {
                    node.set_text_content(Some(&text));
                    break;
                }
            }
        }


    }

    for listener in &mut old.listeners {
//        listener.detach(&old_el_ws);
    }

    // todo detach old ones too!
    for listener in &mut new.listeners {
//        listener.attach(&old_el_ws, mailbox.clone());
    }

//    if old.listeners != new.listeners {
//        crate::log("WOAH");
//    } else { crate::log("SAME")}
}
