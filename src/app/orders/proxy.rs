use super::{
    super::{App, CmdHandle, RenderTimestampDelta, StreamHandle, SubHandle, UndefinedGMsg},
    Orders, OrdersContainer,
};
use crate::virtual_dom::View;
use futures::future::{Future, FutureExt};
use futures::stream::{Stream, StreamExt};
use std::{any::Any, rc::Rc};

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

    fn notify(&mut self, message: impl Any + Clone) -> &mut Self {
        self.orders_container.notify(message);
        self
    }

    #[allow(clippy::redundant_closure)]
    fn send_msg(&mut self, msg: Ms) -> &mut Self {
        let f = self.f.clone();
        self.orders_container.send_msg(f(msg));
        self
    }

    #[allow(clippy::redundant_closure)]
    fn perform_cmd(&mut self, cmd: impl Future<Output = Ms> + 'static) -> &mut Self {
        let f = self.f.clone();
        self.orders_container.perform_cmd(cmd.map(move |ms| f(ms)));
        self
    }

    fn perform_cmd_with_handle(&mut self, cmd: impl Future<Output = Ms> + 'static) -> CmdHandle {
        let f = self.f.clone();
        self.orders_container
            .perform_cmd_with_handle(cmd.map(move |ms| f(ms)))
    }

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self {
        self.orders_container.send_g_msg(g_msg);
        self
    }

    fn perform_g_cmd(&mut self, g_cmd: impl Future<Output = GMs> + 'static) -> &mut Self {
        self.orders_container.perform_g_cmd(g_cmd);
        self
    }

    fn perform_g_cmd_with_handle(
        &mut self,
        g_cmd: impl Future<Output = GMs> + 'static,
    ) -> CmdHandle {
        self.orders_container.perform_g_cmd_with_handle(g_cmd)
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

    fn subscribe<SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> Ms + Clone + 'static,
    ) -> &mut Self {
        let f = self.f.clone();
        self.orders_container
            .subscribe(move |sub_ms| f(handler(sub_ms)));
        self
    }

    fn subscribe_with_handle<SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> Ms + Clone + 'static,
    ) -> SubHandle {
        let f = self.f.clone();
        self.orders_container
            .subscribe_with_handle(move |sub_ms| f(handler(sub_ms)))
    }

    fn stream(&mut self, stream: impl Stream<Item = Ms> + 'static) -> &mut Self {
        let f = self.f.clone();
        self.orders_container.stream(stream.map(move |ms| f(ms)));
        self
    }

    fn stream_with_handle(&mut self, stream: impl Stream<Item = Ms> + 'static) -> StreamHandle {
        let f = self.f.clone();
        self.orders_container
            .stream_with_handle(stream.map(move |ms| f(ms)))
    }
}
