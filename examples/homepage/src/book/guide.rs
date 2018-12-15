
pub fn text() -> String {
"
# Seed

**A Rust framework for creating web apps**

[![](https://meritbadge.herokuapp.com/seed)](https://crates.io/crates/seed)
[![](https://img.shields.io/crates/d/seed.svg)](https://crates.io/crates/seed)
[![API Documentation on docs.rs](https://docs.rs/seed/badge.svg)](https://docs.rs/seed)


## Quickstart

### Setup
This framework requires you to install [Rust](https://www.rust-lang.org/tools/install) - This will
enable the CLI commands below:

 You'll need a recent version of Rust: `rustup update`

The wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

And wasm-bindgen: `cargo install wasm-bindgen-cli`


### The theoretical minimum
To start, clone [This quickstart repo](https://github.com/David-OConnor/seed-quickstart),
run `build.sh` or `build.ps1` in a terminal, then start a dev server that supports WASM.
For example, with [Python](https://www.python.org/downloads/) installed, run `python server.py`.
(Linux users may need to run `python3 server.py`.)
Once you change your package name, you'll
need to tweak the Html file and build script, as described below.
".into()
}