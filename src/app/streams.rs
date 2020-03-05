use futures::stream::{Stream, StreamExt};
use gloo_timers::future::IntervalStream;

pub fn interval<Ms>(
    ms: u32,
    handler: impl FnOnce() -> Ms + Clone + 'static,
) -> impl Stream<Item = Ms> {
    IntervalStream::new(ms).map(move |_| handler.clone()())
}
