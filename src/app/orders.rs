use super::{App, CmdHandle, RenderTimestampDelta, StreamHandle, SubHandle, UndefinedGMsg};
use crate::virtual_dom::View;
use futures::stream::Stream;
use std::{any::Any, future::Future};

pub mod container;
pub mod proxy;

pub use container::OrdersContainer;
pub use proxy::OrdersProxy;

pub trait Orders<Ms: 'static, GMs = UndefinedGMsg> {
    type AppMs: 'static;
    type Mdl: 'static;
    type ElC: View<Self::AppMs> + 'static;

    /// Automatically map message type. It allows you to pass `Orders` into child module.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///Msg::Child(child_msg) => {
    ///    child::update(child_msg, &mut model.child, &mut orders.proxy(Msg::Child));
    ///}
    /// ```
    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl FnOnce(ChildMs) -> Ms + 'static + Clone,
    ) -> OrdersProxy<ChildMs, Self::AppMs, Self::Mdl, Self::ElC, GMs>;

    /// Schedule web page rerender after model update. It's the default behaviour.
    fn render(&mut self) -> &mut Self;

    /// Force web page to rerender immediately after model update.
    fn force_render_now(&mut self) -> &mut Self;

    /// Don't rerender web page after model update.
    fn skip(&mut self) -> &mut Self;

    fn notify(&mut self, message: impl Any + Clone) -> &mut Self;

    /// Call function `update` with the given `msg` after model update.
    /// - You can call this function multiple times - messages will be sent in the same order.
    fn send_msg(&mut self, msg: Ms) -> &mut Self;

    /// @TODO
    /// Schedule given future `cmd` to be executed after model update.
    /// - Result is send to function `update`.
    /// - You can call this function multiple times - futures will be scheduled in the same order.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///async fn write_emoticon_after_delay() -> Msg {
    ///    TimeoutFuture::new(2_000).await;
    ///    Msg::WriteEmoticon
    ///}
    ///orders.perform_cmd(write_emoticon_after_delay());
    /// ```
    fn perform_cmd(&mut self, cmd: impl Future<Output = Ms> + 'static) -> &mut Self;

    #[must_use]
    fn perform_cmd_with_handle(&mut self, cmd: impl Future<Output = Ms> + 'static) -> CmdHandle;

    /// Similar to `send_msg`, but calls function `sink` with the given global message.
    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self;

    /// Similar to `perform_cmd`, but result is send to function `sink`.
    fn perform_g_cmd(&mut self, g_cmd: impl Future<Output = GMs> + 'static) -> &mut Self;

    /// Similar to `perform_g_cmd`, but result is send to function `sink`.
    fn perform_g_cmd_with_handle(
        &mut self,
        g_cmd: impl Future<Output = GMs> + 'static,
    ) -> CmdHandle;

    /// Get app instance. Cloning is cheap because `App` contains only `Rc` fields.
    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::ElC, GMs>;

    /// Get function which maps module's `Msg` to app's (root's) one.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());
    ///app.update(msg_mapper(Msg::AMessage));
    /// ```
    fn msg_mapper(&self) -> Box<dyn Fn(Ms) -> Self::AppMs>;

    /// Register the callback that will be executed after the next render.
    ///
    /// Callback's only parameter is `Option<RenderTimestampDelta>` - the difference between
    /// the old render timestamp and the new one.
    /// The parameter has value `None` if it's the first rendering.
    ///
    /// - It's useful when you want to use DOM API or make animations.
    /// - You can call this function multiple times - callbacks will be executed in the same order.
    ///
    /// _Note:_ [performance.now()](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)
    ///  is used under the hood to get timestamps.
    fn after_next_render(
        &mut self,
        callback: impl FnOnce(Option<RenderTimestampDelta>) -> Ms + 'static,
    ) -> &mut Self;

    fn subscribe<SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> Ms + Clone + 'static,
    ) -> &mut Self;

    #[must_use]
    fn subscribe_with_handle<SubMs: 'static + Clone>(
        &mut self,
        handler: impl FnOnce(SubMs) -> Ms + Clone + 'static,
    ) -> SubHandle;

    fn stream(&mut self, stream: impl Stream<Item = Ms> + 'static) -> &mut Self;

    #[must_use]
    fn stream_with_handle(&mut self, stream: impl Stream<Item = Ms> + 'static) -> StreamHandle;
}
