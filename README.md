# Seed

**A Rust framework for creating web apps**

## Quickstart

### Setup
This package requires you to install [Rust](https://www.rust-lang.org/en-US/) - This will
enable the CLI commands below:

 You'll need a recent version of Rust's nightly toolchain:
`rustup update`
`rustup default nightly`,

The wasm32-unknown-unknown target:
`rustup target add wasm32-unknown-unknown --toolchain nightly`

And wasm-bindgen: 
`cargo +nightly install wasm-bindgen-cli`

To start, either clone [This quickstart repo](https://github.com/David-OConnor/seed-quickstart) 
or create a new lib with Cargo: `cargo new --lib appname` Here and everywhere it appears in this guide, `
appname` refers to the name of your app.

You need an Html file that loads your app's compiled module, and provides a div with id 
to load the framework into. It also needs the following code to load your WASM module -
 Ie, the body should contain this:
 
 (todo: Once the --out-name flag is enabled on bindgen, simplify this section, explaining how
 the quickstart repo works immediately by running the build script, and opening the HTML file)

```html
 <div id="main"></div>

<script>
    delete WebAssembly.instantiateStreaming;
</script>

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
occurances of `appname`. You will eventually need to modify this file to 
change the page's title, add a description, favicon, stylesheet etc.

`Cargo.toml`needs `wasm-bindgen`, `web-sys`, and `seed` as depdendencies, and crate-type
of `"cdylib"`. Example:

```toml
[package]
name = "appname"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
seed = "^0.1.0"
wasm-bindgen = "^0.2.29"
web-sys = "^0.3.6"

# For serialization, eg sending requests to a server. Otherwise, not required.
serde = "^1.0.80"
serde_derive = "^1.0.80"
serde_json = "1.0.33"

```

### A short example
Here's an example app to demonstrating syntax. Descriptions of its parts are in the
Guide section below.

*lib.rs*:
```rust
#[macro_use]
// This is required to allow access to element-creation macros
extern crate seed;

/// Introduce The `El` type into the global namespace, as well as a trait used
/// to make macros work.
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone)]
struct Model {
    count: i32,
    what_we_count: String,
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
    ChangeWWC(String)
}

// Sole source of updating the model; returns a whole new model.
fn update(msg: &Msg, model: &Model) -> Model {
    match msg {
        Msg::Increment => {
            Model {count: model.count + 1, what_we_count: model.what_we_count}
        },
        Msg::Decrement => {
            Model {count: model.count - 1, what_we_count: model.what_we_count}
        },
        Msg::ChangeWWC(text) => {
            Model {count: model.count, what_we_count: text}
        },
    }
}


// View

fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0 ... 3 => "Not very many 🙁",
        4 ... 7 => "An OK amount 😐",
        8 ... 999 => "Good job! 🙂",
        _ => "You broke it 🙃"
    };
    p![ descrip ]
}

// Top-level component we pass to the virtual dom. Must accept the model as its
// only argument, and output a single El.
fn main_comp(model: &Model) -> El<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    let outer_style = style!{
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

     div![ outer_style, vec![
        h1![ "The Grand Total" ],
        div![
            style!{
                "color" => if model.count > 4 {"purple"} else {"gray"};
                "border" => "2px solid #004422"
            },
            vec![
                h3![ format!("{} {}{} so far", model.count, model.what_we_count, plural) ],
                button![ events!{"click" => Msg::Increment}, "+" ],
                button![ events!{"click" => Msg::Decrement}, "-" ]
            ] ],
        success_level(model.count),

        h3![ "What precisely is it we're counting?" ],
        input![ attrs!{"value" => model.what_we_count}, events!{
                "change" => |ev| Msg::ChangeWWC(ev)
        } ]
    ] ]
}


#[wasm_bindgen]
pub fn render() {
    seed::vdom::run(Model::default(), update, main_comp, "main");
}
```

### Building and running
To build your app, run the following two commands:

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
The Quickstart repo includes this, but you'll still need to do the rename. You can then use
`./build.sh` or `.\build.ps1`

For development, you can view your app using a dev server, or by opening the HTML file in a browser.

For example, after installing the  [http crate](https://crates.io/crates/https), run `http`.
Or with [Python](https://www.python.org/) installed, run `python -m http.server` from your crate's root.

For details, reference [the wasm-bindgen documention](https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html).

(Todo: Release version)

### Running included examples
To run an example located in the `examples` folder, navigate to that folder in a terminal, 
run the build script for your system (`build.sh` or `build.ps1`), then open the `index.html` file
in a web browser. Note that if you copy an example to a separate folder, you'll need
to edit its `Cargo.toml` to point to the package.crates.io instead of locally: Ie replace
`seed = { path = "../../"` with `seed = "^0.1.0"`, and in the build script, remove the leading `../../` on the second
line.

## Guide

### Prerequisites
**Rust**: Proficiency in Rust isn't required to get started using this framework.
It helps, but I think you'll be able to build a usable webapp using this guide,
and example code alone. For business logic behind the GUI, more study may be required.
The official [Rust Book](https://doc.rust-lang.org/book/index.html) is a good
place to start.

You'll be able to go with just the basic Rust syntax common to most programming
languages, eg conditionals, equalities, iteration, collections, and how Rust's borrow system applies
to strings. A skim through the first few chapters of the Book, and the examples here should provide 
what you need. Rust's advanced and specialized features like lifetimes, generics, smartpointers, and traits
aren't required to build an interactive GUI.

**Web fundamentals**: Experience building websites using HTML/CSS or other frameworks
is required. Neither this guide nor the API docs describes how web pages are structured,
or what different HTML/DOM elements, attributes, styles etc do. You'll need to know these before
getting started. Seed provides tools used to assemble and manipulate these fundamentals.
Mozilla's [MDN web docs](https://developer.mozilla.org/en-US/docs/Learn)
is a good place to start.

**Other frontend frameworks** The design principles Seed uses are similar to those
used by React, Elm, and Yew. People familiar with how to set up interactive web pages
using these tools will likely have an easy time learning Seed.


### App structure

**Model**

Each app must contain a model [struct]( https://doc.rust-lang.org/book/ch05-00-structs.html), 
which contains the app’s state and data. It should contain 
[owned data](https://doc.rust-lang.org/book/2018-edition/ch04-00-understanding-ownership.html). References
with a static [lifetime](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) may work,
but will be more difficult to work with. Example:

```rust
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
```
 
The first line, `#[derive(Clone)]` is required to let Seed make copies or it, and
display it internally. In this example, we provide 
initialization via Rust’s `Default` trait, in order to keep the initialization code by the
 model itself. When we call `Model.default()`, it initializes with these values. We could 
 also initialize it using a constructor method, or a struct literal. Note the use of `into()` 
 on our string literal, to convert it into an owned string.
 
The model holds all data used by the app, and will be replaced with updated versions when the data changes.
Use owned data in the model; eg `String` instead of `&'static str`.

 The model may be split into sub-structs to organize it – this is especially useful as the app grows. 
The sub-structs must also implement `Clone`:
 

```rust
#[derive(Clone)]
struct FormData {
    name: String,
    age: i8,
}

#[derive(Clone)]
struct Misc {
    value: i8,
    descrip: String,
}

#[derive(Clone)]
struct Model {
    form_data: FormData,
    misc: Misc
}
```

 **Update**

The Message is an [enum]( https://doc.rust-lang.org/book/ch06-00-enums.html) which 
categorizes each type of interaction with the app. Its fields may hold a value, or not. 
We’ve abbreviated it as `Msg` here for brevity. Example:

```rust
#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeDescrip(String),
}
```
 
The update [function]( https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) 
you pass to `seed::vdom::run` describes how the state should change, upon
receiving each type of Message. It is the only place where the model is changed. It accepts a message reference, and model 
reference as parameters, and returns a Model instance. This function signature cannot be changed.
 Note that it doesn’t update the model in place: It returns a new one.
 
Example:

```rust
// Sole source of updating the model; returns a whole new model.
fn update(msg: &Msg, model: &Model) -> Model {
    match msg {
        &Msg::Increment => {
            Model {count: model.count + 1, what_we_count: model.what_we_count}
        },
        &Msg::Decrement => {
            Model {count: model.count - 1, what_we_count: model.what_we_count}
        },
        &Msg::ChangeWWC() => {
//            Model {count: model.count, what_we_count: ev.target.value}
            Model {count: model.count, what_we_count: "thing"}
        },
    }
}
```
As with the model, only one update function is passed to the app, but it may be split into 
sub-functions to aid code organization.
 

**View**

 Visual layout (ie HTML/DOM elements) is described declaratively in Rust, but uses 
[macros]( https://doc.rust-lang.org/book/appendix-04-macros.html) to simplify syntax. 

### Elements, attributes, styles, and events.
When passing your layout to Seed, attributes for DOM elements (eg id, class, src etc), 
styles (eg display, color, font-size), and
events (eg onclick, contextmenu, dblclick) are passed to DOM-macros (like div!{}) using
unique types.

Views are described using El structs, defined in the `dom_types` module. They're most-easily created
with a shorthand using macros. These macros can take any combination of the following 5 argument types:
(0 or 1 of each) `Attrs`, `Style`, `Events`, `Vec<El>` (children), and `&str` (text). Attrs, Style, and Events
are most-easily created usign the following macros respectively: `attrs!{}`, `style!{}`, and `events!{}`. All
elements present must be aranged in the order above: eg `Events` can never be before `Attrs`.

`Attrs`, and `Style` values can be owned `Strings`, `&str`s, or when applicable, numerical and 
boolean values. Eg: `input![ attrs!{"disabled" => false]` and `input![ attrs!{"disabled" => "false"]` 
are equivalent. If a numerical value is used in a `Style`, 'px' will be automatically appended.
If you don't want this behavior, use a `String` or`&str`. Eg: `h2![ style!{"font-size" => 16} ]` , or
`h2![ style!{"font-size" => "1.5em"} ]` for specifying font size in pixels or em respectively. Note that
once created, a `Style` instance holds all its values as `Strings`; eg that `16` above will be stored
as `"16px"`; keep this in mind if editing a style that you made outside an element macro.

For example, the following code returns an `El` representing a few DOM elements displayed
in a flexbox layout:
```rust
    div![ style!{"display" => "flex"; "flex-direction" => "column"}, vec![
        h3![ "Some things" ],
        button![ events!{"click" => |_| Msg::SayHi}, "Click me!" ]
    ] ]
```

The only magic parts of this are the macros used to simplify syntax for creating these
things: text are normal rust borrowed strings; children are Vecs of sub-elements; 
Attrs, Style and Events are thinly-wrapped HashMaps. They can be created independently, and
passed to the macros separately. The following code is equivalent; it uses constructors
from the El struct. Note that `El` type is imported with the Prelude.


```rust
    use seed::dom_types::Tag;
    
    // heading and button here show two types of element constructors
    let mut heading = El::new(
        Tag::H2, 
        Attrs::empty(), 
        Style::empty(), 
        events::Empty,
        "Some things",
        vec::New()
    );  
    
    let mut button = El::empty(Tag::Button);
    button.add_event("click", |_| Msg::SayHi);

    let children = vec![heading, button];
    
    let mut elements = El::empty(Tag::Div);
    el.add_style("display", "flex");
    el.add_style("flex-direction", "column");
    el.children = children;
    
    el
    
```

The following equivalent example shows creating the required structs without constructors,
to demonstrate that the macros and constructors above represent normal Rust structs,
and provide insight into what abstractions they perform:

```rust
// todo: Events is Depricated; below ex is incorrect re that.
use seed::dom_types::{Attrs, Events, Style, Tag};

// Rust has no built-in HashMap literal syntax.
let mut style = HashMap::new();
style.insert("display", "flex");  
style.insert("flex-direction", "column");  

El {
    tag: Tag::Div,
    attrs: Attrs { vals: HashMap::new() },
    style,
    events: Events { vals: Vec::new() },
    text: None,
    children: vec![
        El {
            tag: Tag::H2,
            attrs: Attrs { vals: HashMap::new() },
            style: Style { vals: HashMap::new() },
            events: Events { vals: Vec::new() },
            text: Some(String::from("Some Things")),
            children: Vec::new()
        },
        El {
            tag: Tag::button,
            attrs: Attrs { vals: HashMap::new() },
            style: Style { vals: HashMap::new() },
            events: Events { vals: vec![("click", |_| Msg::SayHi)] },
            text: None,
            children: Vec::new()
        } 
    ]
}
```

For most uses, the first example (using macros) will be the easiest to read and write.
You can mix in constructors (or struct literals) in components as needed, depending on your code structure.


### Components
The analog of components in frameworks like React are normal Rust functions that that return Els.
The parameters these functions take are not treated in a way equivalent
to attributes on native DOM elements; they just provide a way to 
organize your code. In practice, they feel similar to components in React, but are just
functions used to create elements that end up in the `children` property of
parent elements.

For example, you could break up the above example like this:

```rust
    fn text_display(text: &str) -> El<Msg> {
        h3![ text ]
    }  
    
    div![ style!{"display" => "flex"; flex-direction: "column"}, vec![
        text_display("Some things"),
        button![ events!{"click" => |_| Msg::SayHi}, "Click me!" ]
    ] ]
```

The text_display() component returns a single El that is inserted into its parents'
`children` Vec; you can use this in patterns as you would in React. You can also use
functions that return Vecs or Tuples of Els, which you can incorporate into other components
using normal Rust code. See Fragments
section below. Rust's type system
ensures that only `El`s  can end up as children, so if your app compiles,
you haven't violated any rules.
 
Note that unlike in JSX, there's a clear syntax delineation here between natural HTML
elements (element macros), and custom components (function calls).

### Fragments
Fragments (`<>...</>` syntax in React and Yew) are components that represent multiple
elements without a parent. This is useful to avoid
unecessary divs, which may be undesirable on their own, and breaks things like tables and CSS-grid. 
There's no special syntax; just have your component return a Vec of `El`s instead of 
one, and pass them into the parent's `children` parameter via Rust's Vec methods
like `extend`, or pass the whole Vec if there are no other children:

```rust
fn cols() -> Vec<El<Msg>> {
    vec![
        td![ "1" ],
        td![ "2" ],
        td![ "3" ]
    ]
}

fn items() -> El<Msg> {
    table![ vec![
        tr![ cols() ]
    ]
}
```

### Initializing your app
To start your app, pass an instance of your model, the update function, the top-level component function 
(not its output), and name of the div you wish to mount it to to the `seed::vdom::run` function:
```rust
#[wasm_bindgen]
pub fn render() {
    seed::vdom::run(Model::default(), update, main_comp, "main");
}
```
This must be wrapped in a function named `render`, with the `#[wasm_bindgen]` invocation above.
(More correctly, its name must match the func in this line in your html file):
```javascript

function run() {
    render();
}
```
Note that you don't need to pass your Msg enum; it's inferred from the update function.

### Comments in the view
The Element-creation macros used to create views are normal Rust code, you can
use comments in them normally: either on their own line, or in line.


### Logging in the web browser
To output to teh web browser's console (ie `console.log()` in JS), use `web_sys::console_log1`,
or the `log` convenience function: `seed::log("hello, world!")`


### Serialization and deserialization
Use the [Serde](https://serde.rs/) crate to serialize and deserialize data, eg
when sending and receiving data from a REST-etc. It supports most popular formats,
including `JSON`, `YAML`, and `XML`.

(Example, and with our integration)

### Querying servers using fetch
To send and receive data with a server, use `wasm-bindgen`'s `web-sys` fetch methods,
[described here](https://rustwasm.github.io/wasm-bindgen/examples/fetch.html), paired
with Serde.

Check out the `server_interaction` examples for an example of how to send and receive
data from the server in JSON.

### Local storage
You can store page state locally using web_sys's [Storage struct](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html)
.

### Building a release version
The configuration in the [Building and Running](###building-and-running) section towards the top are intended
for development: They produce large `.wasm` file sizes, and unoptimized performance.
For your release version, you'll need to append `--release` to the `cargo build` command,
and point your `wasm-bindgen` command to the `release` subdirectory vice `debug`.
Example:

```
cargo build --target wasm32-unknown-unknown --release
```
and 
```
wasm-bindgen target/wasm32-unknown-unknown/release/appname.wasm --no modules --out-dir ./pkg
```

## Goals
- Learning the syntax, creating a project, and building it should be easy - regardless
of your familiarity with Rust.

- Complete documentation that always matches the current version. Getting examples working, and
 starting a project should be painless, and require nothing beyond this guide.

- An API that's easy to read, write, and understand.


### A note on the view syntax
This project takes a different approach to describing how to display DOM elements 
than others. It neither uses completely natural (ie macro-free) Rust code, nor
an HTML-like abstraction (eg JSX or templates). My intent is to make the code close 
to natural Rust, while streamlining the syntax in a way suited for creating 
a visual layout with minimal repetition. The macros used here are thin wrappers
for constructors, and don't conceal much.

The relative lack of resemblance to HTML be offputting at first, but the learning
curve is shallow, and I think the macro syntax used to create elements, attributes etc
is close-enough to normal Rust syntax that it's easy to reason about how the code
should come together, without compartmentalizing it into logic code and display code.
 This lack of separation
in particlar is a subjective, controversial decision, but I think the benefits 
are worth it.


### Where to start if you're familiar with existing frontend frameworks
The Todo MVC example (examples/todomvc) is an implementation of the [TodoMVC project](http://todomvc.com/),
which has example code in my frameworks that do the same thing. Compare the example in this
project to one on that page that uses a framework you're familiar with.

### Suggestions? Critique? Submit an issue or pull request on Github


### Influences
This project is strongly influenced by Elm, React, and Redux. The overall layout
of Seed apps mimicks that of The Elm Architecture.


### Why another entry in a saturated field?

**There are already several Rust/WASM frameworks; why add another?** 

My goal is for this to be easy to pick up from looking at a tutorial or documentation, regardless of your
level of experience with Rust. I'm distinguising this package through clear examples
and documentation (see goals above), and using `wasm-bindgen` internally. I started this
project after being unable to get existing frameworks to work
due to lack of documented examples, and inconsistency between documentation and 
published versions. My intent is for anyone who's proficient in a frontend
framework to get a standalone app working in the browser within a few minutes, using just the 
[Quickstart guide](##quickstart).

Seed approaches HTML-display syntax differently from existing packages: 
rather than use an HTML-like markup similar to JSX, 
it uses Rust builtin types, thinly-wrapped by a macro for each DOM element.
This decision may not appeal to everyone, 
but I think it integrates more naturally with the language.

**Why build a frontend in Rust over Elm or Javascript-based frameworks?**

You may prefer writing in Rust, and using packages from Cargo vis npm. Getting started with
this framework will, in most cases be faster, and require less config and setup overhead than
with JS frameworks.

You may choose 
this approach over Elm if you're already comfortable with Rust, want the performance 
benefits, or don't want to code business logic in a purely-functional langauge.

Compared to React, for example, you may appreciate the consistency of how to write apps:
There's no distinction between logic and display code; no restrictions on comments;
no distinction between components and normal functions. The API is
flexible, and avoids the OOP boilerplate.

I also hope that config, building, and dependency-management is cleaner with Cargo and
wasm-bindgen than with npm.

## Shoutouts
 - The [WASM-Bindgen](https://github.com/rustwasm/wasm-bindgen) team: 
 For building the tools this project relies on
 - Alex Chrichton: For being extraodinarily helpful in the Rust / WASM community
 - The [Elm](https://elm-lang.org/) team: For creating and standardizing the Elm architecture
 - Denis Kolodin: for creating the inspirational [Yew framework](https://github.com/DenisKolodin/yew)
 - Utkarsh Kukreti, for through his [Draco repo](https://github.com/utkarshkukreti/draco), 
 helping me understand how wasm-bindgen's
 closure system can be used to update state.


## To-do
 - Router
 - Local storage integration
 - More examples
 - Optimization 
 