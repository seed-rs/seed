use seed::{prelude::*, *};
use std::rc::Rc;

mod shared;

const WS_URL: &str = "ws://127.0.0.1:9000/ws";

// ------ ------
//     Model
// ------ ------

pub struct Model {
    sent_messages_count: usize,
    messages: Vec<String>,
    input_text: String,
    input_binary: String,
    web_socket: WebSocket,
    web_socket_reconnector: Option<StreamHandle>,
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        sent_messages_count: 0,
        messages: Vec::new(),
        input_text: String::new(),
        input_binary: String::new(),
        web_socket: create_websocket(orders),
        web_socket_reconnector: None,
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    WebSocketOpened,
    TextMessageReceived(shared::ServerMessage),
    BinaryMessageReceived(shared::ServerMessage),
    CloseWebSocket,
    WebSocketClosed(CloseEvent),
    WebSocketFailed,
    ReconnectWebSocket(usize),
    InputTextChanged(String),
    InputBinaryChanged(String),
    SendMessage(shared::ClientMessage),
    SendBinaryMessage(shared::ClientMessage),
}

fn update(msg: Msg, mut model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WebSocketOpened => {
            model.web_socket_reconnector = None;
            log!("WebSocket connection is open now");
        }
        Msg::TextMessageReceived(message) => {
            log!("Client received a text message");
            model.messages.push(message.text);
        }
        Msg::BinaryMessageReceived(message) => {
            log!("Client received binary message");
            model.messages.push(message.text);
        }
        Msg::CloseWebSocket => {
            model.web_socket_reconnector = None;
            model
                .web_socket
                .close(None, Some("user clicked Close button"))
                .unwrap();
        }
        Msg::WebSocketClosed(close_event) => {
            log!("==================");
            log!("WebSocket connection was closed:");
            log!("Clean:", close_event.was_clean());
            log!("Code:", close_event.code());
            log!("Reason:", close_event.reason());
            log!("==================");

            // Chrome doesn't invoke `on_error` when the connection is lost.
            if !close_event.was_clean() && model.web_socket_reconnector.is_none() {
                model.web_socket_reconnector = Some(
                    orders.stream_with_handle(streams::backoff(None, Msg::ReconnectWebSocket)),
                );
            }
        }
        Msg::WebSocketFailed => {
            log!("WebSocket failed");
            if model.web_socket_reconnector.is_none() {
                model.web_socket_reconnector = Some(
                    orders.stream_with_handle(streams::backoff(None, Msg::ReconnectWebSocket)),
                );
            }
        }
        Msg::ReconnectWebSocket(retries) => {
            log!("Reconnect attempt:", retries);
            model.web_socket = create_websocket(orders);
        }
        Msg::InputTextChanged(text) => {
            model.input_text = text;
        }
        Msg::InputBinaryChanged(text) => {
            model.input_binary = text;
        }
        Msg::SendMessage(msg) => {
            model.web_socket.send_json(&msg).unwrap();
            model.input_text.clear();
            model.sent_messages_count += 1;
        }
        Msg::SendBinaryMessage(msg) => {
            let serialized = rmp_serde::to_vec(&msg).unwrap();
            model.web_socket.send_bytes(&serialized).unwrap();
            model.input_binary.clear();
            model.sent_messages_count += 1;
        }
    }
}

fn create_websocket(orders: &impl Orders<Msg>) -> WebSocket {
    let msg_sender = orders.msg_sender();

    WebSocket::builder(WS_URL, orders)
        .on_open(|| Msg::WebSocketOpened)
        .on_message(move |msg| decode_message(msg, msg_sender))
        .on_close(Msg::WebSocketClosed)
        .on_error(|| Msg::WebSocketFailed)
        .build_and_open()
        .unwrap()
}

fn decode_message(message: WebSocketMessage, msg_sender: Rc<dyn Fn(Option<Msg>)>) {
    if message.contains_text() {
        let msg = message
            .json::<shared::ServerMessage>()
            .expect("Failed to decode WebSocket text message");

        msg_sender(Some(Msg::TextMessageReceived(msg)));
    } else {
        spawn_local(async move {
            let bytes = message
                .bytes()
                .await
                .expect("WebsocketError on binary data");

            let msg: shared::ServerMessage = rmp_serde::from_slice(&bytes).unwrap();
            msg_sender(Some(Msg::BinaryMessageReceived(msg)));
        });
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        h1!["WebSocket example"],
        div![model.messages.iter().map(|message| p![message])],
        hr![],
        if model.web_socket.state() == web_socket::State::Open {
            div![
                div![
                    p!["Message (text)"],
                    input![
                        id!("text"),
                        attrs! {
                            At::Type => "text",
                            At::Value => model.input_text;
                        },
                        input_ev(Ev::Input, Msg::InputTextChanged)
                    ],
                    button![
                        ev(Ev::Click, {
                            let message_text = model.input_text.to_owned();
                            move |_| Msg::SendMessage(shared::ClientMessage { text: message_text })
                        }),
                        "Send"
                    ],
                ],
                div![
                    p!["Message (binary)"],
                    input![
                        id!("binary"),
                        attrs! {
                            At::Type => "text",
                            At::Value => model.input_binary;
                        },
                        input_ev(Ev::Input, Msg::InputBinaryChanged)
                    ],
                    button![
                        ev(Ev::Click, {
                            let message_text = model.input_binary.to_owned();
                            move |_| {
                                Msg::SendBinaryMessage(shared::ClientMessage { text: message_text })
                            }
                        }),
                        "Send"
                    ],
                ],
                hr![style! {St::Margin => px(20) + " " + &px(0)}],
                button![
                    ev(Ev::Click, |_| Msg::CloseWebSocket),
                    "Close websocket connection"
                ],
            ]
        } else {
            div![p![em!["Connecting or closed"]]]
        },
        footer![
            p![format!("{} messages", model.messages.len())],
            p![format!("{} messages sent", model.sent_messages_count)]
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
