use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;

// ------ SubManager ------

type Subscriptions<Ms> = HashMap<TypeId, HashMap<Uuid, Subscription<Ms>>>;

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
        let sub = Subscription::new(handler);
        let (type_id, id) = (sub.type_id, sub.id);

        self.subs
            .borrow_mut()
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(id, sub);
    }

    pub fn subscribe_with_handle<SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> Option<Ms> + Clone + 'static,
    ) -> SubHandle {
        let sub = Subscription::new(handler);
        let (type_id, id) = (sub.type_id, sub.id);

        self.subs
            .borrow_mut()
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(id, sub);

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
}

impl<Ms: 'static> Subscription<Ms> {
    #[allow(clippy::shadow_unrelated)]
    pub fn new<SubMs: 'static + Clone>(
        handler: impl FnOnce(SubMs) -> Option<Ms> + Clone + 'static,
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
