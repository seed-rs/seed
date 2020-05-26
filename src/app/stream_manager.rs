use futures::future::{abortable, ready, AbortHandle, FutureExt};
use futures::stream::{Stream, StreamExt};
use wasm_bindgen_futures::spawn_local;

// ------ StreamManager ------

pub(crate) struct StreamManager;

impl StreamManager {
    pub fn stream(stream: impl Stream<Item = ()> + 'static) {
        // Convert `Stream` to `Future` and execute it. The stream is "leaked" into the JS world.
        spawn_local(stream.for_each(|_| ready(())));
    }

    pub fn stream_with_handle(stream: impl Stream<Item = ()> + 'static) -> StreamHandle {
        // Convert `Stream` to `Future`.
        let stream = stream.for_each(|_| ready(()));
        // Create `AbortHandle`.
        let (stream, handle) = abortable(stream);
        // Ignore the error when the future is aborted. I.e. just stop the stream.
        spawn_local(stream.map(move |_| ()));
        StreamHandle(handle)
    }
}

// ------ StreamHandle ------

#[derive(Debug)]
pub struct StreamHandle(AbortHandle);

impl Drop for StreamHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}
