use super::MessageMapper;
use futures::future::LocalFutureObj;

pub enum Effect<Ms, GMs> {
    Msg(Ms),
    Cmd(LocalFutureObj<'static, Result<Ms, Ms>>),
    GMsg(GMs),
    GCmd(LocalFutureObj<'static, Result<GMs, GMs>>),
}

impl<Ms, GMs> From<Ms> for Effect<Ms, GMs> {
    fn from(message: Ms) -> Self {
        Effect::Msg(message)
    }
}

impl<Ms: 'static, OtherMs: 'static, GMs> MessageMapper<Ms, OtherMs> for Effect<Ms, GMs> {
    type SelfWithOtherMs = Effect<OtherMs, GMs>;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static) -> Effect<OtherMs, GMs> {
        match self {
            Effect::Msg(msg) => Effect::Msg(f(msg)),
            Effect::Cmd(cmd) => Effect::Cmd(LocalFutureObj::new(Box::new(async {
                match cmd.await {
                    Ok(msg) => Ok(f(msg)),
                    Err(msg) => Err(f(msg)),
                }
            }))),
            Effect::GMsg(g_msg) => Effect::GMsg(g_msg),
            Effect::GCmd(g_cmd) => Effect::GCmd(g_cmd),
        }
    }
}
