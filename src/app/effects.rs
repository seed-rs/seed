use super::{MessageMapper, Notification};

pub enum Effect<Ms, GMs> {
    Msg(Ms),
    GMsg(GMs),
    Notification(Notification),
}

impl<Ms, GMs> From<Ms> for Effect<Ms, GMs> {
    fn from(message: Ms) -> Self {
        Effect::Msg(message)
    }
}

impl<Ms: 'static, OtherMs: 'static, GMs> MessageMapper<Ms, OtherMs> for Effect<Ms, GMs> {
    type SelfWithOtherMs = Effect<OtherMs, GMs>;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Effect<OtherMs, GMs> {
        match self {
            Effect::Msg(msg) => Effect::Msg(f(msg)),
            Effect::GMsg(g_msg) => Effect::GMsg(g_msg),
            Effect::Notification(notification) => Effect::Notification(notification),
        }
    }
}
