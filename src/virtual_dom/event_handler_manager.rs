use crate::app::MessageMapper;
use crate::virtual_dom::{Ev, Mailbox};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

pub mod event_handler;
pub mod listener;

pub use event_handler::EventHandler;
pub use listener::Listener;

// ------ EventHandlerManager ------

#[derive(Debug, Default)]
/// Manages event handlers and listeners for elements.
pub struct EventHandlerManager<Ms> {
    groups: BTreeMap<Ev, Group<Ms>>,
}

// @TODO remove custom impl once https://github.com/rust-lang/rust/issues/26925 is fixed
impl<Ms> Clone for EventHandlerManager<Ms> {
    fn clone(&self) -> Self {
        Self {
            groups: self.groups.clone(),
        }
    }
}

impl<Ms> EventHandlerManager<Ms> {
    /// Creates an empty manager instance.
    pub fn new() -> Self {
        Self {
            groups: BTreeMap::new(),
        }
    }

    /// Creates a new manager instance with given event handlers.
    /// It doesn't create listeners automatically - you have to call `attach_listeners`.
    pub fn with_event_handlers(event_handlers: Vec<EventHandler<Ms>>) -> Self {
        let mut manager = Self::new();
        manager.add_event_handlers(event_handlers);
        manager
    }

    /// Creates missing listeners and attaches them to the given `event_target`.
    /// It can reuse listeners from the `old_manager`.
    pub fn attach_listeners(
        &mut self,
        event_target: impl Into<web_sys::EventTarget> + 'static,
        mut old_manager: Option<&mut EventHandlerManager<Ms>>,
        mailbox: &Mailbox<Ms>,
    ) {
        let event_target = event_target.into();

        for (trigger, group) in &mut self.groups {
            if group.listener.is_none() {
                group.listener = old_manager
                    .as_mut()
                    .and_then(|old_manager| {
                        old_manager
                            .take_and_setup_listener(trigger, Rc::clone(&group.event_handlers))
                    })
                    .or_else(|| {
                        Some(Listener::new(
                            trigger.clone(),
                            event_target.clone(),
                            Rc::clone(&group.event_handlers),
                            mailbox.clone(),
                        ))
                    })
            }
        }
    }

    /// Add new event handlers into the manager.
    /// It doesn't create listeners automatically - you have to call `attach_listeners`.
    pub fn add_event_handlers(&mut self, event_handlers: Vec<EventHandler<Ms>>) {
        for handler in event_handlers {
            if let Some(group) = self.groups.get_mut(&handler.trigger) {
                group.event_handlers.borrow_mut().push(handler);
            } else {
                self.groups.insert(
                    handler.trigger.clone(),
                    Group {
                        event_handlers: Rc::new(RefCell::new(vec![handler])),
                        listener: None,
                    },
                );
            }
        }
    }

    /// This method is used in `attach_listeners` method to move listeners from the old manager.
    pub fn take_and_setup_listener(
        &mut self,
        trigger: &Ev,
        event_handlers: Rc<RefCell<Vec<EventHandler<Ms>>>>,
    ) -> Option<Listener<Ms>> {
        self.groups
            .get_mut(trigger)
            .and_then(|group| group.listener.take())
            .map(|listener| {
                listener.set_event_handlers(event_handlers);
                listener
            })
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for EventHandlerManager<Ms> {
    type SelfWithOtherMs = EventHandlerManager<OtherMs>;
    /// _Note:_ Listeners will be automatically detached and removed.
    /// You have to call `attach_listeners` to recreate them.
    fn map_msg(
        self,
        f: impl FnOnce(Ms) -> OtherMs + 'static + Clone,
    ) -> EventHandlerManager<OtherMs> {
        EventHandlerManager {
            groups: self
                .groups
                .into_iter()
                .map(|(trigger, group)| (trigger, group.map_msg(f.clone())))
                .collect(),
        }
    }
}

// ------ Group ------

#[derive(Debug)]
/// A group of event handlers and a listener with the same trigger (event).
struct Group<Ms> {
    // `event_handlers` are wrapped in `Rc` & `RefCell`
    // because they are sent to callback in `listener`.
    event_handlers: Rc<RefCell<Vec<EventHandler<Ms>>>>,
    // `listener` is optional because the element where the manager is placed may be pure virtual
    // - i.e. the element hasn't been associated with the DOM yet.
    listener: Option<Listener<Ms>>,
}

impl<T> Clone for Group<T> {
    /// _Note:_  The group's `listener` will be set to `None` and automatically detached.
    fn clone(&self) -> Self {
        Self {
            event_handlers: Rc::clone(&self.event_handlers),
            // We can't clone `listener` because it's tightly connected to the specific DOM element.
            listener: None,
        }
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Group<Ms> {
    type SelfWithOtherMs = Group<OtherMs>;
    /// _Note:_  The group's `listener` will be set to `None` and automatically detached.
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Group<OtherMs> {
        let mapped_event_handlers = self
            .event_handlers
            .replace(Vec::new())
            .into_iter()
            .map(|handler| handler.map_msg(f.clone()))
            .collect();

        Group {
            event_handlers: Rc::new(RefCell::new(mapped_event_handlers)),
            // `listener` has to be set to `None` because we had to create new `event handlers`.
            // (The reference to the old handlers in the `listener` has become invalid).
            listener: None,
        }
    }
}
