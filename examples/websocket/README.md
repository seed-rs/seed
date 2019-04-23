## Websocket example

Example of communicating with a server using Websockets - simple chat.

- Using web-sys's Websocket in client.
- Serde for [de]serializiation.
- [WS-RS (ws)](https://ws-rs.org/) as a websocket server.
- Demonstrates sending messages and receiving messages with sender id (see console or server logs).
- There is not workspace - client and server dependencies are resolved by `features`, see `Cargo.toml` and `Makefile.toml`. 

---

```bash
cargo make start
```
Open a new terminal window.
```bash
cargo make start_server
```

Open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.