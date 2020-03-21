use super::{
    super::{App, CmdHandle, RenderTimestampDelta, StreamHandle, SubHandle, UndefinedGMsg},
    Orders, OrdersContainer,
};
use crate::virtual_dom::View;
use futures::future::{Future, FutureExt};
use futures::stream::{Stream, StreamExt};
use std::{
    any::{Any, TypeId},
    rc::Rc,
};

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

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn subscribe<MsU: 'static, SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> MsU + Clone + 'static,
    ) -> &mut Self {
        let f = self.f.clone();

        // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
        let t_type = TypeId::of::<MsU>();
        if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
            panic!("Handler can return only Msg or ()!");
        }
        let handler = move |sub_ms| {
            let output = &mut Some(handler.clone()(sub_ms)) as &mut dyn Any;
            output.downcast_mut::<Option<Ms>>().and_then(Option::take)
        };

        self.clone_app()
            .data
            .sub_manager
            .borrow_mut()
            .subscribe(move |sub_ms| handler(sub_ms).map(|ms| f(ms)));
        self
    }

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn subscribe_with_handle<MsU: 'static, SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> MsU + Clone + 'static,
    ) -> SubHandle {
        let f = self.f.clone();

        // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
        let t_type = TypeId::of::<MsU>();
        if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
            panic!("Handler can return only Msg or ()!");
        }
        let handler = move |sub_ms| {
            let output = &mut Some(handler.clone()(sub_ms)) as &mut dyn Any;
            output.downcast_mut::<Option<Ms>>().and_then(Option::take)
        };

        self.clone_app()
            .data
            .sub_manager
            .borrow_mut()
            .subscribe_with_handle(move |sub_ms| handler(sub_ms).map(|ms| f(ms)))
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
