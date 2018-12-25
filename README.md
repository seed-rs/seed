# Seed

**A Rust framework for creating web apps**

[![](https://meritbadge.herokuapp.com/seed)](https://crates.io/crates/seed)
[![](https://img.shields.io/crates/d/seed.svg)](https://crates.io/crates/seed)
[![API Documentation on docs.rs](https://docs.rs/seed/badge.svg)](https://docs.rs/seed)

The best place to learn is the [guide](https://seed-rs.org) - this readme is an excerpt from it.

## Quickstart

## Setup
This framework requires you to install [Rust](https://www.rust-lang.org/tools/install) - This will
enable the CLI commands below:

 You'll need a recent version of Rust: `rustup update`

The wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

And wasm-bindgen: `cargo install wasm-bindgen-cli`


## The theoretical minimum
To start, clone [This quickstart repo](https://github.com/David-OConnor/seed-quickstart),
run `build.sh` or `build.ps1` in a terminal, then start a dev server that supports WASM.
For example, with [Python](https://www.python.org/downloads/) installed, run `python serve.py`.
(Linux users may need to run `python3 serve.py`.)
Once you change your package name, you'll
need to tweak the html file and build script, as described below.


## A little deeper
Or, create a new lib with Cargo: `cargo new --lib appname`. Here and everywhere it appears in this guide, `
appname` should be replaced with the name of your app.

If not using the quickstart repo, create an Html file that loads your app's compiled module, 
and provides an element with id 
to load the framework into. It also needs the following code to load your WASM module -
 Ie, the body should contain this:
 
 ```html
 <section id="main"></section>

<script src='./pkg/appname.js'></script>

<script>
    const { render } = wasm_bindgen;
    function run() {
        render();
    }
    wasm_bindgen('./pkg/appname_bg.wasm')
        .then(run)
        .catch(console.error);
</script>
```

The quickstart repo includes this file, but you will need to rename the two 
occurances of `appname`. (If your project name has a hyphen, use an underscore instead here) You will eventually need to modify this file to 
change the page's title, add a description, favicon, stylesheet etc.

`Cargo.toml`, which is a file created by Cargo that describes your app, needs `wasm-bindgen`, `web-sys`, and `
seed` added as depdendencies,
 and crate-type
of `"cdylib"`. (The version in the quickstart repo has these set up already) Example:

```toml
[package]
name = "appname"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
seed = "^0.1.6"
wasm-bindgen = "^0.2.29"
web-sys = "^0.3.6"

# For serialization, eg sending requests to a server. Otherwise, not required.
serde = "^1.0.80"
serde_derive = "^1.0.80"
serde_json = "1.0.33"
```

## A short example
Here's an example demonstrating structure and syntax; it can be found in working form
under `examples/counter`. Descriptions of its parts are in the
Guide section below. Its structure follows [The Elm Architecture](https://guide.elm-lang.org/architecture/).

*lib.rs*:
```rust

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone)]
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

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model {count: model.count + 1, ..model},
        Msg::Decrement => Model {count: model.count - 1, ..model},
        Msg::ChangeWWC(what_we_count) => Model {what_we_count, ..model }
    }
}


// View

/// A simple component.
fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0 ... 3 => "Not very many 🙁",
        4 ... 7 => "An OK amount 😐",
        8 ... 999 => "Good job! 🙂",
        _ => "You broke it 🙃"
    };
    p![ descrip ]
}

/// The top-level component we pass to the virtual dom. Must the model as its
/// only argument, and output a single El.
fn view(model: Model) -> El<Msg> {
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
            button![ simple_ev("click", Msg::Increment), "+" ],
            button![ simple_ev("click", Msg::Decrement), "-" ],

            // Optionally-displaying an element
            if model.count >= 10 { h2![ style!{"padding" => 50}, "Nice!" ] } else { seed::empty() }

            ],
        success_level(model.count),  // Incorporating a separate component

        h3![ "What precisely is it we're counting?" ],
        input![ attrs!{"value" => model.what_we_count}, input_ev("input", Msg::ChangeWWC) ]
    ]
}


#[wasm_bindgen]
pub fn render() {
    // The final parameter is an optional routing map.
    seed::run(Model::default(), update, view, "main", None);
}
```
For truly minimimal example, see [lib.rs in the quickstart repo](https://github.com/David-OConnor/seed-quickstart/blob/master/src/lib.rs)

## Building and running
To build your app, create a `pkg` subdirectory, and run the following two commands:

```
cargo build --target wasm32-unknown-unknown
```
and 
```
wasm-bindgen target/wasm32-unknown-unknown/debug/appname.wasm --no modules --out-dir ./pkg
```
where `appname` is replaced with your app's name. This compiles your code in the target
folder, and populates the pkg folder with your WASM module, a Typescript definitions file,
and a Javascript file used to link your module from HTML.

You may wish to create a build script with these two lines. (`build.sh` for Linux; `build.ps1` for Windows).
The Quickstart repo includes these, but you'll still need to do the rename. You can then use
`./build.sh` or `.\build.ps1` If you run into permission errors on `build.sh`, try this command
to allow executing the file:`chmod +x build.sh`.

For development, you can view your app using a shimmed Python dev server described above.
(Set up [this mime-type shim](https://github.com/David-OConnor/seed-quickstart/blob/master/serve.py)
from the quickstart repo, and run `python serve.py`).

For details, reference [the wasm-bindgen documention](https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html).
In the future, I'd like the build script and commands above to be replaced by [wasm-pack](https://github.com/rustwasm/wasm-pack).

## Running included examples
To run an example located in the `examples` folder, navigate to that folder in a terminal, 
run the build script for your system (`build.sh` or `build.ps1`), then start a dev server
 as described above. Note that if you copy an example to a separate folder, you'll need
to edit its `Cargo.toml` to point to the package on [crates.io](https://crates.io) instead of locally: Ie replace
`seed = { path = "../../"` with `seed = "^0.1.0"`, and in the build script, remove the leading `../../` on the second
line.
## About

## Goals
- Learning the syntax, creating a project, and building it should be easy - regardless
of your familiarity with Rust.

- Complete documentation that always matches the current version. Getting examples working, and
 starting a project should be painless, and require nothing beyond this guide.
 
- Expressive, flexible vew syntax that's easy to read and write.


## A note on view syntax
This project takes a different approach to describing how to display DOM elements 
than others. It neither uses completely natural (ie macro-free) Rust code, nor
an HTML-like abstraction (eg JSX or templates). My intent is to make the code close 
to natural Rust, while streamlining the syntax in a way suited for creating 
a visual layout with minimal repetition. The macros used here are thin wrappers
for constructors, and don't conceal much. Specifically, the element-creation macros
allow for accepting a variable number of arguments, and the attrs/style marcros are 
essentially HashMap literals, with wrappers that let el macros know how to distinguish
them.

The lack of resemblance to HTML be offputting, but the learning
curve is shallow, and I think the macro syntax used to create elements, attributes etc
is close-enough to normal Rust syntax that it's easy to reason about how the code
should come together, without compartmentalizing it into logic code and display code.
 This lack of separation
in particular is a subjective, controversial decision, but I think the benefits 
are worth it.


## Where to start if you're familiar with existing frontend frameworks
The [todomvc example](https://github.com/David-OConnor/seed/tree/master/examples/todomvc) is an implementation of the [TodoMVC project](http://todomvc.com/),
which has example code in my frameworks that do the same thing. Compare the example in this
project to one on that page that uses a framework you're familiar with.

## Suggestions? Critique? Submit an issue or pull request on Github

## Influences
This project is strongly influenced by Elm, React, and Redux. The overall layout
of Seed apps mimicks that of The Elm Architecture.


## Why another entry in a saturated field?

### There are already several Rust/WASM frameworks; why add another?

My goal is for this to be easy to pick up from looking at a tutorial or documentation, regardless of your
level of experience with Rust. I'm distinguising this package through clear examples
and documentation (see goals above), and using `wasm-bindgen` internally. I started this
project after being unable to get existing frameworks to work
due to lack of documented examples, and inconsistency between documentation and 
published versions. My intent is for anyone who's proficient in a frontend
framework to get a standalone app working in the browser within a few minutes, using just the 
quickstart guide.

Seed approaches HTML-display syntax differently from existing packages: 
rather than use an HTML-like markup similar to JSX, 
it uses Rust builtin types, thinly-wrapped by a macro for each DOM element.
This decision may not appeal to everyone, 
but I think it integrates more naturally with the language.

### Why build a frontend in Rust over Elm or Javascript-based frameworks?

You may prefer writing in Rust, and using packages from Cargo vis npm. Getting started with
this framework will, in most cases be faster, and require less config and setup overhead than
with JS frameworks. You like the advantages of compile-time error-checking.

You may choose 
this approach over Elm if you're already comfortable with Rust, want the performance 
benefits, or don't want to code business logic in a purely-functional langauge.

Compared with React, you may appreciate the consistency of how to write apps:
There's no distinction between logic and display code; no restrictions on comments;
no distinction between components and normal functions. The API is
flexible, and avoids OOP boilerplate.

I also hope that config, building, and dependency-management is cleaner with Cargo and
wasm-bindgen than with npm.

### Shoutouts
 - The [WASM-Bindgen](https://github.com/rustwasm/wasm-bindgen) team: 
 For building the tools this project relies on
 - Alex Chrichton, for being extraodinarily helpful in the Rust / WASM community
 - The [Elm](https://elm-lang.org/) team: For creating and standardizing the Elm architecture
 - Denis Kolodin: for creating the inspirational [Yew framework](https://github.com/DenisKolodin/yew)
 - Utkarsh Kukreti, for through his [Draco repo](https://github.com/utkarshkukreti/draco), 
 helping me understand how wasm-bindgen's
 closure system can be used to update state.
 - Tim Robinson, for being very helpful on the [Rust Gitter](https://gitter.im/rust-lang/rust).

### Features to add
 - High-level fetch API
 - Lifecycle hooks
 - SVG support
 - More flexible routing
 - Virtual DOM optimization 
 - High-level CSS-grid/Flexbox API ?
 
 ### Bugs to fix
 - Text renders above children instead of below
 
 ## Reference
- [wasm-bindgen guide](https://rustwasm.github.io/wasm-bindgen/introduction.html)
- [Mozilla MDN web docs](https://developer.mozilla.org/en-US/)
- [web-sys api](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) (A good partner for the MDN docs - most DOM items have web-sys equivalents used internally)
- [Rust book](https://doc.rust-lang.org/book/index.html)
- [Rust standard library api](https://doc.rust-lang.org/std/)
- [Seed's API docs](https://docs.rs/seed)
- [Learn Rust](https://www.rust-lang.org/learn)

