/// Message from the server to the client.
#[derive(Clone, Serialize, Deserialize)]
pub struct ServerMsg {
    pub id: usize,
    pub text: String,
}

/// Message from the client to the server.
#[derive(Clone, Serialize, Deserialize)]
pub struct ClientMsg {
    pub text: String,
}
