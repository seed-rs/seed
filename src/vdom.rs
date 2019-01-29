use crate::{
    dom_types,
    dom_types::{El, Namespace},
    routing, util, websys_bridge,
};
use std::{cell::RefCell, collections::HashMap, panic, rc::Rc};
use wasm_bindgen::closure::Closure;
use web_sys::{Document, Element, Event, EventTarget, Window};

pub enum Update<Mdl: Clone> {
    Render(Mdl),
    Skip(Mdl),
}

type UpdateFn<Ms, Mdl> = fn(Ms, Mdl) -> Update<Mdl>;
type ViewFn<Ms, Mdl> = fn(App<Ms, Mdl>, &Mdl) -> El<Ms>;
type Routes<Ms> = HashMap<String, Ms>;
//type Routes<Ms> = HashMap<Vec<String>, Fn() -> Ms>;
type WindowEvents<Ms, Mdl> = fn(Mdl) -> Vec<dom_types::Listener<Ms>>;

pub struct Mailbox<Message: 'static> {
    func: Rc<Fn(Message)>,
}

impl<Ms> Mailbox<Ms> {
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

// TODO: Examine what needs to be ref cells, rcs etc

type StoredPopstate = RefCell<Option<Closure<FnMut(Event)>>>;

/// Used as part of an interior-mutability pattern, ie Rc<RefCell<>>
pub struct AppData<Ms: Clone + 'static, Mdl: Clone> {
    // Model is in a RefCell here so we can replace it in self.update().
    pub model: RefCell<Mdl>,
    main_el_vdom: RefCell<El<Ms>>,
    pub popstate_closure: StoredPopstate,
    pub routes: RefCell<Routes<Ms>>,
    window_listeners: RefCell<Vec<dom_types::Listener<Ms>>>,
    msg_listeners: RefCell<Vec<Box<Fn(&Ms)>>>,
}

pub struct AppCfg<Ms: Clone + 'static, Mdl: Clone + 'static> {
    document: web_sys::Document,
    mount_point: web_sys::Element,
    pub update: UpdateFn<Ms, Mdl>,
    view: ViewFn<Ms, Mdl>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
}

pub struct App<Ms: Clone + 'static, Mdl: 'static + Clone> {
    /// Stateless app configuration
    pub cfg: Rc<AppCfg<Ms, Mdl>>,
    /// Mutable app state
    pub data: Rc<AppData<Ms, Mdl>>,
}

#[derive(Clone)]
pub struct AppBuilder<Ms: Clone + 'static, Mdl: 'static + Clone> {
    model: Mdl,
    update: UpdateFn<Ms, Mdl>,
    view: ViewFn<Ms, Mdl>,
    parent_div_id: Option<&'static str>,
    routes: Routes<Ms>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
}

impl<Ms: Clone, Mdl: Clone> AppBuilder<Ms, Mdl> {
    pub fn mount(mut self, id: &'static str) -> Self {
        self.parent_div_id = Some(id);
        self
    }
    pub fn routes(mut self, routes: Routes<Ms>) -> Self {
//        self.routes = Some(routes);
        self.routes = routes;
        self
    }
    pub fn window_events(mut self, evts: WindowEvents<Ms, Mdl>) -> Self {
        self.window_events = Some(evts);
        self
    }
    pub fn finish(self) -> App<Ms, Mdl> {
        let parent_div_id = self.parent_div_id.unwrap_or("app");

        App::new(
            self.model,
            self.update,
            self.view,
            parent_div_id,
            self.routes,
            self.window_events,
        )
    }
}

/// We use a struct instead of series of functions, in order to avoid passing
/// repetative sequences of parameters.
impl<Ms: Clone, Mdl: Clone> App<Ms, Mdl> {
    pub fn build(
        model: Mdl,
        update: UpdateFn<Ms, Mdl>,
        view: ViewFn<Ms, Mdl>,
    ) -> AppBuilder<Ms, Mdl> {
        AppBuilder {
            model,
            update,
            view,
            parent_div_id: None,
            routes: HashMap::new(),
            window_events: None,
        }
    }
    fn new(
        model: Mdl,
        update: UpdateFn<Ms, Mdl>,
        view: ViewFn<Ms, Mdl>,
        parent_div_id: &str,
//        routes: Option<Routes<Ms>>,
        routes: Routes<Ms>,
        window_events: Option<WindowEvents<Ms, Mdl>>,
    ) -> Self {
        let window = util::window();
        let document = window
            .document()
            .expect("Can't find the window's document.");

        let mount_point = document.get_element_by_id(parent_div_id).unwrap();

        Self {
            cfg: Rc::new(AppCfg {
                document,
                mount_point,
                update,
                view,
                window_events,
            }),
            data: Rc::new(AppData {
                model: RefCell::new(model),
                main_el_vdom: RefCell::new(El::empty(dom_types::Tag::Div)),
                popstate_closure: RefCell::new(None),
                routes: RefCell::new(routes),
                window_listeners: RefCell::new(Vec::new()),
                msg_listeners: RefCell::new(Vec::new()),
            }),
        }
    }

    /// App initialization: Collect its fundamental components, setup, and perform
    /// an initial render.
    pub fn run(self) -> Self {
        // Our initial render. Can't initialize in new due to mailbox() requiring self.
        // TODO: maybe have view take an update instead of whole app?
        // TODO: There's a lot of DRY between here and update.
        //    let mut topel_vdom = (app.data.view)(app.clone(), model.clone());

        let window = util::window();

        // Only clone model if we have window events.
        let mut topel_vdom;
        match self.cfg.window_events {
            //TODO: use window events
            Some(_window_events) => {
                topel_vdom = (self.cfg.view)(self.clone(), &self.data.model.borrow());
                setup_window_listeners(
                    &util::window(),
                    &mut Vec::new(),
                    // TODO:
                    // Fix this. Bug where if we try to add initial listeners,
                    // we get many runtime panics. Workaround is to wait until
                    // app.update, which means an event must be triggered
                    // prior to window listeners working.
                    &mut Vec::new(),
                    // &mut (window_events)(model),
                    &self.mailbox(),
                );
            }
            None => {
                topel_vdom = (self.cfg.view)(self.clone(), &self.data.model.borrow());
            }
        }

        let document = window.document().expect("Problem getting document");
        setup_els(&document, &mut topel_vdom, 0, 0);

        attach_listeners(&mut topel_vdom, &self.mailbox());

        // Attach all children: This is where our initial render occurs.
        websys_bridge::attach_els(&mut topel_vdom, &self.cfg.mount_point);

        self.data.main_el_vdom.replace(topel_vdom);

        // Update the state on page load, based
        // on the starting URL. Must be set up on the server as well.
        let routes_inner = self.data.routes.borrow().clone();
        let app2 = routing::initial(self.clone(), routes_inner.clone());
        routing::update_popstate_listener(&app2, routes_inner);

        // Allows panic messages to output to the browser console.error.
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        self
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
    pub fn update(&self, message: Ms) {

        for l in self.data.msg_listeners.borrow().iter() {
            (l)(&message)
        }
        // data.model is the old model; pass it to the update function created in the app,
        // which outputs an updated model.
        // We clone the model before running update, and again before passing it
        // to the view func, instead of using refs, to improve API syntax.
        // This approach may have performance impacts of unknown magnitude.
        let model_to_update = self.data.model.borrow().clone();
        let updated_model = (self.cfg.update)(message, model_to_update);

        let mut should_render = true;
        let updated_model = match updated_model {
            Update::Render(m) => m,
            Update::Skip(m) => {
                should_render = false;
                m
            }
        };

        // Unlike in run, we clone model here anyway, so no need to change top_new_vdom
        // logic based on if we have window listeners.
        if let Some(window_events) = self.cfg.window_events {
            let mut new_listeners = (window_events)(updated_model.clone());
            setup_window_listeners(
                &util::window(),
                &mut self.data.window_listeners.borrow_mut(),
                //                &mut Vec::new(),
                &mut new_listeners,
                &self.mailbox(),
            );
            self.data.window_listeners.replace(new_listeners);
        }

        if should_render {
            // Create a new vdom: The top element, and all its children. Does not yet
            // have ids, nest levels, or associated web_sys elements.
            // We accept cloning here, for the benefit of making data easier to work
            // with in the app.
            let mut topel_new_vdom = (self.cfg.view)(self.clone(), &updated_model);

            // We setup the vdom (which populates web_sys els through it, but don't
            // render them with attach_children; we try to do it cleverly via patch().
            setup_els(&self.cfg.document, &mut topel_new_vdom, 0, 0);

            // Detach all old listeners before patching. We'll re-add them as required during patching.
            // We'll get a runtime panic if any are left un-removed.
            detach_listeners(&mut self.data.main_el_vdom.borrow_mut());

            // We haven't updated data.main_el_vdom, so we use it as our old (previous) state.
            patch(
                &self.cfg.document,
                &mut self.data.main_el_vdom.borrow_mut(),
                &mut topel_new_vdom,
                &self.cfg.mount_point,
                &self.mailbox(),
            );

            // Now that we've re-rendered, replace our stored El with the new one;
            // it will be used as the old El next (.
            self.data.main_el_vdom.replace(topel_new_vdom);
        }

        // We're now done with this updated model; store it for use as the old
        // model for the next update.
        // Note: It appears that this step is why we need data.model to be in a RefCell.
        self.data.model.replace(updated_model);
    }

    pub fn add_route(&self, path: &str, message: Ms) {
            // We're editing a HashMap wrapped in an Option wrapped in an RefCell.
    let mut r = self.data.routes.borrow_mut().clone();
    r.insert(path.to_string(), message.clone());

    routing::update_popstate_listener(&self, r.clone());
    self.data.routes.replace(r);

    // Trigger an update, so the user doesn't have to recursively in the update func.
    }

    pub fn add_message_listener<F>(&self, listener: F)
        where F: Fn(&Ms) + 'static
    {
       self.data.msg_listeners.borrow_mut().push(Box::new(listener));
    }

    fn mailbox(&self) -> Mailbox<Ms> {
        let cloned = self.clone();
        Mailbox::new(move |message| {
            cloned.update(message);
        })
    }
}

/// Populate the attached web_sys elements, ids, and nest-levels. Run this after creating a vdom, but before
/// using it to process the web_sys dom. Does not attach children in the DOM. Run this on the top-level element.
pub fn setup_els<Ms>(document: &Document, el_vdom: &mut El<Ms>, active_level: u32, active_id: u32)
// pub for tests.
where
    Ms: Clone + 'static,
{
    // id iterates once per item; active-level once per nesting level.
    let mut id = active_id;
    el_vdom.id = Some(id);
    id += 1; // Raise the id after each element we process.
    el_vdom.nest_level = Some(active_level);

    // Create the web_sys element; add it to the working tree; store it in
    // its corresponding vdom El.
    let el_ws = websys_bridge::make_websys_el(el_vdom, document);
    el_vdom.el_ws = Some(el_ws);
    for child in &mut el_vdom.children {
        // Raise the active level once per recursion.
        setup_els(document, child, active_level + 1, id);
        id += 1;
    }
}

// trying this approach leads to lifetime problems.
//fn mailbox<Ms, Mdl>(app: &'static App<Ms, Mdl>) -> Mailbox<Ms>
//    where Ms: Clone + 'static, Mdl: Clone + 'static {
//    Mailbox::new(move |message| {
//        app.clone().update(message);
//    })
//
//}

impl<Ms: Clone, Mdl: Clone> Clone for App<Ms, Mdl> {
    fn clone(&self) -> Self {
        App {
            cfg: Rc::clone(&self.cfg),
            data: Rc::clone(&self.data),
        }
    }
}

// TODO: should this be here, or in a diff module?
/// Recursively attach event-listeners. Run this at init.
fn attach_listeners<Ms: Clone>(el: &mut dom_types::El<Ms>, mailbox: &Mailbox<Ms>) {
    let el_ws = el
        .el_ws
        .take()
        .expect("Missing el_ws on attach_all_listeners");

    for listener in &mut el.listeners {
        listener.attach(&el_ws, mailbox.clone());
    }
    for child in &mut el.children {
        attach_listeners(child, mailbox)
    }

    el.el_ws.replace(el_ws);
}

// TODO: should this be here, or in a diff module?
/// Recursively detach event-listeners. Run this before patching.
fn detach_listeners<Ms: Clone>(el: &mut dom_types::El<Ms>) {
    let el_ws = el
        .el_ws
        .take()
        .expect("Missing el_ws on detach_all_listeners");

    for listener in &mut el.listeners {
        listener.detach(&el_ws);
    }
    for child in &mut el.children {
        detach_listeners(child)
    }

    el.el_ws.replace(el_ws);
}

/// We reattach all listeners, as with normal Els, since we have no
/// way of diffing them.
fn setup_window_listeners<Ms: Clone>(
    window: &Window,
    old: &mut Vec<dom_types::Listener<Ms>>,
    new: &mut Vec<dom_types::Listener<Ms>>,
    mailbox: &Mailbox<Ms>,
) {
    for listener in old {
        listener.detach(window);
    }

    for listener in new {
        listener.attach(window, mailbox.clone());
    }
}

fn patch<Ms: Clone>(
    document: &Document,
    old: &mut El<Ms>,
    new: &mut El<Ms>,
    parent: &web_sys::Node,
    mailbox: &Mailbox<Ms>,
) {
    // Old_el_ws is what we're patching, with items from the new vDOM el; or replacing.
    // TODO: Current sceme is that if the parent changes, redraw all children...
    // TODO: fix this later.
    // We make an assumption that most of the page is not dramatically changed
    // by each event, to optimize.
    // TODO: There are a lot of ways you could make this more sophisticated.

    // Assume setup_vdom has been run on the new el, all listeners have been removed
    // from the old el_ws, and the only the old el vdom's elements are still attached.

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

        // If the tag's different, we must redraw the element and its children; there's
        // no way to patch one element type into another.
        // TODO: forcing a rerender for differnet listeners is potentially sloppy,
        // TODO:, but I'm not sure how to patch them, or deal with them.
        if old.tag != new.tag {
            // TODO: DRY here between this and later in func.
            if let Some(unmount_actions) = &mut old.will_unmount {
                unmount_actions(&old_el_ws)
            }
            parent
                .remove_child(&old_el_ws)
                .expect("Problem removing an element");

            websys_bridge::attach_els(new, parent);
            let mut new = new;
            attach_listeners(&mut new, &mailbox);
            // We've re-rendered this child and all children; we're done with this recursion.
            return;
        }

        // Patch parts of the Element.
        websys_bridge::patch_el_details(old, new, &old_el_ws);
    }

    // Before running patch, assume we've removed all listeners from the old element.
    // Perform this attachment after we've verified we can patch this element, ie
    // it has the same tag - otherwise  we'd have to detach after the parent.remove_child step.
    // Note that unlike the attach_listeners function, this only attaches for the currently
    // element.
    for listener in &mut new.listeners {
        listener.attach(&old_el_ws, mailbox.clone());
    }

    // Now pair up children as best we can.
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
    // TODO:: Look into this improvement sometime after the initial release.

    let avail_old_children = &mut old.children;
    for child_new in &mut new.children {
        if avail_old_children.is_empty() {
            // One or more new children has been added, or much content has
            // changed, or we've made a mistake: Attach new children.
            websys_bridge::attach_els(child_new, &old_el_ws);
            let mut child_new = child_new;
            attach_listeners(&mut child_new, &mailbox);
        } else {
            // We still have old children to pick a match from. If we pick
            // incorrectly, or there is no "good" match, we'll have some
            // patching and/or attaching (rendering) to do in subsequent recursions.
            let mut scores: Vec<(u32, f32)> = avail_old_children
                .iter()
                .map(|c| (c.id.unwrap(), match_score(c, child_new)))
                .collect();

            // should put highest score at the end.
            scores.sort_by(|b, a| b.1.partial_cmp(&a.1).unwrap());

            // Sorting children vice picking the best one makes this easier to handle
            // without irking the borrow checker, despite appearing less counter-intuitive,
            // due to the convenient pop method.
            avail_old_children.sort_by(|b, a| {
                scores
                    .iter()
                    .find(|s| s.0 == b.id.unwrap())
                    .unwrap()
                    .1
                    .partial_cmp(&scores.iter().find(|s| s.0 == a.id.unwrap()).unwrap().1)
                    .unwrap()
            });

            let mut best_match = avail_old_children.pop().expect("Probably popping");

            // TODO: do we really need to clone the mb again ehre? Keep this under control!
            patch(document, &mut best_match, child_new, &old_el_ws, &mailbox); // TODO: old vs new for par
        }
    }

    // Now purge any existing children; they're not part of the new model.
    for child in avail_old_children {
        let child_el_ws = child.el_ws.take().expect("Missing child el_ws");

        // TODO: DRY here between this and earlier in func
        if let Some(unmount_actions) = &mut child.will_unmount {
            unmount_actions(&child_el_ws)
        }
        old_el_ws
            .remove_child(&child_el_ws)
            .expect("Problem removing child");

        child.el_ws.replace(child_el_ws);
    }

    new.el_ws = Some(old_el_ws);
}

/// Compare two elements. Rank based on how similar they are, using subjective criteria.
fn match_score<Ms: Clone>(old: &El<Ms>, new: &El<Ms>) -> f32 {
    // children_to_eval is the number of children to look at on each nest level.
    //    let children_to_eval = 3;
    // Don't weight children as heavily as the parent. This effect acculates the further down we go.
    //    let child_score_significance = 0.6;

    let mut score = 0.;

    // Tags are not likely to change! Good indicator of it being the wrong element.
    if old.tag == new.tag {
        score += 0.3
    } else {
        score -= 0.3
    };
    // Attrs are not likely to change.
    // TODO:: Compare attrs more directly.
    if old.attrs == new.attrs {
        score += 0.15
    } else {
        score -= 0.15
    };
    // Style is likely to change.
    if old.style == new.style {
        score += 0.05
    } else {
        score -= 0.05
    };
    // Text is likely to change, but may still be a good indicator.
    if old.text == new.text {
        score += 0.05
    } else {
        score -= 0.05
    };

    // For children length, don't do it based on the difference, since children that actually change in
    // len may have very large changes. But having identical length is a sanity check.
    if old.children.len() == new.children.len() {
        score += 0.1
    //    } else if (old.children.len() as i16 - new.children.len() as i16).abs() == 1 {
    //        // Perhaps we've added or removed a child.
    //        score += 0.05  // TODO: non-even transaction
    } else {
        score -= 0.1
    }
    // Same id implies it may have been added in the same order.
    if old.id.expect("Missing id") == new.id.expect("Missing id") {
        score += 0.15
    } else {
        score -= 0.15
    };

    // For now, just look at the first child: Easier to code, and still helps.
    // Doing indefinite recursion of first child for now. Weight each child
    // subsequently-less.  This is effective for some common HTML patterns.
    //    for posit in 0..children_to_eval {
    //        if let Some(child_old) = &old.children.get(posit) {
    //            if let Some(child_new) = &old.children.get(posit) {
    //                score += child_score_significance * match_score(child_old, child_new);
    //            }
    //        }
    //    }

    score
}

pub trait Attrs: PartialEq + ToString {
    fn vals(self) -> HashMap<String, String>;
}

pub trait Style: PartialEq + ToString {
    fn vals(self) -> HashMap<String, String>;
}

pub trait Listener<Ms>: Sized {
    fn attach<T: AsRef<EventTarget>>(&mut self, el_ws: &T, mailbox: Mailbox<Ms>);
    fn detach<T: AsRef<EventTarget>>(&self, el_ws: &T);
}

/// WIP towards a modular VDOM
/// Assumes dependency on web_sys.
// TODO:: Do we need <Ms> ?
pub trait DomEl<Ms>: Sized + PartialEq + DomElLifecycle {
    type Tg: PartialEq + ToString; // TODO: tostring
    type At: Attrs;
    type St: Style;
    type Ls: Listener<Ms>;
    type Tx: PartialEq + ToString + Clone + Default;

    // Fields
    fn tag(self) -> Self::Tg;
    fn attrs(self) -> Self::At;
    fn style(self) -> Self::St;
    fn listeners(self) -> Vec<Self::Ls>;
    fn text(self) -> Option<Self::Tx>;
    fn children(self) -> Vec<Self>;
    fn websys_el(self) -> Option<web_sys::Element>;
    fn id(self) -> Option<u32>;
    fn raw_html(self) -> bool;
    // TODO: tying to dom_types is temp - defeats the urpose of the trait
    fn namespace(self) -> Option<Namespace>;

    // Methods
    fn empty(self) -> Self;

    // setters
    fn set_id(&mut self, id: Option<u32>);
    fn set_websys_el(&mut self, el: Option<Element>);

    //    fn make_websys_el(&self, document: &web_sys::Document) -> web_sys::Element;
}

pub trait DomElLifecycle {
    fn did_mount(self) -> Option<Box<FnMut(&Element)>>;
    fn did_update(self) -> Option<Box<FnMut(&Element)>>;
    fn will_unmount(self) -> Option<Box<FnMut(&Element)>>;
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use wasm_bindgen_test::*;

    use crate as seed; // required for macros to work.
    use crate::{div, li, prelude::*};

    #[derive(Clone)]
    enum Msg {}

    #[ignore]
    // #[wasm_bindgen_test]
    #[test]
    fn el_added() {
        let mut old_vdom: El<Msg> = div!["text", vec![li!["child1"],]];
        let mut new_vdom: El<Msg> = div!["text", vec![li!["child1"], li!["child2"]]];

        let doc = util::document();
        let old_ws = doc.create_element("div").unwrap();
        let new_ws = doc.create_element("div").unwrap();

        let child1 = doc.create_element("li").unwrap();
        let child2 = doc.create_element("li").unwrap();
        // TODO: make this match how you're setting text_content, eg could
        // TODO: be adding a text node.
        old_ws.set_text_content(Some("text"));
        child1.set_text_content(Some("child1"));
        child2.set_text_content(Some("child2"));

        old_ws.append_child(&child1).unwrap();
        new_ws.append_child(&child1).unwrap();
        new_ws.append_child(&child2).unwrap();

        let mailbox = Mailbox::new(|msg: Msg| {});

        let parent = doc.create_element("div").unwrap();
        patch(&doc, &mut old_vdom, &mut new_vdom, &parent, &mailbox);
        unimplemented!()
    }

    #[ignore]
    #[test]
    fn el_removed() {
        unimplemented!()
    }

    #[ignore]
    #[test]
    fn el_changed() {
        unimplemented!()
    }
}
