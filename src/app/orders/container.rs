use crate::app::cmd_manager::CmdManager;
use crate::app::orders::{proxy::OrdersProxy, Orders};
use crate::app::stream_manager::StreamManager;
use crate::app::{
    App, CmdHandle, Effect, Notification, RenderInfo, ShouldRender, StreamHandle, SubHandle,
};
use crate::virtual_dom::IntoNodes;
use futures::future::FutureExt;
use futures::stream::{Stream, StreamExt};
use std::{any::Any, collections::VecDeque, convert::identity, future::Future, rc::Rc};

#[allow(clippy::module_name_repetitions)]
pub struct OrdersContainer<Ms, Mdl, INodes>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    pub(crate) should_render: ShouldRender,
    pub(crate) effects: VecDeque<Effect<Ms>>,
    app: App<Ms, Mdl, INodes>,
}

impl<Ms, Mdl, INodes> OrdersContainer<Ms, Mdl, INodes>
where
    INodes: IntoNodes<Ms> + 'static,
{
    pub fn new(app: App<Ms, Mdl, INodes>) -> Self {
        Self {
            should_render: ShouldRender::Render,
            effects: VecDeque::<Effect<Ms>>::new(),
            app,
        }
    }
}

impl<Ms, Mdl, INodes> Orders<Ms> for OrdersContainer<Ms, Mdl, INodes>
where
    Ms: 'static,
    INodes: IntoNodes<Ms> + 'static,
{
    type AppMs = Ms;
    type Mdl = Mdl;
    type INodes = INodes;

    #[allow(clippy::redundant_closure)]
    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl FnOnce(ChildMs) -> Ms + 'static + Clone,
    ) -> OrdersProxy<ChildMs, Ms, Mdl, INodes> {
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

    fn notify(&mut self, message: impl Any + Clone) -> &mut Self {
        self.effects
            .push_back(Effect::Notification(Notification::new(message)));
        self
    }

    fn send_msg(&mut self, msg: Ms) -> &mut Self {
        self.effects.push_back(Effect::Msg(Some(msg)));
        self
    }

    fn perform_cmd<MsU: 'static>(&mut self, cmd: impl Future<Output = MsU> + 'static) -> &mut Self {
        let app = self.app.clone();

        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MsU) -> Option<Ms>,
            identity,
            "Cmds can return only Msg, Option<Msg> or ()!",
            Box
        );

        let cmd = cmd.map(move |msg| app.mailbox().send(handler(msg)));
        CmdManager::perform_cmd(cmd);
        self
    }

    fn perform_cmd_with_handle<MsU: 'static>(
        &mut self,
        cmd: impl Future<Output = MsU> + 'static,
    ) -> CmdHandle {
        let app = self.app.clone();

        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MsU) -> Option<Ms>,
            identity,
            "Cmds can return only Msg, Option<Msg> or ()!",
            Box
        );

        let cmd = cmd.map(move |msg| app.mailbox().send(handler(msg)));
        CmdManager::perform_cmd_with_handle(cmd)
    }

    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::INodes> {
        self.app.clone()
    }

    fn msg_mapper(&self) -> Rc<dyn Fn(Ms) -> Self::AppMs> {
        Rc::new(identity)
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

        self.app
            .data
            .after_next_render_callbacks
            .borrow_mut()
            .push(callback);
        self
    }

    fn subscribe<MsU: 'static, SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> MsU + Clone + 'static,
    ) -> &mut Self {
        #[allow(clippy::redundant_closure)]
        let handler = map_callback_return_to_option_ms!(
            dyn Fn(SubMs) -> Option<Ms>,
            handler.clone(),
            "Handler can return only Msg, Option<Msg> or ()!",
            Rc
        );

        #[allow(clippy::redundant_closure)]
        self.app
            .data
            .sub_manager
            .borrow_mut()
            .subscribe(move |sub_ms| handler(sub_ms));
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

        #[allow(clippy::redundant_closure)]
        self.app
            .data
            .sub_manager
            .borrow_mut()
            .subscribe_with_handle(move |sub_ms| handler(sub_ms))
    }

    fn stream<MsU: 'static>(&mut self, stream: impl Stream<Item = MsU> + 'static) -> &mut Self {
        let app = self.app.clone();

        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MsU) -> Option<Ms>,
            identity,
            "Streams can stream only Msg, Option<Msg> or ()!",
            Box
        );

        let stream = stream.map(move |msg| app.mailbox().send(handler(msg)));
        StreamManager::stream(stream);
        self
    }

    fn stream_with_handle<MsU: 'static>(
        &mut self,
        stream: impl Stream<Item = MsU> + 'static,
    ) -> StreamHandle {
        let app = self.app.clone();

        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MsU) -> Option<Ms>,
            identity,
            "Streams can stream only Msg, Option<Msg> or ()!",
            Box
        );

        let stream = stream.map(move |msg| app.mailbox().send(handler(msg)));
        StreamManager::stream_with_handle(stream)
    }
}
