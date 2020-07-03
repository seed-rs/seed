### JWT

How to implement user sessions with JSON web tokens.

This example is split into two parts that have to be run simultaneously.

Our server-side is written using [`tide`]("https://github.com/http-rs/tide") and provides JWT as cookies.

Our Seed client-side queries our server to find out if a user is signed in.

---

```bash
cargo make build
```
In one terminal 
```bash
cargo make start_server
```
And in another terminal
```bash
cargo make start
```


Open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.
