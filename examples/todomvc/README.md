# Rust & Seed TodoMVC Example


> Rust is a systems programming language with a focus on safety, 
especially safe concurrency.

> _[Rust](https://www.rust-lang.org)_

>  wasm-bindgen, and its web-sys package allow Rust to be used in web browsers via WASM.

>  [Wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)

>  Seed is a high-level framework for building websites using these tools.

> _[Seed](https://github.com/seed-rs/seed)_

## Learning Rust

The [Rust book](https://doc.rust-lang.org/book/index.html) is a great resource for getting started.

Here are some links you may find helpful:

* [Code Playground](https://play.rust-lang.org/)
* [Rust Documentation](https://doc.rust-lang.org/)
* [Rust Source Code](https://github.com/rust-lang/rust)
* [wasm-bindgen Source Code](https://github.com/rustwasm/wasm-bindgen)
* [Seed guide](https://github.com/seed-rs/seed)
* [Seed quickstart repo](https://github.com/seed-rs/seed-quickstart)

Get help from Rust users:

* [Rust on StackOverflow](http://stackoverflow.com/questions/tagged/rust)
* [Reddit](https://www.reddit.com/r/rust/)
* [Gitter chat](https://gitter.im/rust-lang/rust)

_If you have other helpful links to share, or find any of the links above no longer work, please [let us know](https://github.com/tastejs/todomvc/issues)._


## Running

#### Prerequisites

- This framework requires you to first install [Rust](https://www.rust-lang.org/tools/install).
- You'll need a recent version of Rust: `rustup update`
- The wasm32 target: `rustup target add wasm32-unknown-unknown`
- And cargo-make: `cargo install --force cargo-make`


#### Build & Run
```bash
cargo make start
```
    
Open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.

---

### [How to make this example standalone]
- **`Makefile.toml`** 
    - Replace tasks with aliases with their parents.
    - Remove root Makefile import.
- **`Cargo.toml`**
    - replace Seed path with version number
- This file
    - Remove this chapter
