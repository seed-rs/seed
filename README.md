[![Build Status](https://travis-ci.org/David-OConnor/seed.svg?branch=master)](https://travis-ci.org/David-OConnor/seed)
[![crates.io version](https://meritbadge.herokuapp.com/seed)](https://crates.io/crates/seed)
[![crates.io downloads](https://img.shields.io/crates/d/seed.svg)](https://crates.io/crates/seed)
[![docs.rs](https://docs.rs/seed/badge.svg)](https://docs.rs/seed)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

<p align="center">
  <img src="/seed_branding/seed_logo.svg" width="256" title="Seed logo">
</p>

# Quickstart

## Setup

This framework requires you to install [Rust](https://www.rust-lang.org/tools/install).

You'll need a recent version of Rust: `rustup update`

The wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

And cargo-make: `cargo install --force cargo-make`

## The theoretical minimum

To start, clone [the quickstart repo](https://github.com/David-OConnor/seed-quickstart):
`git clone https://github.com/david-oconnor/seed-quickstart.git`,
run `cargo make all` in a terminal to build the app, and `cargo make serve` to start a dev server
on `127.0.0.0:8000`.

## A little deeper

Alternatively, create a new lib with Cargo: `cargo new --lib appname`. Here and everywhere it appears in this guide, `appname` should be replaced with the name of your app.

If not using the quickstart repo, create an Html file with a body that contains this:

```html
<section id="app"></section>

<script src="/pkg/package.js"></script>

<script>
  const { render } = wasm_bindgen;
  function run() {
    render();
  }
  wasm_bindgen("/pkg/package_bg.wasm")
    .then(run)
    .catch(console.error);
</script>
```

The first line above is an empty element with id: It's where your app will render.
The subsequent ones load your app's wasm modules.

The quickstart repo includes this file. You will eventually need to modify it to
change the page's title, add a description, favicon, stylesheet etc.

`Cargo.toml`, which is a file created by Cargo that describes your app, needs `wasm-bindgen`, `web-sys`, and `seed` as depdendencies,
and crate-type
of `"cdylib"`. The version in the quickstart repo has these set up already. Example:

```toml
[package]
name = "appname"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
seed = "^0.2.4"
wasm-bindgen = "^0.2.38"
web-sys = "^0.3.6"
```

## A short example

Here's an example demonstrating structure and syntax; it can be found in working form
in the [counter example](https://github.com/David-OConnor/seed/tree/master/examples/counter)
Descriptions of its parts are in the
Guide section below. Its structure follows [The Elm Architecture](https://guide.elm-lang.org/architecture/).

_lib.rs_:

```rust
#[macro_use]
extern crate seed;
use seed::prelude::*;


// Model

struct Model {
    count: i32,
    what_we_count: String
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into()
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeWWC(String),
}

/// The sole source of updating the model
fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
        Msg::ChangeWWC(what_we_count) => model.what_we_count = what_we_count,
    }
    Render.into()
}


// View

/// A simple component.
fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0 ... 5 => "Not very many 🙁",
        6 ... 9 => "I got my first real six-string 😐",
        10 ... 11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃"
    };
    p![ descrip ]
}

/// The top-level component we pass to the virtual dom.
fn view(model: &Model) -> El<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

    div![ outer_style,
        h1![ "The Grand Total" ],
        div![
            style!{
                // Example of conditional logic in a style.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                // When passing numerical values to style!, "px" is implied.
                "border" => "2px solid #004422"; "padding" => 20
            },
            // We can use normal Rust code and comments in the view.
            h3![ format!("{} {}{} so far", model.count, model.what_we_count, plural) ],
            button![ simple_ev(Ev::Click, Msg::Increment), "+" ],
            button![ simple_ev(Ev::Click, Msg::Decrement), "-" ],

            // Optionally-displaying an element
            if model.count >= 10 { h2![ style!{"padding" => 50}, "Nice!" ] } else { seed::empty() }
        ],
        success_level(model.count),  // Incorporating a separate component

        h3![ "What precisely is it we're counting?" ],
        input![ attrs!{At::Value => model.what_we_count}, input_ev(Ev::Input, Msg::ChangeWWC) ]
    ]
}


#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}
```

For a truly minimimal example, see [lib.rs in the quickstart repo](https://github.com/David-OConnor/seed-quickstart/blob/master/src/lib.rs)

## Building and running

To build your app run `cargo make all`, and to host on a dev server, run `cargo make serve`.

For a more robust starting setup, check out Martin Kavik's [seed-quickstart-webpack repo](https://github.com/MartinKavik/seed-quickstart-webpack).

## Running included examples

To run an example located in the [examples folder](https://github.com/David-OConnor/seed/tree/master/examples),
run `cargo make start example_name`, where you replace `example_name` with the example name. Eg:
`cargo make start counter`.

Some examples also require to run API server in another terminal window - `cargo make start_server example_name`.

When server(s) are running, open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.

# About

## Goals

- Learning the syntax, creating a project, and building it should be easy - regardless
  of your familiarity with Rust.

- Complete documentation that always matches the current version. Getting examples working, and
  starting a project should be painless, and require nothing beyond this guide.

- Expressive, flexible view syntax that's easy to read and write.

## A note on view syntax

This project uses an unconventional approach to describe how to display DOM elements.
It neither uses completely natural (ie macro-free) Rust code, nor
an HTML-like abstraction (eg JSX or templates). My intent is to make the code close
to natural Rust, while streamlining the syntax in a way suited for creating
a visual layout with minimal repetition. The macros used are thin wrappers
for constructors, and don't conceal much. Specifically, the element-creation macros
allow for accepting a variable number of parameters, and the attrs/style marcros are
essentially HashMap literals, with wrappers that let element macros know how to distinguish
them.

The lack of resemblance to HTML be offputting, but the learning
curve is shallow, and I think the macro syntax is close-enough to normal Rust that it's
easy to reason about how to build views, without compartmentalizing it into logic code and display code.
This lack of separation in particular is a controversial decision, but I think the benefits
are worth it.

## Where to start if you're familiar with existing frontend frameworks

The [todomvc example](https://github.com/David-OConnor/seed/tree/master/examples/todomvc) is an implementation of the [TodoMVC project](http://todomvc.com/),
which has example code in other frameworks that produce identitcal apps. Compare the example in this
project to one on that page that uses a framework you're familiar with.

## Influences

This project is strongly influenced by Elm, React, and Redux. The overall structure
of Seed apps mimicks that of The Elm Architecture.

## There are already several Rust/WASM frameworks; why add another?

I'm distinguishing Seed through clear examples and documentation, and using `wasm-bindgen`/`web-sys` internally. I started this
project after being unable to get existing frameworks working
due to lack of documented examples, and inconsistency between documentation and
published versions. My intent is for anyone who's proficient in a frontend
framework to get a standalone app working in the browser within a few minutes, using just the
quickstart guide.

Seed's different approach to view syntax also distinguishes it:
rather than use an HTML-like markup similar to JSX,
it uses Rust builtin types, thinly-wrapped by macros that allow flexible composition.
This decision will not appeal to everyone, but I think it integrates more naturally with
the language.

## Why build a frontend in Rust over Elm, or Javascript-based frameworks?

You may prefer writing in Rust, and using packages from Cargo vice npm. Getting started with
this framework will in most cases be easier, and require less config and setup overhead than
with JS frameworks. You may appreciate Rust's compile-time error-checking, and built-in testing.

You may choose this approach over `Elm if you're already comfortable with Rust,
or don't want to code business logic in a purely-functional langauge.

Compared with React, you may appreciate the consistency of how to write apps:
There's no distinction between logic and display code; no restrictions on comments;
no distinction between components and normal functions. The API is
flexible, and avoids OOP boilerplate. Its integrated routing and message system
avoids the dependency glue-code associated with Redux and React-Router.

Seed has a _batteries-included_ approach, which you may appreciate.

## Why not to use this, and stick with JS

Seed's under rapid development, and breaking changes are likely. Finding Rust/WASM-help,
both in person, and in online communities will be difficult, and finding help for Seed
even more so. Seed doesn't have the wealth of existing reusable _components_ that other frameworks
have, so you will need to implement solved problems (eg date-pickers) yourself, or adapt them
from existing solutions. There are no existing tutorials or guides outside the official one, and
few examples.

Seed doesn't have a track-record of production apps. Finding developers experienced with Rust/wasm-bindgen,
or Seed specifically will be much more difficult than popular JS/compile-to-JS frameworks. Seed's feature-set
is incomplete compared to JS frameworks. Seed hasn't been benchmarked, and its performance may
be lower than JS frameworks.

Seed's view syntax is non-standard compared to HTML-templates, or HTML-mockup languages like
`JSX`.

## What about Gloo ?

We're working closely with the `rustwasm` team on [Gloo](https://github.com/rustwasm/gloo), and
intend to incorporate `Gloo` crates into Seed as appropriate, as well as contribute Seed
code into `Gloo` crates. Seed's a cohesive, high-level framework, while `Gloo` will
be a versatile, standardized toolkit.

### Shoutouts

- The [WASM-Bindgen](https://github.com/rustwasm/wasm-bindgen) team,
  for building the tools this project relies on
- Alex Chrichton, for being extraodinarily helpful in the Rust / WASM community
- The [Elm](https://elm-lang.org/) team, for creating and standardizing the Elm architecture
- Mozilla, for excellent DOM documentation
- Denis Kolodin, for creating the inspirational [Yew framework](https://github.com/DenisKolodin/yew)
- Utkarsh Kukreti, for through his [Draco repo](https://github.com/utkarshkukreti/draco),
  helping me understand how wasm-bindgen's
  closure system can be used to update state.
- Tim Robinson, for being very helpful on the [Rust Gitter](https://gitter.im/rust-lang/rust).

## Reference

- [wasm-bindgen guide](https://rustwasm.github.io/wasm-bindgen/introduction.html)
- [Mozilla MDN web docs](https://developer.mozilla.org/en-US/)
- [web-sys api](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) (A good partner for the MDN docs - most DOM items have web-sys equivalents used internally)
- [Rust book](https://doc.rust-lang.org/book/index.html)
- [Rust standard library api](https://doc.rust-lang.org/std/)
- [Seed's API docs](https://docs.rs/seed)
- [Learn Rust](https://www.rust-lang.org/learn)
- [Testing in Headless Browsers](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/browsers.html)
