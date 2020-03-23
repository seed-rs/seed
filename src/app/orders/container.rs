use crate::app::orders::{proxy::OrdersProxy, Orders};
use crate::app::{
    effects::Effect, render_timestamp_delta::RenderTimestampDelta, App, CmdHandle, CmdManager,
    Notification, ShouldRender, StreamHandle, StreamManager, SubHandle, UndefinedGMsg,
};
use crate::virtual_dom::IntoNodes;
use futures::future::FutureExt;
use futures::stream::{Stream, StreamExt};
use std::{
    any::{Any, TypeId},
    collections::VecDeque,
    convert::identity,
    future::Future,
};

#[allow(clippy::module_name_repetitions)]
pub struct OrdersContainer<Ms: 'static, Mdl: 'static, INodes: IntoNodes<Ms>, GMs = UndefinedGMsg> {
    pub(crate) should_render: ShouldRender,
    pub(crate) effects: VecDeque<Effect<Ms, GMs>>,
    app: App<Ms, Mdl, INodes, GMs>,
}

impl<Ms, Mdl, INodes: IntoNodes<Ms>, GMs> OrdersContainer<Ms, Mdl, INodes, GMs> {
    pub fn new(app: App<Ms, Mdl, INodes, GMs>) -> Self {
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

impl<Ms: 'static, Mdl, INodes: IntoNodes<Ms> + 'static, GMs: 'static> Orders<Ms, GMs>
    for OrdersContainer<Ms, Mdl, INodes, GMs>
{
    type AppMs = Ms;
    type Mdl = Mdl;
    type INodes = INodes;

    #[allow(clippy::redundant_closure)]
    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl FnOnce(ChildMs) -> Ms + 'static + Clone,
    ) -> OrdersProxy<ChildMs, Ms, Mdl, INodes, GMs> {
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
        self.effects.push_back(msg.into());
        self
    }

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn perform_cmd<MsU: 'static>(&mut self, cmd: impl Future<Output = MsU> + 'static) -> &mut Self {
        let app = self.app.clone();
        let cmd = cmd.map(move |msg_or_unit| {
            // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
            let t_type = TypeId::of::<MsU>();
            if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
                panic!("Cmds can return only Msg or ()!");
            }
            let msg_or_unit = &mut Some(msg_or_unit) as &mut dyn Any;
            if let Some(msg) = msg_or_unit
                .downcast_mut::<Option<Ms>>()
                .and_then(Option::take)
            {
                app.update(msg)
            }
        });
        CmdManager::perform_cmd(cmd);
        self
    }

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn perform_cmd_with_handle<MsU: 'static>(
        &mut self,
        cmd: impl Future<Output = MsU> + 'static,
    ) -> CmdHandle {
        let app = self.app.clone();
        let cmd = cmd.map(move |msg_or_unit| {
            // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
            let t_type = TypeId::of::<MsU>();
            if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
                panic!("Cmds can return only Msg or ()!");
            }
            let msg_or_unit = &mut Some(msg_or_unit) as &mut dyn Any;
            if let Some(msg) = msg_or_unit
                .downcast_mut::<Option<Ms>>()
                .and_then(Option::take)
            {
                app.update(msg)
            }
        });
        CmdManager::perform_cmd_with_handle(cmd)
    }

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self {
        let effect = Effect::GMsg(g_msg);
        self.effects.push_back(effect);
        self
    }

    fn perform_g_cmd(&mut self, cmd: impl Future<Output = GMs> + 'static) -> &mut Self {
        let app = self.app.clone();
        let cmd = cmd.map(move |msg| app.sink(msg));
        CmdManager::perform_cmd(cmd);
        self
    }

    fn perform_g_cmd_with_handle(&mut self, cmd: impl Future<Output = GMs> + 'static) -> CmdHandle {
        let app = self.app.clone();
        let cmd = cmd.map(move |msg| app.sink(msg));
        CmdManager::perform_cmd_with_handle(cmd)
    }

    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::INodes, GMs> {
        self.app.clone()
    }

    fn msg_mapper(&self) -> Box<dyn Fn(Ms) -> Self::AppMs> {
        Box::new(identity)
    }

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn after_next_render<MsU: 'static>(
        &mut self,
        callback: impl FnOnce(Option<RenderTimestampDelta>) -> MsU + 'static,
    ) -> &mut Self {
        // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
        let t_type = TypeId::of::<MsU>();
        if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
            panic!("Callback can return only Msg or ()!");
        }
        let callback = move |delta| {
            let output = &mut Some(callback(delta)) as &mut dyn Any;
            output.downcast_mut::<Option<Ms>>().and_then(Option::take)
        };

        self.app
            .data
            .after_next_render_callbacks
            .borrow_mut()
            .push(Box::new(callback));
        self
    }

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn subscribe<MsU: 'static, SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> MsU + Clone + 'static,
    ) -> &mut Self {
        // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
        let t_type = TypeId::of::<MsU>();
        if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
            panic!("Handler can return only Msg or ()!");
        }
        let handler = move |sub_ms| {
            let output = &mut Some(handler.clone()(sub_ms)) as &mut dyn Any;
            output.downcast_mut::<Option<Ms>>().and_then(Option::take)
        };

        self.app.data.sub_manager.borrow_mut().subscribe(handler);
        self
    }

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn subscribe_with_handle<MsU: 'static, SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> MsU + Clone + 'static,
    ) -> SubHandle {
        // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
        let t_type = TypeId::of::<MsU>();
        if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
            panic!("Handler can return only Msg or ()!");
        }
        let handler = move |sub_ms| {
            let output = &mut Some(handler.clone()(sub_ms)) as &mut dyn Any;
            output.downcast_mut::<Option<Ms>>().and_then(Option::take)
        };

        self.app
            .data
            .sub_manager
            .borrow_mut()
            .subscribe_with_handle(handler)
    }

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn stream<MsU: 'static>(&mut self, stream: impl Stream<Item = MsU> + 'static) -> &mut Self {
        let app = self.app.clone();
        let stream = stream.map(move |msg_or_unit| {
            // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
            let t_type = TypeId::of::<MsU>();
            if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
                panic!("Streams can stream only Msg or ()!");
            }
            let msg_or_unit = &mut Some(msg_or_unit) as &mut dyn Any;
            if let Some(msg) = msg_or_unit
                .downcast_mut::<Option<Ms>>()
                .and_then(Option::take)
            {
                app.update(msg)
            }
        });
        StreamManager::stream(stream);
        self
    }

    #[allow(clippy::shadow_unrelated)]
    // @TODO remove `'static`s once `optin_builtin_traits`
    // @TODO or https://github.com/rust-lang/rust/issues/41875 is stable
    fn stream_with_handle<MsU: 'static>(
        &mut self,
        stream: impl Stream<Item = MsU> + 'static,
    ) -> StreamHandle {
        let app = self.app.clone();
        let stream = stream.map(move |msg_or_unit| {
            // @TODO refactor once `optin_builtin_traits` is stable (https://github.com/seed-rs/seed/issues/391)
            let t_type = TypeId::of::<MsU>();
            if t_type != TypeId::of::<Ms>() && t_type != TypeId::of::<()>() {
                panic!("Streams can stream only Msg or ()!");
            }
            let msg_or_unit = &mut Some(msg_or_unit) as &mut dyn Any;
            if let Some(msg) = msg_or_unit
                .downcast_mut::<Option<Ms>>()
                .and_then(Option::take)
            {
                app.update(msg)
            }
        });
        StreamManager::stream_with_handle(stream)
    }
}
