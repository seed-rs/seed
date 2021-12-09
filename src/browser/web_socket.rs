#![allow(clippy::module_name_repetitions)]

use crate::app::Orders;
use gloo_file::FileReadError;
use js_sys::JSON;
use serde::Serialize;
use serde_wasm_bindgen as swb;
use wasm_bindgen::{JsCast, JsValue};

mod builder;
mod message;

pub use builder::Builder;
use builder::Callbacks;
pub use message::WebSocketMessage;

// ------ ALIASES ------

/// Convenient type alias.
pub type Result<T> = std::result::Result<T, WebSocketError>;

/// `WebSocket` message data is either text or binary.
/// Binary data can be represented as `Blob` (default) or `ArrayBuffer`.
pub type BinaryType = web_sys::BinaryType;

/// One of the binary data types.
///
/// _Note:_: `gloo`'s `Blob` is used to make the usage more comfortable and async-friendly.
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
pub type Blob = gloo_file::Blob;

/// Represents the current state of the `WebSocket` connection.
/// - `State::Connecting` - Socket has been created. The connection is not yet open.
/// - `State::Open` - The connection is open and ready to communicate.
/// - `State::Closing` - The connection is in the process of closing.
/// - `State::Closed` - The connection is closed or couldn't be opened.
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)
pub type State = web_sys::TcpReadyState;

/// A `CloseEvent` is sent to clients using Web Sockets when the connection is closed.
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)
pub type CloseEvent = web_sys::CloseEvent;

// ------ WebSocketError ------

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
/// `WebSocket` error enum.
/// You can find more details in documentation for methods that return those errors.
pub enum WebSocketError {
    TextError(&'static str),
    SendError(JsValue),
    SerdeError(swb::Error),
    ConversionError,
    PromiseError(JsValue),
    FileReaderError(FileReadError),
    OpenError(JsValue),
    CloseError(JsValue),
}

impl From<swb::Error> for WebSocketError {
    fn from(v: swb::Error) -> Self {
        Self::SerdeError(v)
    }
}

// ------ WebSocket ------

/// `WebSocket` is the most important item in the Web Socket API.
/// - It's created by the `Builder` (see example below).
/// - Should be saved into app's `Model` because the connection is closed on drop.
///
/// _Note:_: `CloseEvent` won't be passed to handler if the connection has been closed on drop.
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
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
#[derive(Debug)]
#[must_use = "WebSocket is closed on drop"]
pub struct WebSocket {
    ws: web_sys::WebSocket,

    #[allow(dead_code)]
    callbacks: Callbacks,
}

impl WebSocket {
    /// Creates a new `Builder`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let web_socket = WebSocket::builder("ws://127.0.0.1:9000/ws", orders)
    ///     .on_message(Msg::MessageReceived)
    ///     .build_and_open();
    ///```
    ///
    /// _Note:_ Always prefer `wss://` - encrypted and more reliable.
    pub fn builder<U: AsRef<str>, Ms: 'static, O: Orders<Ms>>(
        url: U,
        orders: &O,
    ) -> Builder<U, Ms, O> {
        Builder::new(url, orders)
    }

    /// Send string message.
    ///
    /// # Errors
    ///
    /// Returns error when sending fails.
    pub fn send_text<S>(&self, message: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.ws
            .send_with_str(message.as_ref())
            .map_err(WebSocketError::SendError)
    }

    /// Send message with JSON encoded provided data.
    ///
    /// # Errors
    ///
    /// Returns error when JSON serialization or sending fails.
    pub fn send_json<T: Serialize + ?Sized>(&self, data: &T) -> Result<()> {
        let data: String = JSON::stringify(&swb::to_value(data)?)
            .map_err(|_| WebSocketError::ConversionError)?
            .into();
        self.send_text(data)
    }

    /// Send byte message.
    ///
    /// # Errors
    ///
    /// Returns error when sending fails.
    pub fn send_bytes(&self, message: &[u8]) -> Result<()> {
        self.ws
            .send_with_u8_array(message)
            .map_err(WebSocketError::SendError)
    }

    /// Returns the number of bytes of data that have been queued using `send_*` calls
    /// but not yet transmitted to the network. This value resets to zero once all queued data has been sent.
    /// This value does not reset to zero when the connection is closed;
    /// if you keep calling `send_*`, this will continue to climb.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/bufferedAmount)
    pub fn buffered_amount(&self) -> u32 {
        self.ws.buffered_amount()
    }

    /// Returns the name of the sub-protocol the server selected; this will be one of the strings
    /// specified in the `Builder` method `protocols` when creating the `WebSocket` instance,
    /// or the empty string if no connection is established.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/protocol)
    pub fn protocol(&self) -> String {
        self.ws.protocol()
    }

    /// Returns the extensions selected by the server.
    /// This is currently only the empty string or a list of extensions as negotiated by the connection.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/extensions)
    pub fn extensions(&self) -> String {
        self.ws.extensions()
    }

    /// Closes the Web Socket connection or connection attempt, if any.
    /// If the connection is already closed, this method does nothing.
    ///
    /// # Arguments
    ///
    /// * `code` - The status code explaining why the connection is being closed.
    /// `1000` or `3000`-`4999`. Default is `1000`.
    /// [Status codes](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent#Status_codes).
    ///
    /// * `reason` - A human-readable string explaining why the connection is closing.
    /// This string must be no longer than 123 bytes of UTF-8 text (**not** characters).
    /// Default is the empty string.
    ///
    /// _Note:_ `code` and `reason` will be send to the server.
    ///
    /// # Errors
    ///
    /// Returns `WebSocketError::CloseError` when:
    /// - Invalid `code` was specified.
    /// - The `reason` string is too long or contains unpaired surrogates.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)
    pub fn close(&self, code: Option<u16>, reason: Option<&str>) -> Result<()> {
        self.ws
            .close_with_code_and_reason(code.unwrap_or(1000), reason.unwrap_or_default())
            .map_err(WebSocketError::CloseError)
    }

    /// Returns the current state of the `WebSocket` connection.
    /// - `State::Connecting` - Socket has been created. The connection is not yet open.
    /// - `State::Open` - The connection is open and ready to communicate.
    /// - `State::Closing` - The connection is in the process of closing.
    /// - `State::Closed` - The connection is closed or couldn't be opened.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)
    #[allow(clippy::missing_panics_doc)]
    pub fn state(&self) -> State {
        match self.ws.ready_state() {
            0 => State::Connecting,
            1 => State::Open,
            2 => State::Closing,
            3 => State::Closed,
            state_id => panic!("unknown WebSocket State id: {}", state_id),
        }
    }

    /// Get underlying `web_sys::WebSocket`.
    ///
    /// This is an escape path if current API can't handle your needs.
    /// Should you find yourself using it, please consider [opening an issue][issue].
    ///
    /// [issue]: https://github.com/seed-rs/seed/issues
    pub const fn raw_web_socket(&self) -> &web_sys::WebSocket {
        &self.ws
    }

    /// This method is private because it should be used only by the `Builder`.
    fn new(
        url: &str,
        callbacks: Callbacks,
        protocols: &[&str],
        binary_type: Option<BinaryType>,
    ) -> Result<Self> {
        let ws = {
            if protocols.is_empty() {
                web_sys::WebSocket::new(url).map_err(WebSocketError::OpenError)?
            } else {
                let protocol_array = protocols
                    .iter()
                    .map(|protocol| JsValue::from(*protocol))
                    .collect::<js_sys::Array>();
                web_sys::WebSocket::new_with_str_sequence(url, &JsValue::from(&protocol_array))
                    .map_err(WebSocketError::OpenError)?
            }
        };

        if let Some(binary_type) = binary_type {
            ws.set_binary_type(binary_type);
        }

        if let Some(on_open) = &callbacks.on_open {
            ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        }
        if let Some(on_close) = &callbacks.on_close {
            ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        }
        if let Some(on_error) = &callbacks.on_error {
            ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        }
        if let Some(on_message) = &callbacks.on_message {
            ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        }

        Ok(Self { ws, callbacks })
    }
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        if matches!(self.state(), State::Connecting | State::Open) {
            self.ws.close().expect("close WebSocket connection");
        }
        self.ws.set_onopen(None);
        self.ws.set_onclose(None);
        self.ws.set_onerror(None);
        self.ws.set_onmessage(None);
    }
}
