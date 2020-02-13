use super::{
    super::{App, Effect, MessageMapper, RenderTimestampDelta, UndefinedGMsg},
    Orders, OrdersContainer,
};
use crate::virtual_dom::View;
use futures::future::LocalFutureObj;
use std::future::Future;
use std::rc::Rc;

#[allow(clippy::module_name_repetitions)]
pub struct OrdersProxy<
    'a,
    Ms,
    AppMs: 'static,
    Mdl: 'static,
    ElC: View<AppMs>,
    GMs: 'static = UndefinedGMsg,
> {
    orders_container: &'a mut OrdersContainer<AppMs, Mdl, ElC, GMs>,
    f: Rc<dyn Fn(Ms) -> AppMs>,
}

impl<'a, Ms: 'static, AppMs: 'static, Mdl, ElC: View<AppMs>, GMs>
    OrdersProxy<'a, Ms, AppMs, Mdl, ElC, GMs>
{
    pub fn new(
        orders_container: &'a mut OrdersContainer<AppMs, Mdl, ElC, GMs>,
        f: impl Fn(Ms) -> AppMs + 'static,
    ) -> Self {
        OrdersProxy {
            orders_container,
            f: Rc::new(f),
        }
    }
}

impl<'a, Ms: 'static, AppMs: 'static, Mdl, ElC: View<AppMs> + 'static, GMs> Orders<Ms, GMs>
    for OrdersProxy<'a, Ms, AppMs, Mdl, ElC, GMs>
{
    type AppMs = AppMs;
    type Mdl = Mdl;
    type ElC = ElC;

    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl FnOnce(ChildMs) -> Ms + 'static + Clone,
    ) -> OrdersProxy<ChildMs, AppMs, Mdl, ElC, GMs> {
        let previous_f = self.f.clone();
        OrdersProxy {
            orders_container: self.orders_container,
            f: Rc::new(move |child_ms| previous_f(f.clone()(child_ms))),
        }
    }

    fn render(&mut self) -> &mut Self {
        self.orders_container.render();
        self
    }

    fn force_render_now(&mut self) -> &mut Self {
        self.orders_container.force_render_now();
        self
    }

    fn skip(&mut self) -> &mut Self {
        self.orders_container.skip();
        self
    }

    #[allow(clippy::redundant_closure)]
    fn send_msg(&mut self, msg: Ms) -> &mut Self {
        let f = self.f.clone();
        self.orders_container
            .effects
            .push_back(Effect::Msg(msg).map_msg(move |ms| f(ms)));
        self
    }

    #[allow(clippy::redundant_closure)]
    fn perform_cmd<C>(&mut self, cmd: C) -> &mut Self
    where
        C: Future<Output = Ms> + 'static,
    {
        let f = self.f.clone();
        let effect = Effect::Cmd(LocalFutureObj::new(Box::new(cmd))).map_msg(move |ms| f(ms));
        self.orders_container.effects.push_back(effect);
        self
    }

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self {
        let effect = Effect::GMsg(g_msg);
        self.orders_container.effects.push_back(effect);
        self
    }

    fn perform_g_cmd<C>(&mut self, g_cmd: C) -> &mut Self
    where
        C: Future<Output = GMs> + 'static,
    {
        let effect = Effect::GCmd(LocalFutureObj::new(Box::new(g_cmd)));
        self.orders_container.effects.push_back(effect);
        self
    }

    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::ElC, GMs> {
        self.orders_container.clone_app()
    }

    #[allow(clippy::redundant_closure)]
    fn msg_mapper(&self) -> Box<dyn Fn(Ms) -> Self::AppMs> {
        let f = self.f.clone();
        Box::new(move |ms| f(ms))
    }

    fn after_next_render(
        &mut self,
        callback: impl FnOnce(Option<RenderTimestampDelta>) -> Ms + 'static,
    ) -> &mut Self {
        let f = self.f.clone();
        self.orders_container
            .after_next_render(move |timestamp_delta| f(callback(timestamp_delta)));
        self
    }
}
