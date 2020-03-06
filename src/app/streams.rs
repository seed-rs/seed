use futures::stream::{Stream, StreamExt};
use gloo_timers::future::IntervalStream;

// ------ Interval stream ------

/// Stream no values on predefined time interval in milliseconds.
///
/// # Example
///
/// ```rust,no_run
///orders.stream_with_handle(streams::interval(1000, || Msg::OnTick));
/// ```
pub fn interval<Ms>(
    ms: u32,
    handler: impl FnOnce() -> Ms + Clone + 'static,
) -> impl Stream<Item = Ms> {
    IntervalStream::new(ms).map(move |_| handler.clone()())
}

// ------ Window Event stream ------

mod window_event;
pub use window_event::window_event;
