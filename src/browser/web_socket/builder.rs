use super::{BinaryType, CloseEvent, Result, WebSocket, WebSocketMessage};
use crate::app::Orders;
use std::marker::PhantomData;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::MessageEvent;

// ------ Callbacks ------

// `Callbacks` are used internally by `WebSocket` and `Builder`.
#[derive(Default, Debug)]
pub struct Callbacks {
    pub on_open: Option<Closure<dyn Fn(JsValue)>>,
    pub on_close: Option<Closure<dyn Fn(JsValue)>>,
    pub on_error: Option<Closure<dyn Fn(JsValue)>>,
    pub on_message: Option<Closure<dyn Fn(MessageEvent)>>,
}

// ------ Builder ------

/// `Builder` creates a new `WebSocket` instance.
///
/// # Example
///
/// ```rust,no_run
/// enum Msg { MessageReceived(WebSocketMessage) }
/// ...
/// let web_socket = WebSocket::builder("ws://127.0.0.1:9000/ws", orders)
///     .on_message(Msg::MessageReceived)
///     .build_and_open();
///```
pub struct Builder<'a, U: AsRef<str>, Ms: 'static, O: Orders<Ms>> {
    url: U,
    orders: &'a O,
    callbacks: Callbacks,
    protocols: &'a [&'a str],
    binary_type: Option<BinaryType>,
    phantom: PhantomData<Ms>,
}

impl<'a, U: AsRef<str>, Ms: 'static, O: Orders<Ms>> Builder<'a, U, Ms, O> {
    // Note: `WebSocket::builder` is the preferred way how to crate a new `Builder` instance.
    pub(crate) fn new(url: U, orders: &'a O) -> Self {
        Self {
            url,
            orders,
            callbacks: Callbacks::default(),
            protocols: &[],
            binary_type: None,
            phantom: PhantomData,
        }
    }

    /// Set preferred Web Socket sub-protocols.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)
    pub fn protocols(mut self, protocols: &'a [&'a str]) -> Self {
        self.protocols = protocols;
        self
    }

    /// Set binary data type to `ArrayBuffer`.
    ///
    /// _Notes:_:
    /// - Default binary type is `Blob`.
    /// - For small binary messages, like CBOR, `ArrayBuffer` is more efficient than Blob handling.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)
    pub fn use_array_buffers(mut self) -> Self {
        self.binary_type = Some(BinaryType::Arraybuffer);
        self
    }

    /// Set `on_open` Web Socket handler. The handler is called when connection's state changes
    /// to `State::Open`; this indicates that the connection is ready to send and receive data.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)
    #[allow(clippy::missing_panics_doc)]
    pub fn on_open<MsU: 'static>(
        mut self,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> Self {
        let handler = map_callback_return_to_option_ms!(
            dyn Fn(JsValue) -> Option<Ms>,
            // The event is generic - doesn't contain any useful information.
            |_| handler.clone()(),
            "WebSocket handler on_open can return only Msg, Option<Msg> or ()!",
            Rc
        );
        let callback = create_js_handler(handler, self.orders);
        self.callbacks.on_open = Some(callback);
        self
    }

    /// Set `on_close` Web Socket handler. The handler is called when connection's state changes
    /// to `State::Closed`.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)
    #[allow(clippy::missing_panics_doc)]
    pub fn on_close<MsU: 'static>(
        mut self,
        handler: impl FnOnce(CloseEvent) -> MsU + Clone + 'static,
    ) -> Self {
        let handler = map_callback_return_to_option_ms!(
            dyn Fn(JsValue) -> Option<Ms>,
            |event: JsValue| { handler.clone()(event.unchecked_into()) },
            "WebSocket handler on_close can return only Msg, Option<Msg> or ()!",
            Rc
        );
        let callback = create_js_handler(handler, self.orders);
        self.callbacks.on_close = Some(callback);
        self
    }

    /// Set `on_error` Web Socket handler.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)
    #[allow(clippy::missing_panics_doc)]
    pub fn on_error<MsU: 'static>(
        mut self,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> Self {
        let handler = map_callback_return_to_option_ms!(
            dyn Fn(JsValue) -> Option<Ms>,
            // The event is generic - doesn't contain any useful information.
            |_| handler.clone()(),
            "WebSocket handler on_error can return only Msg, Option<Msg> or ()!",
            Rc
        );
        let callback = create_js_handler(handler, self.orders);
        self.callbacks.on_error = Some(callback);
        self
    }

    /// Set `on_message` Web Socket handler. The handler is called when a message is received
    /// from the server.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)
    #[allow(clippy::missing_panics_doc)]
    pub fn on_message<MsU: 'static>(
        mut self,
        handler: impl FnOnce(WebSocketMessage) -> MsU + Clone + 'static,
    ) -> Self {
        let handler = map_callback_return_to_option_ms!(
            dyn Fn(MessageEvent) -> Option<Ms>,
            |message_event: MessageEvent| {
                let message = WebSocketMessage {
                    data: message_event.data(),
                    message_event,
                };
                handler.clone()(message)
            },
            "WebSocket handler on_message can return only Msg, Option<Msg> or ()!",
            Rc
        );
        let callback = create_js_handler(handler, self.orders);
        self.callbacks.on_message = Some(callback);
        self
    }

    /// Create a new `WebSocket` instance from the `Builder` and try to open it.
    ///
    /// # Errors
    ///
    /// Returns `WebSocketError::OpenError` when Web Socket opening fails.
    /// E.g. when the chosen port is blocked.
    ///
    /// _Note:_: It doesn't return error when the socket is open on the client side,
    /// but fails to connect to the server - use `on_error` handler to resolve such cases.
    pub fn build_and_open(self) -> Result<WebSocket> {
        WebSocket::new(
            self.url.as_ref(),
            self.callbacks,
            self.protocols,
            self.binary_type,
        )
    }
}

// ------ HELPERS ------

fn create_js_handler<T: wasm_bindgen::convert::FromWasmAbi + 'static, Ms: 'static>(
    handler: Rc<dyn Fn(T) -> Option<Ms>>,
    orders: &impl Orders<Ms>,
) -> Closure<dyn Fn(T)> {
    let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());
    let mailbox = app.mailbox();
    // @TODO replace with `Closure::new` once stable.
    Closure::wrap(Box::new(move |data| {
        #[allow(clippy::redundant_closure)]
        mailbox.send(handler(data).map(|msg| msg_mapper(msg)));
    }) as Box<dyn Fn(T)>)
}
