# Rust & Seed TodoMVC Example


> Rust is a systems programming language with a focus on safety, 
especially safe concurrency.

> _[Rust](https://www.rust-lang.org)_

>  wasm-bindgen, and its web-sys package allow Rust to be used in web browsers via WASM.

>  [Wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)

>  Seed is a high-level framework for building websites using these tools.

> _[Seed](https://github.com/David-OConnor/seed)_

## Learning Rust

The [Rust book](https://doc.rust-lang.org/book/index.html) is a great resource for getting started.

Here are some links you may find helpful:

* [Code Playground](https://play.rust-lang.org/)
* [Rust Documentation](https://doc.rust-lang.org/)
* [Rust Source Code](https://github.com/rust-lang/rust)
* [wasm-bindgen Source Code](https://github.com/rustwasm/wasm-bindgen)
* [Seed guide](https://github.com/David-OConnor/seed)
* [Seed quickstart repo](https://github.com/David-OConnor/seed-quickstart)

Get help from Rust users:

* [Rust on StackOverflow](http://stackoverflow.com/questions/tagged/rust)
* [Reddit](https://www.reddit.com/r/rust/)
* [Gitter chat](https://gitter.im/rust-lang/rust)

_If you have other helpful links to share, or find any of the links above no longer work, please [let us know](https://github.com/tastejs/todomvc/issues)._


## Running

This framework requires you to first install [Rust](https://www.rust-lang.org/tools/install).

You'll need a recent version of Rust: `rustup update`

The wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

And wasm-bindgen: `cargo install wasm-bindgen-cli`

If you run into errors while installing `wasm-bindgen-cli`, you may need to install C++
build tools. On linux, run `sudo apt install build-essential`. On Windows, download and install
[Visual Studio 2017](https://visualstudio.microsoft.com/downloads/); when asked in the installer,
include the C++ workload.

Build using one of the two following commands:

    build.sh
or

    build.ps1
    
Open index.html in a web browser, or with a dev server, eg

    python serve.py