use crate::browser::util::ClosureNew;
use crate::virtual_dom::{Ev, EventHandler, Mailbox};
use std::{cell::RefCell, fmt, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};

/// Represents attached DOM event listener with the callback that calls event handlers.
pub struct Listener {
    // Event to listen to.
    trigger: Ev,
    // `callback` is invoked from the JS world and calls closed event handlers.
    callback: Closure<dyn FnMut(web_sys::Event)>,
    // Element where the listener is attached.
    event_target: web_sys::EventTarget,
}

impl Listener {
    /// Crate a new listener and attach it to the element.
    pub fn new<Ms>(
        trigger: Ev,
        event_target: web_sys::EventTarget,
        event_handlers: Rc<RefCell<Vec<EventHandler<Ms>>>>,
        mailbox: Mailbox<Ms>,
    ) -> Self {
        let callback = Closure::new(move |event: web_sys::Event| {
            for event_handler in event_handlers.borrow().iter() {
                let msg = event_handler.call(event.clone());
                mailbox.send(msg);
            }
        });

        event_target
            .add_event_listener_with_callback(trigger.as_str(), callback.as_ref().unchecked_ref())
            .expect("attach listener");

        Self {
            trigger,
            callback,
            event_target,
        }
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        self.event_target
            .remove_event_listener_with_callback(
                self.trigger.as_str(),
                self.callback.as_ref().unchecked_ref(),
            )
            .expect("detach listener");
    }
}

impl fmt::Debug for Listener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Listener('{}')", self.trigger.as_str())
    }
}
