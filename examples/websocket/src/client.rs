#[macro_use]
extern crate seed;

use js_sys::Function;
use seed::{prelude::*, App};
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

mod json;

const WS_URL: &str = "ws://127.0.0.1:9000/ws";

// Model

struct Model {
    ws: WebSocket,
    connected: bool,
    msg_rx_cnt: usize,
    msg_tx_cnt: usize,
    input_text: String,
    messages: Vec<String>,
}

// Init

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Init<Model> {
    let ws = WebSocket::new(WS_URL).unwrap();

    register_ws_handler(WebSocket::set_onopen, Msg::Connected, &ws, orders);
    register_ws_handler(WebSocket::set_onclose, Msg::Closed, &ws, orders);
    register_ws_handler(WebSocket::set_onmessage, Msg::ServerMessage, &ws, orders);
    register_ws_handler(WebSocket::set_onerror, Msg::Error, &ws, orders);

    Init::new(Model {
        ws,
        connected: false,
        msg_rx_cnt: 0,
        msg_tx_cnt: 0,
        input_text: "".into(),
        messages: vec![],
    })
}

fn register_ws_handler<T, F>(
    ws_cb_setter: fn(&WebSocket, Option<&Function>),
    msg: F,
    ws: &WebSocket,
    orders: &mut impl Orders<Msg>,
) where
    T: wasm_bindgen::convert::FromWasmAbi + 'static,
    F: Fn(T) -> Msg + 'static,
{
    let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());

    let closure = Closure::new(move |data| {
        app.update(msg_mapper(msg(data)));
    });

    ws_cb_setter(ws, Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

// Update

#[derive(Clone)]
enum Msg {
    Connected(JsValue),
    ServerMessage(MessageEvent),
    Send(json::ClientMessage),
    Sent,
    EditChange(String),
    Closed(JsValue),
    Error(JsValue),
}

fn update(msg: Msg, mut model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Connected(_) => {
            log!("WebSocket connection is open now");
            model.connected = true;
        }
        Msg::ServerMessage(msg_event) => {
            log!("Client received a message");
            let txt = msg_event.data().as_string().unwrap();
            let json: json::ServerMessage = serde_json::from_str(&txt).unwrap();

            model.msg_rx_cnt += 1;
            model.messages.push(json.text);
        }
        Msg::EditChange(input_text) => {
            model.input_text = input_text;
        }
        Msg::Send(msg) => {
            let s = serde_json::to_string(&msg).unwrap();
            model.ws.send_with_str(&s).unwrap();
            orders.send_msg(Msg::Sent);
        }
        Msg::Sent => {
            model.input_text = "".into();
            model.msg_tx_cnt += 1;
        }
        Msg::Closed(_) => {
            log!("WebSocket connection was closed");
        }
        Msg::Error(_) => {
            log!("Error");
        }
    }
}

// View

fn view(model: &Model) -> impl View<Msg> {
    vec![
        h1!["seed websocket example"],
        if model.connected {
            div![
                input![
                    attrs! {
                        "type"=>"text";
                        "id"=>"text";
                        At::Value => model.input_text;
                    },
                    input_ev(Ev::Input, Msg::EditChange)
                ],
                button![
                    attrs! {"type"=>"button";"id"=>"send"},
                    simple_ev(
                        "click",
                        Msg::Send(json::ClientMessage {
                            text: model.input_text.clone()
                        })
                    ),
                    "Send"
                ]
            ]
        } else {
            div![p![em!["Connecting..."]]]
        },
        div![model.messages.iter().map(|m| p![m])],
        footer![
            if model.connected {
                p!["Connected"]
            } else {
                p!["Disconnected"]
            },
            p![format!("{} messages received", model.msg_rx_cnt)],
            p![format!("{} messages sent", model.msg_tx_cnt)]
        ],
    ]
}

// Start

#[wasm_bindgen(start)]
pub fn start() {
    App::build(init, update, view).finish().run();
}
