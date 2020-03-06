use futures::stream::{Stream, StreamExt};
use gloo_timers::future::IntervalStream;

// ------ Interval stream ------

pub fn interval<Ms>(
    ms: u32,
    handler: impl FnOnce() -> Ms + Clone + 'static,
) -> impl Stream<Item = Ms> {
    IntervalStream::new(ms).map(move |_| handler.clone()())
}

// ------ Window Event stream ------

pub mod window_event;
pub use window_event::window_event;
