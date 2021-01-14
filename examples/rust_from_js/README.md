## Rust from JS example

How to call Rust functions from Javascript.

_Note:_ See example `update_from_js` for more advanced example without the mutable global variable as a state. 

---

```bash
cargo make start
```

Open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.

See the code in `index.html`. 

Run in the browser dev console `rust.set_title("New title")` and click the `Rerender` button.
