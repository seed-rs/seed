use super::{
    super::{App, CmdHandle, RenderInfo, StreamHandle, SubHandle},
    Orders, OrdersContainer,
};

use crate::app::cmd_manager::CmdManager;
use crate::app::stream_manager::StreamManager;
use crate::virtual_dom::IntoNodes;
use futures::future::{Future, FutureExt};
use futures::stream::{Stream, StreamExt};
use std::{any::Any, convert::identity, rc::Rc};

#[allow(clippy::module_name_repetitions)]
pub struct OrdersProxy<'a, Ms, AppMs, Mdl, INodes>
where
    AppMs: 'static,
    Mdl: 'static,
    INodes: IntoNodes<AppMs>,
{
    orders_container: &'a mut OrdersContainer<AppMs, Mdl, INodes>,
    f: Rc<dyn Fn(Ms) -> AppMs>,
}

impl<'a, Ms, AppMs, Mdl, INodes> OrdersProxy<'a, Ms, AppMs, Mdl, INodes>
where
    Ms: 'static,
    AppMs: 'static,
    INodes: IntoNodes<AppMs>,
{
    pub fn new(
        orders_container: &'a mut OrdersContainer<AppMs, Mdl, INodes>,
        f: impl Fn(Ms) -> AppMs + 'static,
    ) -> Self {
        OrdersProxy {
            orders_container,
            f: Rc::new(f),
        }
    }
}

impl<'a, Ms, AppMs, Mdl, INodes> Orders<Ms> for OrdersProxy<'a, Ms, AppMs, Mdl, INodes>
where
    Ms: 'static,
    AppMs: 'static,
    INodes: IntoNodes<AppMs> + 'static,
{
    type AppMs = AppMs;
    type Mdl = Mdl;
    type INodes = INodes;

    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl FnOnce(ChildMs) -> Ms + 'static + Clone,
    ) -> OrdersProxy<ChildMs, AppMs, Mdl, INodes> {
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
    fn perform_cmd<MsU: 'static>(&mut self, cmd: impl Future<Output = MsU> + 'static) -> &mut Self {
        let f = self.f.clone();
        let app = self.clone_app();

        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MsU) -> Option<Ms>,
            identity,
            "Cmds can return only Msg, Option<Msg> or ()!",
            Box
        );

        let cmd = cmd.map(move |msg| app.mailbox().send(handler(msg).map(|msg| f(msg))));
        CmdManager::perform_cmd(cmd);
        self
    }

    fn perform_cmd_with_handle<MsU: 'static>(
        &mut self,
        cmd: impl Future<Output = MsU> + 'static,
    ) -> CmdHandle {
        let f = self.f.clone();
        let app = self.clone_app();

        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MsU) -> Option<Ms>,
            identity,
            "Cmds can return only Msg, Option<Msg> or ()!",
            Box
        );

        #[allow(clippy::redundant_closure)]
        let cmd = cmd.map(move |msg| app.mailbox().send(handler(msg).map(|msg| f(msg))));
        CmdManager::perform_cmd_with_handle(cmd)
    }

    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::INodes> {
        self.orders_container.clone_app()
    }

    #[allow(clippy::redundant_closure)]
    fn msg_mapper(&self) -> Rc<dyn Fn(Ms) -> Self::AppMs> {
        let f = self.f.clone();
        Rc::new(move |ms| f(ms))
    }

    fn after_next_render<MsU: 'static>(
        &mut self,
        callback: impl FnOnce(RenderInfo) -> MsU + 'static,
    ) -> &mut Self {
        let callback = map_callback_return_to_option_ms!(
            dyn FnOnce(RenderInfo) -> Option<Ms>,
            callback,
            "Callback can return only Msg, Option<Msg> or ()!",
            Box
        );

        let f = self.f.clone();
        #[allow(clippy::redundant_closure)]
        self.clone_app()
            .data
            .after_next_render_callbacks
            .borrow_mut()
            .push(Box::new(move |render_info| {
                callback(render_info).map(|ms| f(ms))
            }));
        self
    }

    fn subscribe<MsU: 'static, SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> MsU + Clone + 'static,
    ) -> &mut Self {
        let handler = map_callback_return_to_option_ms!(
            dyn Fn(SubMs) -> Option<Ms>,
            handler.clone(),
            "Handler can return only Msg, Option<Msg> or ()!",
            Rc
        );

        let f = self.f.clone();
        #[allow(clippy::redundant_closure)]
        self.clone_app()
            .data
            .sub_manager
            .borrow_mut()
            .subscribe(move |sub_ms| handler(sub_ms).map(|ms| f(ms)));
        self
    }

    fn subscribe_with_handle<MsU: 'static, SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> MsU + Clone + 'static,
    ) -> SubHandle {
        let handler = map_callback_return_to_option_ms!(
            dyn Fn(SubMs) -> Option<Ms>,
            handler.clone(),
            "Handler can return only Msg, Option<Msg> or ()!",
            Rc
        );

        let f = self.f.clone();
        #[allow(clippy::redundant_closure)]
        self.clone_app()
            .data
            .sub_manager
            .borrow_mut()
            .subscribe_with_handle(move |sub_ms| handler(sub_ms).map(|ms| f(ms)))
    }

    fn stream<MsU: 'static>(&mut self, stream: impl Stream<Item = MsU> + 'static) -> &mut Self {
        let f = self.f.clone();
        let app = self.clone_app();

        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MsU) -> Option<Ms>,
            identity,
            "Streams can stream only Msg, Option<Msg> or ()!",
            Box
        );

        #[allow(clippy::redundant_closure)]
        let stream = stream.map(move |msg| app.mailbox().send(handler(msg).map(|msg| f(msg))));
        StreamManager::stream(stream);
        self
    }

    fn stream_with_handle<MsU: 'static>(
        &mut self,
        stream: impl Stream<Item = MsU> + 'static,
    ) -> StreamHandle {
        let f = self.f.clone();
        let app = self.clone_app();

        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MsU) -> Option<Ms>,
            identity,
            "Streams can stream only Msg, Option<Msg> or ()!",
            Box
        );

        #[allow(clippy::redundant_closure)]
        let stream = stream.map(move |msg| app.mailbox().send(handler(msg).map(|msg| f(msg))));
        StreamManager::stream_with_handle(stream)
    }
}
