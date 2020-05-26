use super::{MessageMapper, Notification};

pub(crate) enum Effect<Ms> {
    Msg(Option<Ms>),
    Notification(Notification),
    TriggeredHandler(Box<dyn FnOnce() -> Option<Ms>>),
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Effect<Ms> {
    type SelfWithOtherMs = Effect<OtherMs>;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Effect<OtherMs> {
        match self {
            Effect::Msg(msg) => Effect::Msg(msg.map(f)),
            Effect::Notification(notification) => Effect::Notification(notification),
            Effect::TriggeredHandler(handler) => {
                Effect::TriggeredHandler(Box::new(move || handler().map(f)))
            }
        }
    }
}
