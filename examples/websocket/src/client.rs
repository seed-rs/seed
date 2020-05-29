use seed::{prelude::*, *};

mod shared;

const WS_URL: &str = "ws://127.0.0.1:9000/ws";

// ------ ------
//     Model
// ------ ------

struct Model {
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

fn create_websocket(orders: &impl Orders<Msg>) -> WebSocket {
    WebSocket::builder(WS_URL, orders)
        .on_open(|| Msg::WebSocketOpened)
        .on_message(Msg::MessageReceived)
        .on_close(Msg::WebSocketClosed)
        .on_error(|| Msg::WebSocketFailed)
        .build_and_open()
        .unwrap()
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    WebSocketOpened,
    MessageReceived(WebSocketMessage),
    BytesReceived(Vec<u8>),
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
        Msg::MessageReceived(message) => {
            log!("Client received a message");

            if message.contains_text() {
                model
                    .messages
                    .push(message.json::<shared::ServerMessage>().unwrap().text);
            } else {
                orders.perform_cmd(async move {
                    let bytes = message.bytes().await;
                    bytes.map(Msg::BytesReceived).ok()
                });
            }
        }
        Msg::BytesReceived(bytes) => {
            log!("Client received binary message");
            let msg: shared::ServerMessage = rmp_serde::from_slice(&bytes).unwrap();
            model.messages.push(msg.text);
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

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        h1!["WebSocket example"],
        div![model.messages.iter().map(|message| p![message])],
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
                div![button![
                    ev(Ev::Click, |_| Msg::CloseWebSocket),
                    "Close websocket connection"
                ],]
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
