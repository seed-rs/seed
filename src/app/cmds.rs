use futures::future::{Future, FutureExt};
use gloo_timers::future::TimeoutFuture;

// ------ Timeout cmd ------

pub fn timeout<Ms>(
    ms: u32,
    handler: impl FnOnce() -> Ms + Clone + 'static,
) -> impl Future<Output = Ms> {
    TimeoutFuture::new(ms).map(move |_| handler())
}
