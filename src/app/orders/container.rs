use crate::app::orders::{proxy::OrdersProxy, Orders};
use crate::app::{
    effects::Effect, render_timestamp_delta::RenderTimestampDelta, App, ShouldRender, UndefinedGMsg,
};
use crate::virtual_dom::view::View;
use futures::future::LocalFutureObj;
use std::{collections::VecDeque, convert::identity, future::Future};

#[allow(clippy::module_name_repetitions)]
pub struct OrdersContainer<Ms: 'static, Mdl: 'static, ElC: View<Ms>, GMs = UndefinedGMsg> {
    pub(crate) should_render: ShouldRender,
    pub(crate) effects: VecDeque<Effect<Ms, GMs>>,
    app: App<Ms, Mdl, ElC, GMs>,
}

impl<Ms, Mdl, ElC: View<Ms>, GMs> OrdersContainer<Ms, Mdl, ElC, GMs> {
    pub fn new(app: App<Ms, Mdl, ElC, GMs>) -> Self {
        Self {
            should_render: ShouldRender::Render,
            effects: VecDeque::new(),
            app,
        }
    }

    pub(crate) fn merge(&mut self, mut other: Self) {
        self.should_render = other.should_render;
        self.effects.append(&mut other.effects);
    }
}

impl<Ms: 'static, Mdl, ElC: View<Ms> + 'static, GMs> Orders<Ms, GMs>
    for OrdersContainer<Ms, Mdl, ElC, GMs>
{
    type AppMs = Ms;
    type Mdl = Mdl;
    type ElC = ElC;

    #[allow(clippy::redundant_closure)]
    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl FnOnce(ChildMs) -> Ms + 'static + Clone,
    ) -> OrdersProxy<ChildMs, Ms, Mdl, ElC, GMs> {
        OrdersProxy::new(self, move |child_ms| f.clone()(child_ms))
    }

    fn render(&mut self) -> &mut Self {
        self.should_render = ShouldRender::Render;
        self
    }

    fn force_render_now(&mut self) -> &mut Self {
        self.should_render = ShouldRender::ForceRenderNow;
        self
    }

    fn skip(&mut self) -> &mut Self {
        self.should_render = ShouldRender::Skip;
        self
    }

    fn send_msg(&mut self, msg: Ms) -> &mut Self {
        self.effects.push_back(msg.into());
        self
    }

    fn perform_cmd<C>(&mut self, cmd: C) -> &mut Self
    where
        C: Future<Output = Ms> + 'static,
    {
        let effect = Effect::Cmd(LocalFutureObj::new(Box::new(cmd)));
        self.effects.push_back(effect);
        self
    }

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self {
        let effect = Effect::GMsg(g_msg);
        self.effects.push_back(effect);
        self
    }

    fn perform_g_cmd<C>(&mut self, g_cmd: C) -> &mut Self
    where
        C: Future<Output = GMs> + 'static,
    {
        let effect = Effect::GCmd(LocalFutureObj::new(Box::new(g_cmd)));
        self.effects.push_back(effect);
        self
    }

    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::ElC, GMs> {
        self.app.clone()
    }

    fn msg_mapper(&self) -> Box<dyn Fn(Ms) -> Self::AppMs> {
        Box::new(identity)
    }

    fn after_next_render(
        &mut self,
        callback: impl FnOnce(Option<RenderTimestampDelta>) -> Ms + 'static,
    ) -> &mut Self {
        self.app
            .data
            .after_next_render_callbacks
            .borrow_mut()
            .push(Box::new(callback));
        self
    }
}
