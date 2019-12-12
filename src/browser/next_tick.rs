use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// Code + docs: https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/

/// A future that becomes ready after a tick of the micro task queue.
pub struct NextTick {
    inner: JsFuture,
}

impl NextTick {
    /// Construct a new `NextTick` future.
    pub fn new() -> NextTick {
        // Create a resolved promise that will run its callbacks on the next
        // tick of the micro task queue.
        let promise = js_sys::Promise::resolve(&JsValue::NULL);
        // Convert the promise into a `JsFuture`.
        let inner = JsFuture::from(promise);
        NextTick { inner }
    }
}

impl Default for NextTick {
    fn default() -> Self {
        Self::new()
    }
}

impl Future for NextTick {
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<()> {
        // Polling a `NextTick` just forwards to polling if the inner promise is
        // ready.
        match Pin::new(&mut self.get_mut().inner).poll(ctx) {
            Poll::Ready(_) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}
