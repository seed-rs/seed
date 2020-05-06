use crate::virtual_dom::Ev;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::EventTarget;

// ------ EventStream ------

// @TODO Replace `mpsc` with `crossbeam`, `futures-signals` or `flume`?
// (And integrate it into the other Seed parts (e.g. `Listener`, `SubManager`, `BackoffStream`)).

// @TODO Update it to support different `web_sys` events
// during implementation of https://github.com/seed-rs/seed/issues/331

#[derive(Debug)]
pub struct EventStream<E> {
    node: EventTarget,
    trigger: Ev,
    callback: Closure<dyn Fn(JsValue)>,
    receiver: UnboundedReceiver<E>,
}

impl<E> EventStream<E>
where
    E: JsCast + 'static,
{
    pub fn new(node: &EventTarget, trigger: impl Into<Ev>) -> Self {
        let trigger = trigger.into();

        let (sender, receiver) = unbounded();

        // @TODO replace with `Closure::new` once stable (or use the Seed's temporary one).
        let callback = Closure::wrap(Box::new(move |event: JsValue| {
            sender.unbounded_send(event.dyn_into().unwrap()).unwrap();
        }) as Box<dyn Fn(JsValue)>);

        node.add_event_listener_with_callback(trigger.as_str(), callback.as_ref().unchecked_ref())
            .unwrap();

        Self {
            node: node.clone(),
            trigger,
            callback,
            receiver,
        }
    }
}

impl<E> Stream for EventStream<E> {
    type Item = E;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.receiver), cx)
    }
}

impl<E> Drop for EventStream<E> {
    fn drop(&mut self) {
        self.node
            .remove_event_listener_with_callback(
                self.trigger.as_str(),
                self.callback.as_ref().unchecked_ref(),
            )
            .unwrap();
    }
}
