## Pages example

How to create and browse multiple pages in your app.

A simple and predictable page navigation example pattern.

In this example the current page is a value from the `Page` enum and is stored in [Model.page](https://github.com/seed-rs/seed/blob/master/examples/pages/src/lib.rs#L31)

The `pages_keep_state` example is probably better suited to more complex websites or apps where you want to keep Model state across page loads.

---

```bash
cargo make start
```

Open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.
