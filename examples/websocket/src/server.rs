use ws::{listen, Handler, Message, Request, Response, Result, Sender};

mod json;

// Server web application handler
struct Server {
    out: Sender,
}

impl Handler for Server {
    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let client_id: usize = self.out.token().into();

        let client_msg: json::ClientMessage =
            serde_json::from_str(&msg.into_text().unwrap()).unwrap();

        println!(
            "Server received text: '{}'\nfrom client '{}'\n",
            client_msg.text, client_id
        );

        let server_msg: Message = serde_json::to_string(&json::ServerMessage {
            id: client_id,
            text: client_msg.text,
        })
        .unwrap()
        .into();
        // Broadcast to all connections
        self.out.broadcast(server_msg)
    }

    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            _ => Ok(Response::new(
                200,
                "OK",
                b"Websocket server is running".to_vec(),
            )),
        }
    }
}

fn main() {
    // Listen on an address and call the closure for each connection
    listen("127.0.0.1:9000", |out| Server { out }).unwrap()
}
