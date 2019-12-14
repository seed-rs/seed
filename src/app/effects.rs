use super::MessageMapper;
use futures::Future;

pub enum Effect<Ms, GMs> {
    Msg(Ms),
    Cmd(Box<dyn Future<Item = Ms, Error = Ms> + 'static>),
    GMsg(GMs),
    GCmd(Box<dyn Future<Item = GMs, Error = GMs> + 'static>),
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
            Effect::Cmd(cmd) => Effect::Cmd(Box::new(cmd.map(f.clone()).map_err(f))),
            Effect::GMsg(g_msg) => Effect::GMsg(g_msg),
            Effect::GCmd(g_cmd) => Effect::GCmd(g_cmd),
        }
    }
}
