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
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let web_socket = WebSocket::builder(WS_URL, orders)
        .on_open(|| log!("WebSocket connection is open now"))
        .on_message(Msg::MessageReceived)
        .on_close(Msg::WebSocketClosed)
        .on_error(|| log!("Error"))
        .build_and_open()
        .unwrap();

    Model {
        sent_messages_count: 0,
        messages: Vec::new(),
        input_text: String::new(),
        web_socket,
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    MessageReceived(WebSocketMessage),
    CloseWebSocket,
    WebSocketClosed(CloseEvent),
    InputTextChanged(String),
    SendMessage(shared::ClientMessage),
}

fn update(msg: Msg, mut model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::MessageReceived(message) => {
            log!("Client received a message");
            model
                .messages
                .push(message.json::<shared::ServerMessage>().unwrap().text);
        }
        Msg::CloseWebSocket => {
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
