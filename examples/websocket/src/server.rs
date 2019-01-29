#[macro_use]
extern crate serde_derive;

use futures::{sync::mpsc, Future, Stream};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use warp::{
    http::Response,
    ws::{Message, WebSocket},
    Filter,
};

mod json;

// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

static INDEX_HTML: &str = include_str!("../index.html");
static WS_JS: &str = include_str!("../pkg/websocket.js");
static WS_WASM: &[u8] = include_bytes!("../pkg/websocket_bg.wasm");

// Our state of currently connected users.
//
// - Key is their id
// - Value is a sender of `warp::ws::Message`
type Users = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

fn main() {
    pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let users = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());

    // GET /chat -> websocket upgrade
    let chat = warp::path("ws")
        // The `ws2()` filter will prepare Websocket handshake...
        .and(warp::ws2())
        .and(users)
        .map(|ws: warp::ws::Ws2, users| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, users))
        });

    // GET / -> index.html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    // GET /websocket.js
    let ws_js = warp::path("websocket.js").map(|| {
        Response::builder()
            .header("Content-Type", "application/javascript")
            .body(WS_JS)
    });

    // GET /websocket_bg.wasm
    let ws_wasm = warp::path("websocket_bg.wasm").map(|| {
        Response::builder()
            .header("Content-Type", "application/wasm")
            .body(WS_WASM)
    });

    let routes = index.or(ws_js).or(ws_wasm).or(chat);

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}

fn user_connected(ws: WebSocket, users: Users) -> impl Future<Item = (), Error = ()> {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded();
    warp::spawn(
        rx.map_err(|()| -> warp::Error { unreachable!("unbounded rx never errors") })
            .forward(user_ws_tx)
            .map(|_tx_rx| ())
            .map_err(|ws_err| eprintln!("websocket send error: {}", ws_err)),
    );

    // Save the sender in our list of connected users.
    users.lock().unwrap().insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Make an extra clone to give to our disconnection handler...
    let users2 = users.clone();

    let m = json::ClientMsg {
        text: "::connected::".into(),
    };
    user_message(
        my_id,
        Message::text(serde_json::to_string(&m).unwrap()),
        &users,
    );

    user_ws_rx
        // Every time the user sends a message, broadcast it to
        // all other users...
        .for_each(move |msg| {
            user_message(my_id, msg, &users);
            Ok(())
        })
        // for_each will keep processing as long as the user stays
        // connected. Once they disconnect, then...
        .then(move |result| {
            user_disconnected(my_id, &users2);
            result
        })
        // If at any time, there was a websocket error, log here...
        .map_err(move |e| {
            eprintln!("websocket error(uid={}): {}", my_id, e);
        })
}

fn user_message(my_id: usize, msg: Message, users: &Users) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };
    let msg: json::ClientMsg = serde_json::from_str(msg).unwrap();

    let new_msg = json::ServerMsg {
        id: my_id,
        text: format!("<User#{}>: {}", my_id, msg.text),
    };

    let new_msg = serde_json::to_string(&new_msg).unwrap();

    // New message from this user, send it to everyone else (except same uid)...
    //
    // We use `retain` instead of a for loop so that we can reap any user that
    // appears to have disconnected.
    for (&uid, tx) in users.lock().unwrap().iter() {
        if my_id != uid {
            match tx.unbounded_send(Message::text(new_msg.clone())) {
                Ok(()) => (),
                Err(_disconnected) => {
                    // The tx is disconnected, our `user_disconnected` code
                    // should be happening in another task, nothing more to
                    // do here.
                }
            }
        }
    }
}

fn user_disconnected(my_id: usize, users: &Users) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.lock().unwrap().remove(&my_id);
}
