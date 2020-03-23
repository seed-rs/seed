use futures::stream::{Stream, StreamExt};
use gloo_timers::future::IntervalStream;

// ------ Interval stream ------

/// Stream no values on predefined time interval in milliseconds.
///
/// Handler has to return `Msg`, `Option<Msg>` or `()`.
///
/// # Example
///
/// ```rust,no_run
///orders.stream(streams::interval(1000, || Msg::OnTick));
///orders.stream_with_handle(streams::interval(1000, || log!("Tick!")));
/// ```
///
/// # Panics
///
/// Panics when the handler doesn't return `Msg`, `Option<Msg>` or `()`.
/// (It will be changed to a compile-time error).
pub fn interval<MsU>(
    ms: u32,
    handler: impl FnOnce() -> MsU + Clone + 'static,
) -> impl Stream<Item = MsU> {
    IntervalStream::new(ms).map(move |_| handler.clone()())
}

// ------ Window Event stream ------

mod window_event;
pub use window_event::window_event;
