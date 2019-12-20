use super::Ev;
use crate::app::MessageMapper;
use crate::browser::{dom::lifecycle_hooks::fmt_hook_fn, util::ClosureNew};
use enclose::enclose;
use std::{fmt, mem};
use wasm_bindgen::{closure::Closure, JsCast};

type EventHandler<Ms> = Box<dyn FnMut(web_sys::Event) -> Ms>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Category {
    Custom,
    Input,
    Keyboard,
    Mouse,
    Pointer,
    Raw,
    Simple,
}

/// Ev-handling for Elements
pub struct Listener<Ms> {
    pub trigger: Ev,
    // Handler describes how to handle the event, and is used to generate the closure.
    pub handler: Option<EventHandler<Ms>>,
    // We store closure here so we can detach it later.
    pub closure: Option<Closure<dyn FnMut(web_sys::Event)>>,
    // Control listeners prevent input on controlled input elements, and
    // are not assoicated with a message.
    pub control_val: Option<String>,
    pub control_checked: Option<bool>,

    // category and message are used as an aid for comparing Listeners, and therefore diffing.
    // todo: Neither are fully implemented.
    category: Option<Category>,
    // An associated message, if applicable.
    message: Option<Ms>,
}

impl<Ms> fmt::Debug for Listener<Ms> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Listener {{ trigger:{:#?}, handler:{:#?}, closure:{:#?}, control:{:#?}{:#?}, category:{:#?}",
            self.trigger,
            fmt_hook_fn(&self.handler),
            fmt_hook_fn(&self.closure),
            self.control_val,
            self.control_checked,
            self.category,
        )
    }
}

impl<Ms> Listener<Ms> {
    pub fn new(
        trigger: &str,
        handler: Option<EventHandler<Ms>>,
        category: Option<Category>,
        message: Option<Ms>,
    ) -> Self {
        Self {
            // We use &str instead of Event here to allow flexibility in helper funcs,
            // without macros by using ToString.
            trigger: trigger.into(),
            handler,
            closure: None,
            control_val: None,
            control_checked: None,
            category,
            message,
        }
    }

    /// Set up a listener that keeps the field's value in sync with the specific value,
    /// from the model
    pub fn new_control(val: String) -> Self {
        Self {
            trigger: Ev::Input,
            handler: None,
            closure: None,
            control_val: Some(val),
            control_checked: None,
            category: None,
            message: None,
        }
    }

    /// Similar to `new_control`, but for checkboxes
    pub fn new_control_check(checked: bool) -> Self {
        Self {
            trigger: Ev::Click,
            handler: None,
            closure: None,
            control_val: None,
            control_checked: Some(checked),
            category: None,
            message: None,
        }
    }

    /// This method is where the processing logic for events happens.
    pub fn attach<T>(&mut self, el_ws: &T, mailbox: crate::virtual_dom::mailbox::Mailbox<Ms>)
    where
        T: AsRef<web_sys::EventTarget>,
    {
        let mut handler = self.handler.take().expect("Can't find old handler");
        // This is the closure ran when a DOM element has an user defined callback
        let closure = Closure::new(move |event: web_sys::Event| {
            let msg = handler(event);
            mailbox.send(msg);
        });

        (el_ws.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback(
                self.trigger.as_str(),
                closure.as_ref().unchecked_ref(),
            )
            .expect("Problem adding listener to element");

        // Store the closure so we can detach it later. Not detaching it when an element
        // is removed will trigger a panic.
        if self.closure.replace(closure).is_some() {
            panic!("self.closure already set in attach");
        }
    }

    pub fn detach<T>(&mut self, el_ws: &T)
    where
        T: AsRef<web_sys::EventTarget>,
    {
        let closure = self.closure.take().expect("Can't find closure to detach");

        (el_ws.as_ref() as &web_sys::EventTarget)
            .remove_event_listener_with_callback(
                self.trigger.as_str(),
                closure.as_ref().unchecked_ref(),
            )
            .expect("Problem removing listener from element");
    }
}

impl<Ms> PartialEq for Listener<Ms> {
    fn eq(&self, other: &Self) -> bool {
        // Todo: This isn't (yet) a comprehensive check, but can catch some differences.
        self.trigger == other.trigger
            && self.category == other.category
            // We use discriminant so we don't have to force Ms to impl PartialEq.
            && mem::discriminant(&self.message) == mem::discriminant(&other.message)
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Listener<Ms> {
    type SelfWithOtherMs = Listener<OtherMs>;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Listener<OtherMs> {
        Listener {
            trigger: self.trigger,
            handler: self.handler.map(enclose!((f) |mut eh| {
                Box::new(move |event| {
                    let m = (*eh)(event);
                    (f.clone())(m)
                }) as EventHandler<OtherMs>
            })),
            closure: self.closure,
            control_val: self.control_val,
            control_checked: self.control_checked,

            category: self.category,
            message: self.message.map(f),
        }
    }
}
