use futures::future::{abortable, ready, AbortHandle, FutureExt};
use futures::stream::{Stream, StreamExt};
use wasm_bindgen_futures::spawn_local;

// ------ StreamManager ------

#[derive(Default)]
pub struct StreamManager;

impl StreamManager {
    pub fn stream(stream: impl Stream<Item = ()> + 'static) {
        spawn_local(stream.for_each(|_| ready(())));
    }

    pub fn stream_with_handle(stream: impl Stream<Item = ()> + 'static) -> StreamHandle {
        let stream = stream.for_each(|_| ready(()));
        let (stream, handle) = abortable(stream);
        spawn_local(stream.map(move |_| ()));
        StreamHandle(handle)
    }
}

// ------ StreamHandle ------

pub struct StreamHandle(AbortHandle);

impl Drop for StreamHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}
