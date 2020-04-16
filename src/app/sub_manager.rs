use indexmap::IndexMap;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;

// ------ SubManager ------

type Subscriptions<Ms> = HashMap<TypeId, IndexMap<Uuid, Subscription<Ms>>>;

#[derive(Default)]
pub struct SubManager<Ms> {
    subs: Rc<RefCell<Subscriptions<Ms>>>,
}

impl<Ms: 'static> SubManager<Ms> {
    pub fn new() -> Self {
        Self {
            subs: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn subscribe<SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> Option<Ms> + Clone + 'static,
    ) {
        self.subscribe_with_priority(handler, 0);
    }

    pub(crate) fn subscribe_with_priority<SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> Option<Ms> + Clone + 'static,
        priority: i8,
    ) {
        let sub = Subscription::new_with_priority(handler, priority);
        let (type_id, id) = (sub.type_id, sub.id);

        let mut subs = self.subs.borrow_mut();
        subs.entry(type_id)
            .or_insert_with(IndexMap::new)
            .insert(id, sub);

        subs.entry(type_id).and_modify(|subs_group| {
            subs_group.sort_by(|_, sub_a, _, sub_b| Ord::cmp(&sub_b.priority, &sub_a.priority))
        });
    }

    pub fn subscribe_with_handle<SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> Option<Ms> + Clone + 'static,
    ) -> SubHandle {
        let sub = Subscription::new(handler);
        let (type_id, id) = (sub.type_id, sub.id);

        let mut subs = self.subs.borrow_mut();
        subs.entry(type_id)
            .or_insert_with(IndexMap::new)
            .insert(id, sub);

        subs.entry(type_id).and_modify(|subs_group| {
            subs_group.sort_by(|_, sub_a, _, sub_b| Ord::cmp(&sub_b.priority, &sub_a.priority))
        });

        let subs = Rc::clone(&self.subs);
        SubHandle {
            unsubscriber: Box::new(move || {
                subs.borrow_mut()
                    .get_mut(&type_id)
                    .expect("get subscriptions by `type_id`")
                    .remove(&id)
                    .expect("remove subscription");
            }),
        }
    }

    pub fn notify(&self, notification: &Notification) -> Vec<Ms> {
        self.subs
            .borrow()
            .get(&notification.type_id)
            .map(|subscriptions| {
                subscriptions
                    .values()
                    .filter_map(|subscription| (&subscription.handler)(&notification.message))
                    .collect()
            })
            .unwrap_or_default()
    }
}

// ------ SubHandle ------

pub struct SubHandle {
    unsubscriber: Box<dyn Fn()>,
}

impl fmt::Debug for SubHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SubHandle")
            .field("unsubscriber", &"Box<dyn Fn()>")
            .finish()
    }
}

impl Drop for SubHandle {
    fn drop(&mut self) {
        (self.unsubscriber)();
    }
}

// ------ Subscription ------

struct Subscription<Ms> {
    type_id: TypeId,
    id: Uuid,
    handler: Box<dyn Fn(&Box<dyn Any>) -> Option<Ms>>,
    priority: i8,
}

impl<Ms: 'static> Subscription<Ms> {
    #[allow(clippy::shadow_unrelated)]
    pub fn new<SubMs: 'static + Clone>(
        handler: impl FnOnce(SubMs) -> Option<Ms> + Clone + 'static,
    ) -> Self {
        Self::new_with_priority(handler, 0)
    }

    #[allow(clippy::shadow_unrelated)]
    pub(crate) fn new_with_priority<SubMs: 'static + Clone>(
        handler: impl FnOnce(SubMs) -> Option<Ms> + Clone + 'static,
        priority: i8,
    ) -> Self {
        // Convert `FnOnce + Clone` to `Fn`.
        let handler = move |sub_msg: SubMs| handler.clone()(sub_msg);

        // Convert `Fn(SubMs)` to `Fn(&Box<dyn Any>)` where `Any` is `SubMs`.
        let handler = move |sub_msg: &Box<dyn Any>| {
            let sub_msg = sub_msg
                .downcast_ref::<SubMs>()
                .expect("downcast to `SubMs`");
            handler(sub_msg.clone())
        };

        Self {
            type_id: TypeId::of::<SubMs>(),
            id: Uuid::new_v4(),
            handler: Box::new(handler),
            priority,
        }
    }
}

// ------ Notification ------

pub struct Notification {
    type_id: TypeId,
    message: Box<dyn Any>,
}

impl Notification {
    pub fn new<SubMs: 'static + Any + Clone>(message: SubMs) -> Self {
        Self {
            type_id: TypeId::of::<SubMs>(),
            message: Box::new(message),
        }
    }
}
