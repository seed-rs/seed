use crate::{
    dom_types,
    dom_types::{El, Namespace},
    routing, util, websys_bridge,
};
use std::{cell::RefCell, collections::HashMap, panic, rc::Rc};
use wasm_bindgen::closure::Closure;
use web_sys::{Document, Element, Event, EventTarget, Window};

pub enum Update<Ms, Mdl> {
    Render(Mdl),
    Skip(Mdl),
    RenderThen(Mdl, Ms)
}

impl<Ms, Mdl> Update<Ms, Mdl> {
    pub fn model(self) -> Mdl {
        use Update::*;
        match self {
            Render(model) => model,
            Skip(model) => model,
            RenderThen(model, _) => model,
        }
    }
}
// todo should this go here? do we need it?


type UpdateFn<Ms, Mdl> = fn(Ms, Mdl) -> Update<Ms, Mdl>;
type ViewFn<Ms, Mdl> = fn(App<Ms, Mdl>, &Mdl) -> El<Ms>;
type RoutesFn<Ms> = fn(&crate::routing::Url) -> Ms;
type WindowEvents<Ms, Mdl> = fn(&Mdl) -> Vec<dom_types::Listener<Ms>>;
type MsgListeners<Ms> = Vec<Box<Fn(&Ms)>>;

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
pub struct AppData<Ms: Clone + 'static, Mdl> {
    // Model is in a RefCell<Option> here so we can replace it in self.update().
    pub model: RefCell<Option<Mdl>>,
    main_el_vdom: RefCell<Option<El<Ms>>>,
    pub popstate_closure: StoredPopstate,
    pub routes: RefCell<Option<RoutesFn<Ms>>>,
    window_listeners: RefCell<Vec<dom_types::Listener<Ms>>>,
    msg_listeners: RefCell<MsgListeners<Ms>>,
}

pub struct AppCfg<Ms: Clone + 'static, Mdl: 'static> {
    document: web_sys::Document,
    mount_point: web_sys::Element,
    pub update: UpdateFn<Ms, Mdl>,
    view: ViewFn<Ms, Mdl>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
}

pub struct App<Ms: Clone + 'static, Mdl: 'static> {
    /// Stateless app configuration
    pub cfg: Rc<AppCfg<Ms, Mdl>>,
    /// Mutable app state
    pub data: Rc<AppData<Ms, Mdl>>,
}

impl<Ms: Clone + 'static, Mdl: 'static> ::std::fmt::Debug for App<Ms, Mdl> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "App")
    }
}

#[derive(Clone)]
pub struct AppBuilder<Ms: Clone + 'static, Mdl: 'static> {
    model: Mdl,
    update: UpdateFn<Ms, Mdl>,
    view: ViewFn<Ms, Mdl>,
    parent_div_id: Option<&'static str>,
    routes: Option<RoutesFn<Ms>>,
    window_events: Option<WindowEvents<Ms, Mdl>>,
}

impl<Ms: Clone, Mdl> AppBuilder<Ms, Mdl> {
    pub fn mount(mut self, id: &'static str) -> Self {
        self.parent_div_id = Some(id);
        self
    }

    pub fn routes(mut self, routes: RoutesFn<Ms>) -> Self {
        self.routes = Some(routes);
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
/// repetitive sequences of parameters.
impl<Ms: Clone, Mdl> App<Ms, Mdl> {
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
            routes: None,
            window_events: None,
        }
    }

    fn new(
        model: Mdl,
        update: UpdateFn<Ms, Mdl>,
        view: ViewFn<Ms, Mdl>,
        parent_div_id: &str,
        routes: Option<RoutesFn<Ms>>,
        window_events: Option<WindowEvents<Ms, Mdl>>,
    ) -> Self {
        let window = util::window();
        let document = window
            .document()
            .expect("Can't find the window's document");

        // We log an error instead of relying on panic/except due to the panic hook not yet
        // being active.
        let mount_point;
        match document
            .get_element_by_id(parent_div_id) {
            Some(mp) => mount_point = mp,
            None => {
                let text = "Can't finding parent div id (defaults to \"app\", or can be set with the .mount() method)";
                crate::error(text);
                panic!(text);
            },
        }

        Self {
            cfg: Rc::new(AppCfg {
                document,
                mount_point,
                update,
                view,
                window_events,
            }),
            data: Rc::new(AppData {
                model: RefCell::new(Some(model)),
                // This is filled for the first time in run()
                main_el_vdom: RefCell::new(None),
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

        let mut topel_vdom = {
            let model = self.data.model.borrow();
            let model = model.as_ref().expect("missing model");
            (self.cfg.view)(self.clone(), model)
        };

        // TODO: use window events
        if self.cfg.window_events.is_some() {
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

        let document = window.document().expect("Problem getting document");
        setup_els(&document, &mut topel_vdom);

        attach_listeners(&mut topel_vdom, &self.mailbox());

        // Attach all children: This is where our initial render occurs.
        websys_bridge::attach_el_and_children(&mut topel_vdom, &self.cfg.mount_point);

        self.data.main_el_vdom.replace(Some(topel_vdom));

        // Update the state on page load, based
        // on the starting URL. Must be set up on the server as well.
        if let Some(routes) = self.data.routes.borrow().clone() {  // ignore clippy re clone() on copy
            routing::setup_popstate_listener(
                &routing::initial(self.clone(), routes),
                routes
            );
            routing::setup_link_listener(&self, routes);

        }

        // Allows panic messages to output to the browser console.error.
        panic::set_hook(Box::new(console_error_panic_hook::hook));



        self
    }

    /// Do the actual self.cfg.update call. Updates self.data.model and returns (should_render, effect_msg)
    fn call_update(&self, message: Ms) -> (bool, Option<Ms>) {
        // data.model is the old model; Remove model from self.data.model, then pass it to the
        // update function created in the app, which outputs an updated model.
        let model = self.data.model.borrow_mut().take().expect("missing model");
        let updated_model_wrapped = (self.cfg.update)(message, model);

        let mut should_render = true;
        let mut effect_msg = None;
        let model = match updated_model_wrapped {
            Update::Render(mdl) => mdl,
            Update::Skip(mdl) => {
                should_render = false;
                mdl
            },
            Update::RenderThen(mdl, msg) => {
                effect_msg = Some(msg);
                mdl
            }
        };

        // Store updated model back to self.data.model
        self.data.model.borrow_mut().replace(model);

        (should_render, effect_msg)
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
    /// The model stored in inner is the old model; updated_model is a newly-calculated one.
    pub fn update(&self, message: Ms) {
        for l in self.data.msg_listeners.borrow().iter() {
            (l)(&message)
        }

        let (should_render, effect_msg) = self.call_update(message);

        let model = self.data.model.borrow();
        let model = model.as_ref().expect("missing model");

        if let Some(window_events) = self.cfg.window_events {
            let mut new_listeners = (window_events)(model);
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
            // have associated web_sys elements.
            let mut topel_new_vdom = (self.cfg.view)(self.clone(), model);

            // We setup the vdom (which populates web_sys els through it, but don't
            // render them with attach_children; we try to do it cleverly via patch().
            setup_els(&self.cfg.document, &mut topel_new_vdom);

            let mut old_vdom = self.data.main_el_vdom.borrow_mut().take().expect("missing main_el_vdom");

            // Detach all old listeners before patching. We'll re-add them as required during patching.
            // We'll get a runtime panic if any are left un-removed.
            detach_listeners(&mut old_vdom);

            patch(
                &self.cfg.document,
                old_vdom,
                &mut topel_new_vdom,
                &self.cfg.mount_point,
                None,
                &self.mailbox(),
            );

            // Now that we've re-rendered, replace our stored El with the new one;
            // it will be used as the old El next time.
            self.data.main_el_vdom.borrow_mut().replace(topel_new_vdom);
        }

        if let Some(msg) = effect_msg {
            self.update(msg)
        }
    }

    pub fn add_message_listener<F>(&self, listener: F)
    where
        F: Fn(&Ms) + 'static,
    {
        self.data
            .msg_listeners
            .borrow_mut()
            .push(Box::new(listener));
    }

    fn mailbox(&self) -> Mailbox<Ms> {
        let cloned = self.clone();
        Mailbox::new(move |message| {
            cloned.update(message);
        })
    }
}

/// Set up controlled components: Input, Select, and TextArea elements must stay in sync with the
/// model; don't let them get out of sync from typing or other events, which can occur if a change
/// doesn't trigger a re-render, or if something else modifies them using a side effect.
/// Handle controlled inputs: Ie force sync with the model.
fn setup_input_listener<Ms>(el: &mut El<Ms>)
where
    Ms: Clone + 'static,
{
    if el.tag == dom_types::Tag::Input || el.tag == dom_types::Tag::Select || el.tag == dom_types::Tag::TextArea {
        let listener = if let Some(checked) = el.attrs.vals.get(&dom_types::At::Checked) {
            let checked_bool = match checked.as_ref() {
                "true" => true,
                "false" => false,
                _ => panic!("checked must be true or false.")
            };
            dom_types::Listener::new_control_check(checked_bool)
        } else if let Some(control_val) = el.attrs.vals.get(&dom_types::At::Value) {
            dom_types::Listener::new_control(control_val.to_string())
        } else {
            // If Value is not specified, force the field to be blank.
            dom_types::Listener::new_control("".to_string())
        };
        el.listeners.push(listener);  // Add to the El, so we can deattach later.
    }
}

// Create the web_sys element; add it to the working tree; store it in its corresponding vdom El.
fn setup_websys_el<Ms>(document: &Document, el: &mut El<Ms>)
where
    Ms: Clone + 'static,
{
    el.el_ws = Some(websys_bridge::make_websys_el(el, document));
}

/// Populate the attached web_sys elements. Run this after creating a vdom, but before using it to
/// process the web_sys dom. Does not attach children in the DOM. Run this on the top-level
/// element.
fn setup_els<Ms>(document: &Document, el_vdom: &mut El<Ms>)
where
    Ms: Clone + 'static,
{
    el_vdom.walk_tree_mut(|el| {
        setup_input_listener(el);
        setup_websys_el(document, el);
    });
}

impl<Ms: Clone, Mdl> Clone for App<Ms, Mdl> {
    fn clone(&self) -> Self {
        App {
            cfg: Rc::clone(&self.cfg),
            data: Rc::clone(&self.data),
        }
    }
}

/// Recursively attach all event-listeners. Run this after creating fresh elements.
fn attach_listeners<Ms: Clone>(el: &mut dom_types::El<Ms>, mailbox: &Mailbox<Ms>) {
    el.walk_tree_mut(|el| {
        if let Some(el_ws) = el.el_ws.as_ref() {
            for listener in &mut el.listeners {
                // todo ideally we unify attach as one method
                if listener.control_val.is_some() || listener.control_checked.is_some() {
                    listener.attach_control(&el_ws);
                } else {
                    listener.attach(el_ws, mailbox.clone());
                }
            }
        }
    });
}

/// Recursively detach event-listeners. Run this before patching.
fn detach_listeners<Ms: Clone>(el: &mut dom_types::El<Ms>) {
    el.walk_tree_mut(|el| {
        if let Some(el_ws) = el.el_ws.as_ref() {
            for listener in &mut el.listeners {
                listener.detach(el_ws);
            }
        }
    });
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

fn patch<'a, Ms: Clone>(
    document: &Document,
    mut old: El<Ms>,
    new: &'a mut El<Ms>,
    parent: &web_sys::Node,
    next_node: Option<web_sys::Node>,
    mailbox: &Mailbox<Ms>,
) -> Option<&'a web_sys::Node> {
    // Old_el_ws is what we're patching, with items from the new vDOM el; or replacing.
    // TODO: Current sceme is that if the parent changes, redraw all children...
    // TODO: fix this later.
    // We make an assumption that most of the page is not dramatically changed
    // by each event, to optimize.

    // Assume setup_vdom has been run on the new el, all listeners have been removed
    // from the old el_ws, and the only the old el vdom's elements are still attached.

    // take removes the interior value from the Option; otherwise we run into problems
    // about not being able to remove from borrowed content.
    // We remove it from the old el_vodom now, and at the end... add it to the new one.
    // We don't run attach_children() when patching, hence this approach.

    let old_el_ws = old.el_ws.take()?;

    if old != *new {

        // At this step, we already assume we have the right element - either
        // by entering this func directly for the top-level, or recursively after
        // analyzing children

        // If the tag's different, we must redraw the element and its children; there's
        // no way to patch one element type into another.
        // TODO: forcing a rerender for differnet listeners is inefficient
        // TODO:, but I'm not sure how to patch them.
        if new.empty && !old.empty {
            parent.remove_child(&old_el_ws)
                .expect("Problem removing old we_el when updating to empty");
            if let Some(unmount_actions) = &mut old.hooks.will_unmount {
                unmount_actions(&old_el_ws)
            }
            return None;
            // If new and old are empty, we don't need to do anything.
        } else if new.empty && old.empty {
            return None;
        }
            // Namespaces can't be patched, since they involve create_element_ns instead of create_element.
            // Something about this element itself is different: patch it.
//            else if old.tag != new.tag || old.namespace != new.namespace || old.empty != new.empty {
            else if old.tag != new.tag || old.namespace != new.namespace {
                // TODO: DRY here between this and later in func.
                if let Some(unmount_actions) = &mut old.hooks.will_unmount {
                    unmount_actions(&old_el_ws)
                }

                // todo: Perhaps some of this next segment should be moved to websys_bridge
                websys_bridge::attach_children(new);

                let new_el_ws = new.el_ws.as_ref().expect("Missing websys el");

                if old.empty {
                    parent.insert_before(new_el_ws, next_node.as_ref())
                        .expect("Problem adding element to replace previously empty one");
                } else {
                    parent
                        .replace_child(new_el_ws, &old_el_ws)
                        .expect("Problem replacing element");
                }

                // Perform side-effects specified for mounting.
                if let Some(mount_actions) = &mut new.hooks.did_mount {
                    mount_actions(new_el_ws);
                }

                let mut new = new;
                attach_listeners(&mut new, &mailbox);
                // We've re-rendered this child and all children; we're done with this recursion.
                return new.el_ws.as_ref();
            }
        // The fourth empty case, where old is empty and new isn't, is handled when iterating through children.

        // Patch parts of the Element.
        websys_bridge::patch_el_details(&mut old, new, &old_el_ws);
    }

    // Before running patch, assume we've removed all listeners from the old element.
    // Perform this attachment after we've verified we can patch this element, ie
    // it has the same tag - otherwise  we'd have to detach after the parent.remove_child step.
    // Note that unlike the attach_listeners function, this only attaches for the current
    // element.
    for listener in &mut new.listeners {
        if listener.control_val.is_some() || listener.control_checked.is_some() {
            listener.attach_control(&old_el_ws);
        } else {
            listener.attach(&old_el_ws, mailbox.clone());
        }
    }

    let num_children_in_both = old.children.len().min(new.children.len());
    let mut old_children_iter = old.children.into_iter();
    let mut new_children_iter = new.children.iter_mut();

    let mut last_visited_node: Option<web_sys::Node> = None;

    // Not using .zip() here to make sure we don't miss any of the children when one array is
    // longer than the other.
    for _i in 0..num_children_in_both {
        let child_old = old_children_iter.next().unwrap();
        let child_new = new_children_iter.next().unwrap();

        // If a key's specified, use it to match the child
        // There can be multiple optomizations, but assume one key. If there are multiple
        // keys, use the first (There should only be one, but no constraints atm).
//        if let Some(key) = child_new.key() {
//            let _matching = old.children.iter().filter(|c| c.key() == Some(key));
//            // todo continue implementation: Patch and re-order.
//        }

        //                match old.children.get(i_new) {
//            Some(child_old) => {
//                // todo: This approach is still inefficient use of key, since it overwrites
//                // todo non-matching keys, preventing them from being found later.
//                if let Some(key) = child_new.key() {
//                    if child_old.key() == Some(key) {
//                        continue
//                    }
//                }
//
//                // Don't compare equality here; we do that at the top of this function
//                // in the recursion.
//                patch(document, &mut child_old.clone(), child_new, &old_el_ws, &mailbox);
//                old_children_patched.push(child_old.id.expect("Can't find child's id"));
//            },
//            None => {
//                // We ran out of old children to patch; create new ones.
//                websys_bridge::attach_el_and_children(child_new, &old_el_ws);
//                let mut child_new = child_new;
//                attach_listeners(&mut child_new, &mailbox);
//            }
//        }

        // Don't compare equality here; we do that at the top of this function
        // in the recursion.
        if let Some(new_el_ws) = patch(
            document,
            child_old,
            child_new,
            &old_el_ws,
            match last_visited_node.as_ref() {
                Some(node) => node.next_sibling(),
                None => old_el_ws.first_child(),
            },
            &mailbox)
        {
            last_visited_node = Some(new_el_ws.clone());
        }
    }

    // Now one of the iterators is entirely consumed, and any items left in one iterator
    // don't have any matching items in the other.

    while let Some(child_new) = new_children_iter.next() {


        // We ran out of old children to patch; create new ones.
        websys_bridge::attach_el_and_children(child_new, &old_el_ws);
        attach_listeners(child_new, &mailbox);
    }

//    // Now pair up children as best we can.
//    // If there are the same number of children, assume there's a 1-to-1 mapping,
//    // where we will not add or remove any; but patch as needed.
//    let avail_old_children = &mut old.children;
//    let mut prev_child: Option<web_sys::Node> = None;
//    let mut best_match;
////    let mut t;
//    for (i_new, child_new) in new.children.iter_mut().enumerate() {
//        if avail_old_children.is_empty() {
//            // One or more new children has been added, or much content has
//            // changed, or we've made a mistake: Attach new children.
//            websys_bridge::attach_els(child_new, &old_el_ws);
//            let mut child_new = child_new;
//            attach_listeners(&mut child_new, &mailbox);
//
//        } else {
//            // We still have old children to pick a match from. If we pick
//            // incorrectly, or there is no "good" match, we'll have some
//            // patching and/or attaching (rendering) to do in subsequent recursions.
//            let mut scores: Vec<(u32, f32)> = avail_old_children
//                .iter()
//                .enumerate()
//                .map(|(i_old, c_old)| (c_old.id.unwrap(), match_score(c_old, i_old, child_new, i_new)))
//                .collect();
//
//            // should put highest score at the end.
//            scores.sort_by(|b, a| b.1.partial_cmp(&a.1).unwrap());
//
//            // Sorting children vice picking the best one makes this easier to handle
//            // without irking the borrow checker, despite appearing less counter-intuitive,
//            // due to the convenient pop method.
//            avail_old_children.sort_by(|b, a| {
//                scores
//                    .iter()
//                    .find(|s| s.0 == b.id.unwrap())
//                    .unwrap()
//                    .1
//                    .partial_cmp(&scores.iter().find(|s| s.0 == a.id.unwrap()).unwrap().1)
//                    .unwrap()
//            });
//
//            best_match = avail_old_children.pop().expect("Problem popping");


    // Now purge any existing no-longer-needed children; they're not part of the new vdom.
    while let Some(mut child) = old_children_iter.next() {
        let child_el_ws = child.el_ws.take().expect("Missing child el_ws");

        // TODO: DRY here between this and earlier in func
        if let Some(unmount_actions) = &mut child.hooks.will_unmount {
            unmount_actions(&child_el_ws)
        }

        // todo get to the bottom of this
        match old_el_ws.remove_child(&child_el_ws) {
            Ok(_) => {},
            Err(_) => {crate::log("Minor error patching html element. (remove)");}
        }
    }

    new.el_ws = Some(old_el_ws);
    new.el_ws.as_ref()
}

/// Update app state directly, ie not from a Listener/event.
//pub fn update<Ms>(message: Ms {  // todo deal with this.
//    let mailbox = Mailbox::new(move |msg| {
//        app.update(msg);
//    });
//    mailbox.send(message);
//}


pub trait _Attrs: PartialEq + ToString {
    fn vals(self) -> HashMap<String, String>;
}

pub trait _Style: PartialEq + ToString {
    fn vals(self) -> HashMap<String, String>;
}

pub trait _Listener<Ms>: Sized {
    fn attach<T: AsRef<EventTarget>>(&mut self, el_ws: &T, mailbox: Mailbox<Ms>);
    fn detach<T: AsRef<EventTarget>>(&self, el_ws: &T);
}

/// WIP towards a modular VDOM
/// Assumes dependency on web_sys.
// TODO:: Do we need <Ms> ?
pub trait _DomEl<Ms>: Sized + PartialEq + DomElLifecycle {
    type Tg: PartialEq + ToString; // TODO: tostring
    type At: _Attrs;
    type St: _Style;
    type Ls: _Listener<Ms>;
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
    // TODO: tying to dom_types is temp - defeats the urpose of the trait
    fn namespace(self) -> Option<Namespace>;

    // Methods
    fn empty(self) -> Self;

    // setters
    fn set_id(&mut self, id: Option<u32>);
    fn set_websys_el(&mut self, el: Option<Element>);
}

pub trait DomElLifecycle {
    fn did_mount(self) -> Option<Box<FnMut(&Element)>>;
    fn did_update(self) -> Option<Box<FnMut(&Element)>>;
    fn will_unmount(self) -> Option<Box<FnMut(&Element)>>;
}

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;

    use crate as seed; // required for macros to work.
    use crate::{class, prelude::*};
    use wasm_bindgen::JsCast;
    use web_sys::Node;

    #[derive(Clone, Debug)]
    enum Msg {}

    fn make_vdom(doc: &Document, el: El<Msg>) -> El<Msg> {
        let mut vdom = el;
        setup_els(doc, &mut vdom);
        vdom
    }

    fn setup_and_patch(
        doc: &Document,
        parent: &Element,
        mailbox: &Mailbox<Msg>,
        old_vdom: El<Msg>,
        new_vdom: El<Msg>,
    ) -> El<Msg> {
        let mut new_vdom = make_vdom(&doc, new_vdom);
        patch(&doc, old_vdom, &mut new_vdom, parent, None, mailbox);
        new_vdom
    }

    fn iter_nodelist(list: web_sys::NodeList) -> impl Iterator<Item = Node> {
        (0..list.length()).map(move |i| list.item(i).unwrap())
    }

    fn iter_child_nodes(node: &Node) -> impl Iterator<Item = Node> {
        iter_nodelist(node.child_nodes())
    }

    #[wasm_bindgen_test]
    fn el_added() {
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = make_vdom(&doc, El::empty(seed::dom_types::Tag::Div));
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
        parent.append_child(&old_ws).unwrap();

        assert_eq!(parent.children().length(), 1);
        assert_eq!(old_ws.child_nodes().length(), 0);

        vdom = setup_and_patch(&doc, &parent, &mailbox, vdom, div!["text"]);
        assert_eq!(parent.children().length(), 1);
        assert!(old_ws.is_same_node(parent.first_child().as_ref()));
        assert_eq!(old_ws.child_nodes().length(), 1);
        assert_eq!(old_ws.first_child().unwrap().text_content().unwrap(), "text");

        setup_and_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div!["text", "more text", vec![li!["even more text"]]],
        );

        assert_eq!(parent.children().length(), 1);
        assert!(old_ws.is_same_node(parent.first_child().as_ref()));
        assert_eq!(old_ws.child_nodes().length(), 3);
        assert_eq!(old_ws.child_nodes().item(0).unwrap().text_content().unwrap(), "text");
        assert_eq!(old_ws.child_nodes().item(1).unwrap().text_content().unwrap(), "more text");
        let child3 = old_ws.child_nodes().item(2).unwrap();
        assert_eq!(child3.node_name(), "LI");
        assert_eq!(child3.text_content().unwrap(), "even more text");
    }

    #[wasm_bindgen_test]
    fn el_removed() {
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = make_vdom(&doc, El::empty(seed::dom_types::Tag::Div));
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
        parent.append_child(&old_ws).unwrap();

        // First add some child nodes using the vdom
        vdom = setup_and_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div!["text", "more text", vec![li!["even more text"]]],
        );

        assert_eq!(parent.children().length(), 1);
        assert_eq!(old_ws.child_nodes().length(), 3);
        let old_child1 = old_ws.child_nodes().item(0).unwrap();

        // Now test that patch function removes the last 2 nodes
        setup_and_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div!["text"],
        );

        assert_eq!(parent.children().length(), 1);
        assert!(old_ws.is_same_node(parent.first_child().as_ref()));
        assert_eq!(old_ws.child_nodes().length(), 1);
        assert!(old_child1.is_same_node(old_ws.child_nodes().item(0).as_ref()));
    }

    #[wasm_bindgen_test]
    fn el_changed() {
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = make_vdom(&doc, El::empty(seed::dom_types::Tag::Div));
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
        parent.append_child(&old_ws).unwrap();

        // First add some child nodes using the vdom
        vdom = setup_and_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div![span!["hello"], ", ", span!["world"]],
        );

        assert_eq!(parent.child_nodes().length(), 1);
        assert_eq!(old_ws.child_nodes().length(), 3);

        // Now add some attributes
        setup_and_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div![
                span![class!["first"], "hello"],
                ", ",
                span![class!["second"], "world"],
            ],
        );

        let child1 = old_ws.child_nodes().item(0).unwrap().dyn_into::<Element>().unwrap();
        assert_eq!(child1.get_attribute("class"), Some("first".to_string()));
        let child3 = old_ws.child_nodes().item(2).unwrap().dyn_into::<Element>().unwrap();
        assert_eq!(child3.get_attribute("class"), Some("second".to_string()));
    }

    /// Test that if the first child was a seed::empty() and it is changed to a non-empty El,
    /// then the new element is inserted at the correct position.
    #[wasm_bindgen_test]
    fn empty_changed_in_front() {
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = make_vdom(&doc, El::empty(seed::dom_types::Tag::Div));
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
        parent.append_child(&old_ws).unwrap();

        assert_eq!(parent.children().length(), 1);
        assert_eq!(old_ws.child_nodes().length(), 0);

        vdom = setup_and_patch(&doc, &parent, &mailbox, vdom, div![seed::empty(), "b", "c"]);
        assert_eq!(parent.children().length(), 1);
        assert!(old_ws.is_same_node(parent.first_child().as_ref()));
        assert_eq!(
            iter_child_nodes(&old_ws).map(|node| node.text_content().unwrap()).collect::<Vec<_>>(),
            &["b", "c"],
        );

        setup_and_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div!["a", "b", "c"],
        );

        assert_eq!(parent.children().length(), 1);
        assert!(old_ws.is_same_node(parent.first_child().as_ref()));
        assert_eq!(
            iter_child_nodes(&old_ws).map(|node| node.text_content().unwrap()).collect::<Vec<_>>(),
            &["a", "b", "c"],
        );
    }

    /// Test that if a middle child was a seed::empty() and it is changed to a non-empty El,
    /// then the new element is inserted at the correct position.
    #[wasm_bindgen_test]
    fn empty_changed_in_the_middle() {
        let mailbox = Mailbox::new(|_msg: Msg| {});

        let doc = util::document();
        let parent = doc.create_element("div").unwrap();

        let mut vdom = make_vdom(&doc, El::empty(seed::dom_types::Tag::Div));
        // clone so we can keep using it after vdom is modified
        let old_ws = vdom.el_ws.as_ref().unwrap().clone();
        parent.append_child(&old_ws).unwrap();

        assert_eq!(parent.children().length(), 1);
        assert_eq!(old_ws.child_nodes().length(), 0);

        vdom = setup_and_patch(&doc, &parent, &mailbox, vdom, div!["a", seed::empty(), "c"]);
        assert_eq!(parent.children().length(), 1);
        assert!(old_ws.is_same_node(parent.first_child().as_ref()));
        assert_eq!(
            iter_child_nodes(&old_ws).map(|node| node.text_content().unwrap()).collect::<Vec<_>>(),
            &["a", "c"],
        );

        setup_and_patch(
            &doc,
            &parent,
            &mailbox,
            vdom,
            div!["a", "b", "c"],
        );

        assert_eq!(parent.children().length(), 1);
        assert!(old_ws.is_same_node(parent.first_child().as_ref()));
        assert_eq!(
            iter_child_nodes(&old_ws).map(|node| node.text_content().unwrap()).collect::<Vec<_>>(),
            &["a", "b", "c"],
        );
    }
}
