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
    CloseWebSocket,
    WebSocketClosed(CloseEvent),
    WebSocketFailed,
    ReconnectWebSocket(usize),
    InputTextChanged(String),
    SendMessage(shared::ClientMessage),
}

fn update(msg: Msg, mut model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WebSocketOpened => {
            model.web_socket_reconnector = None;
            log!("WebSocket connection is open now");
        }
        Msg::MessageReceived(message) => {
            log!("Client received a message");
            model
                .messages
                .push(message.json::<shared::ServerMessage>().unwrap().text);
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
        Msg::InputTextChanged(input_text) => {
            model.input_text = input_text;
        }
        Msg::SendMessage(msg) => {
            model.web_socket.send_json(&msg).unwrap();
            model.input_text.clear();
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
                button![ev(Ev::Click, |_| Msg::CloseWebSocket), "Close"],
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
