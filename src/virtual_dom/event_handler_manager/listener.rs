use crate::virtual_dom::{Ev, EventHandler, Mailbox};
use enclose::enc;
use std::{
    cell::{Cell, RefCell},
    fmt,
    rc::Rc,
};
use wasm_bindgen::{closure::Closure, JsCast};

// ------ Listener ------

/// Represents attached DOM event listener with the callback that calls event handlers.
pub struct Listener<Ms> {
    // Event to listen to.
    trigger: Ev,
    // "portal" to event handlers - it allows to call event handlers from the JS world.
    portal: Portal<Rc<RefCell<Vec<EventHandler<Ms>>>>>,
    // `callback` is invoked from the JS world and calls event handlers in the `portal`.
    callback: Closure<dyn FnMut(web_sys::Event)>,
    // Element where the listener is attached.
    event_target: web_sys::EventTarget,
}

impl<Ms> Listener<Ms> {
    /// Create a new listener and attach it to the element.
    pub fn new(
        trigger: Ev,
        event_target: web_sys::EventTarget,
        event_handlers: Rc<RefCell<Vec<EventHandler<Ms>>>>,
        mailbox: Mailbox<Ms>,
    ) -> Self {
        let portal_to_event_handlers = Portal::new(event_handlers);

        let callback = Closure::new(
            enc!((portal_to_event_handlers) move |event: web_sys::Event| {
                let mut handler_callbacks = Vec::new();
                portal_to_event_handlers.update(|event_handlers| {
                    // We need to clone handler callbacks and call them later
                    // because otherwise the app may crash while mutable borrowing event handlers.
                    // As a trade-off, all callbacks are called although their parents may not exist anymore.
                    for event_handler in event_handlers.borrow().iter() {
                        handler_callbacks.push(Rc::clone(&event_handler.callback));
                    }
                    event_handlers
                });
                for handler_callback in handler_callbacks {
                    let msg = handler_callback(event.clone());
                    mailbox.send(msg);
                }
            }),
        );

        event_target
            .add_event_listener_with_callback(trigger.as_str(), callback.as_ref().unchecked_ref())
            .expect("attach listener");

        Self {
            trigger,
            callback,
            event_target,
            portal: portal_to_event_handlers,
        }
    }

    pub fn set_event_handlers(&self, event_handlers: Rc<RefCell<Vec<EventHandler<Ms>>>>) {
        self.portal.update(|_| event_handlers);
    }
}

impl<Ms> Drop for Listener<Ms> {
    fn drop(&mut self) {
        self.event_target
            .remove_event_listener_with_callback(
                self.trigger.as_str(),
                self.callback.as_ref().unchecked_ref(),
            )
            .expect("detach listener");
    }
}

impl<Ms> fmt::Debug for Listener<Ms> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Listener('{}')", self.trigger.as_str())
    }
}

// ------ Portal ------

#[derive(Clone)]
/// "Portal" between the Rust world and the JS world.
struct Portal<T>(Rc<Cell<Option<T>>>);

impl<T> Portal<T> {
    pub fn new(shared_data: T) -> Self {
        Self(Rc::new(Cell::new(Some(shared_data))))
    }

    pub fn update(&self, f: impl FnOnce(T) -> T) {
        // @TODO replace with `Cell::update` once stable
        self.0.set(self.0.take().map(f))
    }
}
