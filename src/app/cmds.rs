use futures::future::{Future, FutureExt};
use gloo_timers::future::TimeoutFuture;

// @TODO add fetch cmd?

// ------ Timeout cmd ------

/// Set timeout in milliseconds.
///
/// # Example
///
/// ```rust,no_run
///orders.perform_cmd_with_handle(cmds::timeout(2000, || Msg::OnTimeout));
/// ```
pub fn timeout<Ms>(
    ms: u32,
    handler: impl FnOnce() -> Ms + Clone + 'static,
) -> impl Future<Output = Ms> {
    TimeoutFuture::new(ms).map(move |_| handler())
}
