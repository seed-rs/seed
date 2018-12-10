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

Articles and guides from the community:

* [Thoughts on TypeScript](http://www.nczonline.net/blog/2012/10/04/thoughts-on-typescript)
* [ScreenCast - Why I Like TypeScript](https://www.youtube.com/watch?v=Mh5VQVfWTbs)

Get help from other TypeScript users:

* [TypeScript on StackOverflow](http://stackoverflow.com/questions/tagged/rust)
* [Reddit](https://www.reddit.com/r/rust/)
* [Gitter chat](https://gitter.im/rust-lang/rust)

_If you have other helpful links to share, or find any of the links above no longer work, please [let us know](https://github.com/tastejs/todomvc/issues)._


## Running

This package requires you to install [Rust](https://www.rust-lang.org/en-US/) - This will
enable the CLI commands below:

 You'll need a recent version of Rust's nightly toolchain:

    rustup update
    rustup default nightly

The wasm32-unknown-unknown target:

    rustup target add wasm32-unknown-unknown --toolchain nightly

And wasm-bindgen: 

    Cargo +nightly install wasm-bindgen-cli

Build using one of the two following commands:

    build.sh
or

    build.ps1
    
Open index.html in a web browser, or with a dev server, eg

    python -m http.server