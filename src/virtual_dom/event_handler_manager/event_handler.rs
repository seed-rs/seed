use crate::app::MessageMapper;
use crate::virtual_dom::Ev;
use std::{fmt, rc::Rc};

#[derive(Clone)]
/// `EventHandler`s are called by DOM event listeners with the same trigger (an event to listen to).
pub struct EventHandler<Ms> {
    pub trigger: Ev,
    pub callback: Rc<dyn Fn(web_sys::Event) -> Ms>,
}

impl<Ms> EventHandler<Ms> {
    pub fn new(trigger: impl Into<Ev>, callback: impl Fn(web_sys::Event) -> Ms + 'static) -> Self {
        Self {
            trigger: trigger.into(),
            callback: Rc::new(callback),
        }
    }

    pub fn call(&self, event: web_sys::Event) -> Ms {
        (self.callback)(event)
    }
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for EventHandler<Ms> {
    type SelfWithOtherMs = EventHandler<OtherMs>;
    fn map_msg(
        self,
        msg_mapper: impl FnOnce(Ms) -> OtherMs + 'static + Clone,
    ) -> EventHandler<OtherMs> {
        let old_callback = self.callback;
        let new_callback = move |event| {
            let msg = old_callback(event);
            (msg_mapper.clone())(msg)
        };
        EventHandler {
            trigger: self.trigger,
            callback: Rc::new(new_callback),
        }
    }
}

impl<Ms> fmt::Debug for EventHandler<Ms> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventHandler('{}')", self.trigger.as_str())
    }
}
