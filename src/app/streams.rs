use crate::browser::util::{document, window};
use crate::virtual_dom::Ev;
use futures::stream::{Stream, StreamExt};
use gloo_timers::future::IntervalStream;
use web_sys::Event;

mod event_stream;
use event_stream::EventStream;

mod backoff_stream;
use backoff_stream::BackoffStream;

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

// ------ Backoff stream ------

/// Stream retries count in increasing intervals.
///
/// Algorithm - [Truncated exponential backoff](https://cloud.google.com/storage/docs/exponential-backoff)
///
/// # Arguments
///
/// * `max_seconds` - Typically `32` or `64` seconds. Default is `32`.
/// * `handler` - Receives the number of retries (starting from 1); Has to return `Msg`, `Option<Msg>` or `()`.
///
/// # Example
///
/// ```rust,no_run
///orders.stream(streams::backoff(None, |_retries| Msg::OnTick));
///orders.stream_with_handle(streams::backoff(Some(15), |_| log!("Tick!")));
/// ```
///
/// # Panics
///
/// Panics when the handler doesn't return `Msg`, `Option<Msg>` or `()`.
/// (It will be changed to a compile-time error).
pub fn backoff<MsU>(
    max_seconds: Option<u32>,
    handler: impl FnOnce(usize) -> MsU + Clone + 'static,
) -> impl Stream<Item = MsU> {
    BackoffStream::new(max_seconds.unwrap_or(32)).map(move |retries| handler.clone()(retries))
}

// ------ Window Event stream ------

/// Stream `Window` `web_sys::Event`s.
///
/// Handler has to return `Msg`, `Option<Msg>` or `()`.
///
/// # Example
///
/// ```rust,no_run
///orders.stream(streams::window_event(Ev::Resize, |_| Msg::OnResize));
///orders.stream_with_handle(streams::window_event(Ev::Click, |_| log!("Clicked!")));
/// ```
///
/// # Panics
///
/// Panics when the handler doesn't return `Msg`, `Option<Msg>` or `()`.
/// (It will be changed to a compile-time error).
pub fn window_event<MsU>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(Event) -> MsU + Clone + 'static,
) -> impl Stream<Item = MsU> {
    EventStream::new(&window(), trigger.into()).map(move |event| handler.clone()(event))
}

// ------ Document Event stream ------

/// Stream `Document` `web_sys::Event`s.
///
/// Handler has to return `Msg`, `Option<Msg>` or `()`.
///
/// # Example
///
/// ```rust,no_run
///orders.stream(streams::document_event(Ev::SelectionChange, |_| Msg::OnSelection));
///orders.stream_with_handle(streams::document_event(Ev::SelectionChange, |_| log!("Selection changed!")));
/// ```
///
/// # Panics
///
/// Panics when the handler doesn't return `Msg`, `Option<Msg>` or `()`.
/// (It will be changed to a compile-time error).
pub fn document_event<MsU>(
    trigger: impl Into<Ev>,
    handler: impl FnOnce(Event) -> MsU + Clone + 'static,
) -> impl Stream<Item = MsU> {
    EventStream::new(&document(), trigger.into()).map(move |event| handler.clone()(event))
}
