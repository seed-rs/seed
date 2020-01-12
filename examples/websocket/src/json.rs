use serde::{Deserialize, Serialize};

/// Message from the server to the client.
#[derive(Serialize, Deserialize)]
pub struct ServerMessage {
    pub id: usize,
    pub text: String,
}

/// Message from the client to the server.
#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    pub text: String,
}
