use js_sys::Function;
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

mod json;

const WS_URL: &str = "ws://127.0.0.1:9000/ws";

// ------ ------
//     Model
// ------ ------

struct Model {
    data: Data,
    services: Services,
}

#[derive(Default)]
struct Data {
    connected: bool,
    msg_rx_cnt: usize,
    msg_tx_cnt: usize,
    input_text: String,
    messages: Vec<String>,
}

struct Services {
    ws: WebSocket,
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    let ws = WebSocket::new(WS_URL).unwrap();

    register_ws_handler(WebSocket::set_onopen, Msg::Connected, &ws, orders);
    register_ws_handler(WebSocket::set_onclose, Msg::Closed, &ws, orders);
    register_ws_handler(WebSocket::set_onmessage, Msg::ServerMessage, &ws, orders);
    register_ws_handler(WebSocket::set_onerror, Msg::Error, &ws, orders);

    AfterMount::new(Model {
        data: Data::default(),
        services: Services { ws },
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

// ------ ------
//    Update
// ------ ------

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
            model.data.connected = true;
        }
        Msg::ServerMessage(msg_event) => {
            log!("Client received a message");
            let txt = msg_event.data().as_string().unwrap();
            let json: json::ServerMessage = serde_json::from_str(&txt).unwrap();

            model.data.msg_rx_cnt += 1;
            model.data.messages.push(json.text);
        }
        Msg::EditChange(input_text) => {
            model.data.input_text = input_text;
        }
        Msg::Send(msg) => {
            let s = serde_json::to_string(&msg).unwrap();
            model.services.ws.send_with_str(&s).unwrap();
            orders.send_msg(Msg::Sent);
        }
        Msg::Sent => {
            model.data.input_text = "".into();
            model.data.msg_tx_cnt += 1;
        }
        Msg::Closed(_) => {
            log!("WebSocket connection was closed");
        }
        Msg::Error(_) => {
            log!("Error");
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    let data = &model.data;

    vec![
        h1!["seed websocket example"],
        if data.connected {
            div![
                input![
                    id!("text"),
                    attrs! {
                        At::Type => "text",
                        At::Value => data.input_text;
                    },
                    input_ev(Ev::Input, Msg::EditChange)
                ],
                button![
                    id!("send"),
                    attrs! { At::Type => "button" },
                    ev(Ev::Click, {
                        let message_text = data.input_text.to_owned();
                        move |_| Msg::Send(json::ClientMessage { text: message_text })
                    }),
                    "Send"
                ]
            ]
        } else {
            div![p![em!["Connecting..."]]]
        },
        div![data.messages.iter().map(|message| p![message])],
        footer![
            if data.connected {
                p!["Connected"]
            } else {
                p!["Disconnected"]
            },
            p![format!("{} messages received", data.msg_rx_cnt)],
            p![format!("{} messages sent", data.msg_tx_cnt)]
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
