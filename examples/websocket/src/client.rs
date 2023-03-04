use seed::{prelude::*, *};
use std::rc::Rc;
use wasm_sockets::{self, ConnectionStatus, EventClient, Message, WebSocketError};

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
    web_socket: EventClient,
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
        web_socket: create_websocket(orders).unwrap(),
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
            model.web_socket.close().unwrap();
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
            model.web_socket = create_websocket(orders).unwrap();
        }
        Msg::InputTextChanged(text) => {
            model.input_text = text;
        }
        Msg::InputBinaryChanged(text) => {
            model.input_binary = text;
        }
        Msg::SendMessage(msg) => {
            let txt = serde_json::to_string(&msg).unwrap();
            model.web_socket.send_string(&txt).unwrap();
            model.input_text.clear();
            model.sent_messages_count += 1;
        }
        Msg::SendBinaryMessage(msg) => {
            let serialized = rmp_serde::to_vec(&msg).unwrap();
            model.web_socket.send_binary(serialized).unwrap();
            model.input_binary.clear();
            model.sent_messages_count += 1;
        }
    }
}

fn create_websocket(orders: &impl Orders<Msg>) -> Result<EventClient, WebSocketError> {
    let msg_sender = orders.msg_sender();

    let mut client = EventClient::new(WS_URL)?;

    client.set_on_error(Some(Box::new(|error| {
        error!("WS: {:#?}", error);
    })));

    let send = msg_sender.clone();
    client.set_on_connection(Some(Box::new(move |client: &EventClient| {
        log!("{:#?}", client.status);
        let msg = match *client.status.borrow() {
            ConnectionStatus::Connecting => {
                log!("Connecting...");
                None
            }
            ConnectionStatus::Connected => Some(Msg::WebSocketOpened),
            ConnectionStatus::Error => Some(Msg::WebSocketFailed),
            ConnectionStatus::Disconnected => {
                log!("Disconnected");
                None
            }
        };
        send(msg);
    })));

    let send = msg_sender.clone();
    client.set_on_close(Some(Box::new(move |ev| {
        log!("WS: Connection closed");
        send(Some(Msg::WebSocketClosed(ev)));
    })));

    let send = msg_sender.clone();
    client.set_on_message(Some(Box::new(
        move |_: &EventClient, msg: wasm_sockets::Message| decode_message(msg, Rc::clone(&send)),
    )));

    Ok(client)
}

fn decode_message(message: Message, msg_sender: Rc<dyn Fn(Option<Msg>)>) {
    match message {
        Message::Text(txt) => {
            let msg = serde_json::from_str::<shared::ServerMessage>(&txt)
                .expect("Failed to decode WebSocket text message");
            msg_sender(Some(Msg::TextMessageReceived(msg)));
        }
        Message::Binary(bytes) => {
            let msg: shared::ServerMessage = rmp_serde::from_slice(&bytes).unwrap();
            msg_sender(Some(Msg::BinaryMessageReceived(msg)));
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
        hr![],
        if *model.web_socket.status.borrow() == ConnectionStatus::Connected {
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
                            let message_text = model.input_text.clone();
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
                            let message_text = model.input_binary.clone();
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
