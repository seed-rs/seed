#[macro_use]
extern crate seed;

use seed::{prelude::*, App};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

mod json;

const WS_URL: &str = "ws://127.0.0.1:9000/ws";

#[derive(Clone, Default)]
struct Model {
    connected: bool,
    msg_rx_cnt: usize,
    msg_tx_cnt: usize,
    input_text: String,
    messages: Vec<String>,
}

// `Serialize` is required by `seed::update(..)`
// `Deserialize` is required by `trigger_update_handler`
#[derive(Clone, Serialize, Deserialize)]
enum Msg {
    Connected,
    ServerMessage(json::ServerMessage),
    Send(json::ClientMessage),
    Sent,
    EditChange(String),
}

fn update(msg: Msg, mut model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Connected => {
            model.connected = true;
        }
        Msg::ServerMessage(msg) => {
            model.connected = true;
            model.msg_rx_cnt += 1;
            model.messages.push(msg.text);
        }
        Msg::EditChange(input_text) => {
            model.input_text = input_text;
        }
        Msg::Send(_) => {
            orders.skip();
        }
        Msg::Sent => {
            model.input_text = "".into();
            model.msg_tx_cnt += 1;
        }
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
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

#[wasm_bindgen(start)]
pub fn start() {
    let app = App::build(|_,_| Model::default(), update, view)
        // `trigger_update_handler` is necessary,
        // because we want to process `seed::update(..)` calls.
        .window_events(|_| vec![trigger_update_handler()])
        .finish()
        .run();

    let ws = WebSocket::new(WS_URL).unwrap();
    register_handlers(&ws);
    register_message_listener(ws, &app)
}

fn register_handlers(ws: &web_sys::WebSocket) {
    register_handler_on_open(ws);
    register_handler_on_message(ws);
    register_handler_on_close(ws);
    register_handler_on_error(ws);
}

fn register_message_listener<ElC>(ws: web_sys::WebSocket, app: &App<Msg, Model, ElC>)
where
    ElC: View<Msg> + 'static,
{
    app.add_message_listener(move |msg| {
        if let Msg::Send(msg) = msg {
            let s = serde_json::to_string(msg).unwrap();
            ws.send_with_str(&s).unwrap();
            seed::update(Msg::Sent);
        }
    });
}

// ------ HANDLERS -------

fn register_handler_on_open(ws: &web_sys::WebSocket) {
    let on_open = Closure::new(move |_: JsValue| {
        log!("WebSocket connection is open now");
        seed::update(Msg::Connected);
    });

    ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();
}

fn register_handler_on_close(ws: &web_sys::WebSocket) {
    let on_close = Closure::new(|_: JsValue| {
        log!("WebSocket connection was closed");
    });

    ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
    on_close.forget();
}

fn register_handler_on_message(ws: &web_sys::WebSocket) {
    let on_message = Closure::new(move |ev: MessageEvent| {
        log!("Client received a message");
        let txt = ev.data().as_string().unwrap();
        let json: json::ServerMessage = serde_json::from_str(&txt).unwrap();
        log!("- text message: ", &txt);
        seed::update(Msg::ServerMessage(json));
    });

    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
}

fn register_handler_on_error(ws: &web_sys::WebSocket) {
    let on_error = Closure::new(|_: JsValue| {
        log!("Error");
    });

    ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    on_error.forget();
}
