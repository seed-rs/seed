//! A simple example demonstrating the usage of seed with WebSockets.

#[macro_use]
extern crate seed;
#[macro_use]
extern crate serde_derive;

use seed::{prelude::*, App};
use wasm_bindgen::JsCast;
use web_sys::{
    console::{log_1, log_2},
    BinaryType, MessageEvent, WebSocket,
};

mod json;

const WS_URL: &str = "ws://127.0.0.1:3030/ws";

#[derive(Clone, Default)]
struct Model {
    connected: bool,
    msg_rx_cnt: usize,
    msg_tx_cnt: usize,
    input_text: String,
    messages: Vec<String>,
}

#[derive(Clone)]
enum Msg {
    Connected,
    ServerMsg(json::ServerMsg),
    Send(json::ClientMsg),
    Sent,
    EditChange(String),
}

fn update(msg: Msg, mut model: Model) -> Update<Model> {
    match msg {
        Msg::Connected => {
            model.connected = true;
            Render(model)
        }
        Msg::ServerMsg(msg) => {
            model.connected = true;
            model.msg_rx_cnt += 1;
            model.messages.push(msg.text);
            Render(model)
        }
        Msg::EditChange(input_text) => Render(Model {
            input_text,
            ..model
        }),
        Msg::Send(_) => Skip(model),
        Msg::Sent => {
            model.input_text = "".into();
            model.msg_tx_cnt += 1;
            Render(model)
        }
    }
}

fn render_messages(msgs: &[String]) -> El<Msg> {
    let msgs: Vec<_> = msgs.iter().map(|m| p![m]).collect();
    div![msgs]
}

fn view(_: App<Msg, Model>, model: &Model) -> El<Msg> {
    div![
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
                        Msg::Send(json::ClientMsg {
                            text: model.input_text.clone()
                        })
                    ),
                    "Send"
                ]
            ]
        } else {
            div![p![em!["Connecting..."]]]
        },
        render_messages(&model.messages),
        footer![
            if model.connected {
                p!["Connected"]
            } else {
                p!["Disconnected"]
            },
            p![format!("{} messages received", model.msg_rx_cnt)],
            p![format!("{} messages sent", model.msg_tx_cnt)]
        ]
    ]
}

fn open_ws(state: App<Msg, Model>) {
    let ws = WebSocket::new(WS_URL).unwrap();
    ws.set_binary_type(BinaryType::Arraybuffer);

    let s = state.clone();
    let on_open = Closure::wrap(Box::new(move |_| {
        log_1(&"WebSocket connection is open now".into());
        s.update(Msg::Connected);
    }) as Box<FnMut(JsValue)>);

    let on_close = Closure::wrap(Box::new(|_| {
        log_1(&"WebSocket connection was closed".into());
    }) as Box<FnMut(JsValue)>);

    let s = state.clone();
    let on_message = Closure::wrap(Box::new(move |ev: MessageEvent| {
        log_1(&"Client received a message".into());
        let txt = ev.data().as_string().unwrap();
        let json: json::ServerMsg = serde_json::from_str(&txt).unwrap();
        log_2(&"text message:".into(), &txt.into());
        s.update(Msg::ServerMsg(json));
    }) as Box<FnMut(MessageEvent)>);

    let on_error = Closure::wrap(Box::new(|_| {
        log_1(&"err".into());
    }) as Box<FnMut(JsValue)>);

    ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();
    ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
    on_close.forget();
    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
    ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    on_error.forget();
    let app = state.clone();
    state.add_message_listener(move |msg| match msg {
        Msg::Send(msg) => {
            let s = serde_json::to_string(msg).unwrap();
            ws.send_with_str(&s).unwrap();
            app.update(Msg::Sent);
        }
        _ => {}
    });
}

#[wasm_bindgen]
pub fn start() {
    log_1(&"start the websocket client app".into());
    let app = App::build(Model::default(), update, view).finish().run();
    open_ws(app);
}
