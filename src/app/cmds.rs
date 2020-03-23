use futures::future::{Future, FutureExt};
use gloo_timers::future::TimeoutFuture;

// @TODO add fetch cmd?

// ------ Timeout cmd ------

/// Set timeout in milliseconds.
///
/// Handler has to return `Msg`, `Option<Msg>` or `()`.
///
/// # Example
///
/// ```rust,no_run
///orders.perform_cmd_with_handle(cmds::timeout(2000, || Msg::OnTimeout));
///orders.perform_cmd(cmds::timeout(1000, || log!("Tick!")));
/// ```
///
/// # Panics
///
/// Panics when the command doesn't return `Msg`, `Option<Msg>` or `()`.
/// (It will be changed to a compile-time error).
pub fn timeout<MsU>(
    ms: u32,
    handler: impl FnOnce() -> MsU + Clone + 'static,
) -> impl Future<Output = MsU> {
    TimeoutFuture::new(ms).map(move |_| handler())
}
