# Seed

**A Rust framework for creating web apps**

[![](https://meritbadge.herokuapp.com/seed)](https://crates.io/crates/seed)
[![](https://img.shields.io/crates/d/seed.svg)](https://crates.io/crates/seed)
[![API Documentation on docs.rs](https://docs.rs/seed/badge.svg)](https://docs.rs/seed)


## Quickstart

### Setup
This package requires you to install [Rust](https://www.rust-lang.org/tools/install) - This will
enable the CLI commands below:

 You'll need a recent version of Rust: `rustup update`

The wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

And wasm-bindgen: `cargo install wasm-bindgen-cli`


### The theoretical minimum
To start, clone [This quickstart repo](https://github.com/David-OConnor/seed-quickstart),
run `build.sh` or `build.ps1` in a terminal, and open `index.html`. (May need to use
a local server depending on the browser) Once you change your package name, you'll
need to tweak the Html file and build script, as described below.

### A little deeper
Or, create a new lib with Cargo: `cargo new --lib appname`. Here and everywhere it appears in this guide, `
appname` should be replaced with the name of your app.

If not using the quickstart repo, create an Html file that loads your app's compiled module, 
and provides an element with id 
to load the framework into. It also needs the following code to load your WASM module -
 Ie, the body should contain this:
 
 ```html
 <section id="main"></section>

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
The `delete WebAssembly.instantiateStreaming;` line is an unsavory hack, but without it,
Most dev servers, and opening the html file directly won't work. If you'd like to avoid this, 
delete that line, install
[Python](https://www.python.org/downloads/), and run `python server.py`. This
file is included in the quickstart repo, and in each example folder. It's a shim
that allows Python's dev server to work with the WASM mime type. Linux users may have
to run `python3 server.py`.

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
seed = "^0.1.4"
wasm-bindgen = "^0.2.29"
web-sys = "^0.3.6"

# For serialization, eg sending requests to a server. Otherwise, not required.
serde = "^1.0.80"
serde_derive = "^1.0.80"
serde_json = "1.0.33"

```

### A short example
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

/// The top-level component we pass to the virtual dom. Must accept a ref to the model as its
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
    seed::run(Model::default(), update, view, "main");
}
```

### Building and running
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
`./build.sh` or `.\build.ps1`

For development, you can view your app using a shimmed Python dev server described above,
(Run `python server.py`) or by opening the HTML file in a browser.

For details, reference [the wasm-bindgen documention](https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html).
In the future, I'd like the build script and commands above to be replaced by [wasm-pack](https://github.com/rustwasm/wasm-pack).

### Running included examples
To run an example located in the `examples` folder, navigate to that folder in a terminal, 
run the build script for your system (`build.sh` or `build.ps1`), then open the `index.html` file
in a web browser, or use the Python dev server. Note that if you copy an example to a separate folder, you'll need
to edit its `Cargo.toml` to point to the package on [crates.io](https://crates.io) instead of locally: Ie replace
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
using these tools will likely have an easy time learning this.


### App structure

**Model**

Each app must contain a model [struct]( https://doc.rust-lang.org/book/ch05-00-structs.html), 
which contains the app’s state and data. It must derive `Clone`, and should contain 
[owned data](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html). References
with a static [lifetime](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) may work,
but may be more difficult to work with. Example:

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
 
In this example, we provide 
initialization via Rust’s `Default` trait, in order to keep the initialization code by the
 model itself. When we call `Model.default()`, it initializes with these values. We could 
 also initialize it using a constructor method, or a struct literal. Note the use of `into()` 
 on our string literal, to convert it into an owned string.
 
The model holds all data used by the app, and will be replaced with updated versions when the data changes.
Use owned data in the model; eg `String` instead of `&'static str`.

 The model may be split into sub-structs to organize it – this is especially useful as the app grows. 
Sub-structs must implement `Clone`:
 

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
We’ve abbreviated it as `Msg` here for brevity. If you're not familiar with enums,
think of one as a set of options; in other languages, you might use an integer, or string 
for this, but an enum is explicitly limited in which values it can take. Example:

```rust
#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeDescrip(String),
}
```
 
The update [function]( https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) 
you pass to `seed::run` describes how the state should change, upon
receiving each type of Message. It is the only place where the model is changed. It accepts a message, 
and model as parameters, and returns a model. This function signature cannot be changed.
 Note that it doesn’t update the model in place: It returns a new one.

Example:

```rust
fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model {count: model.count + 1, ..model},
        Msg::SetCount(count) => Model {count, ..model},
    }
}
```

 While the signature of the update function is fixed (Accepts a Msg and ref to the model; outputs
 a new model), and will usually involve a match pattern, with an arm for each Msg, there
 are many ways you can structure this function. Some may be easier to write, and others may 
 be more efficient, or appeal to specific aesthetics. While the example above
 it straightforward, this becomes import with more complicated updates.
 
 The signature suggests taking an immutable-design/functional approach. This can be verbose
 when modifying collections, but is a common pattern in Elm and Redux. Unlike in a pure functional language,
 side-effects (ie other things that happen other than updating the model) don't require special 
 handling. Example, from the todomvc example:
```rust
fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::ClearCompleted => {
            let todos = model.todos.into_iter()
                .filter(|t| !t.completed)
                .collect();
            Model {todos, ..model}
        },
        Msg::Destroy(posit) => {
            let todos = model.todos.into_iter()
                .enumerate()
                .filter(|(i, t)| i != &posit)
                .map(|(i, t)| t)
                .collect();
            Model {todos, ..model}
        },
        Msg::Toggle(posit) => {
            let mut todos = model.todos;
            let mut todo = todos.remove(posit);
            todo.completed = !todo.completed;
            todos.insert(posit, todo);

            Model {todos, ..model}
        },
        Msg::ToggleAll => {
            let completed = model.active_count() != 0;
            let todos = model.todos.into_iter()
                .map(|t| Todo {completed, ..t})
                .collect();
            Model {todos, ..model}
        }
    }
}
 ```
In this example, we avoid mutating data. In the first two Msgs, we filter the todos,
then pass them to a new model using [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax)
.  In the third Msg, we mutate todos, but don't mutate the model itself. In the fourth,
we build a new todo list using a functional technique. The [docs for Rust Iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
show helpful methods for functional iterator manipulation.

Alternatively, we could write the same update function like this:
```rust
fn update(msg: Msg, model: Model) -> Model {
    let mut model = model;
    match msg {
        Msg::ClearCompleted => {
            model.todos = model.todos.into_iter()
            .filter(|t| !t.completed)
            .collect();
        },
        Msg::Destroy(posit) => {
            model.todos.remove(posit);
        },
        Msg::Toggle(posit) => model.todos[posit].completed = !model.todos[posit].completed,
        Msg::ToggleAll => {
            let completed = model.active_count() != 0;
            for todo in &mut model.todos {
                todo.completed = completed;
        }
    };
    model
}
 ```
This approach, where we mutate the model directly, is much more concise when
handling collections. How-to: Reassign `model` as mutable at the start of `update`. 
Return `model` at the end. Mutate it during the match legs.

As with the model, only one update function is passed to the app, but it may be split into 
sub-functions to aid code organization.

Note that you can perform updates recursively, ie have one update trigger another. For example,
here's a non-recursive approach, where functions do_things() and do_other_things() each
act on an Model, and output a Model:
 ```rust
fn update(fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::A => do_things(model),
        Msg::B => do_other_things(do_things(model)),
    }
}
 ```
Here's a recursive equivalent:
 ```rust
fn update(fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::A => do_things(model),
        Msg::B => do_other_things(update(Msg::A, model)),
    }
}
 ```

 
**View**

 Visual layout (ie HTML/DOM elements) is described declaratively in Rust, but uses 
[macros]( https://doc.rust-lang.org/book/appendix-04-macros.html) to simplify syntax. 

### Elements, attributes, styles
Elements are created using macros, named by the lowercase name of
each element, and imported into the global namespace:
```rust
#[macro_use]
extern crate seed;

// ...

div![]
```
These macros accept any combination (0 or 1 per) of the following parameters:
- One [Attrs](https://docs.rs/seed/0.1.4/seed/dom_types/struct.Attrs.html) struct
- One [Style](https://docs.rs/seed/0.1.4/seed/dom_types/struct.Style.html) struct
- One or more [Listener](https://docs.rs/seed/0.1.4/seed/dom_types/struct.Listener.html) structs, which handle events
- One or more Vecs of Listener structs
- One String or &str representing a node text
- One or more [El](https://docs.rs/seed/0.1.4/seed/dom_types/struct.El.html) structs, representing a child
- One or more Vecs of El structs, representing multiple children

The parameters can be passed in any order; the compiler knows how to handle them
based on their types. Children are rendered in the order passed.

Views are described using [El structs](https://docs.rs/seed/0.1.4/seed/dom_types/struct.El.html), 
defined in the [seed::dom_types](https://docs.rs/seed/0.1.4/seed/dom_types/index.html) module. They're most-easily created
with a shorthand using macros.

`Attrs` and `Style` are thinly-wrapped hashmaps created with their own macros: `attrs!{}` and `style!{}`
respectively.

Example:
```rust
let things = vec![ h4![ "thing1" ], h4![ "thing2" ] ];

div![ attrs!{"class" => "hardly-any"}, 
    things,
    h4![ "thing3?" ]
]
```
Note that you can create any of the above items inside an element macro, or create it separately,
and pass it in.

Values passed to `attrs`, and `style` macros can be owned `Strings`, `&str`s, or when applicable, numerical and 
boolean values. Eg: `input![ attrs!{"disabled" => false]` and `input![ attrs!{"disabled" => "false"]` 
are equivalent. If a numerical value is used in a `Style`, 'px' will be automatically appended.
If you don't want this behavior, use a `String` or`&str`. Eg: `h2![ style!{"font-size" => 16} ]` , or
`h2![ style!{"font-size" => "1.5em"} ]` for specifying font size in pixels or em respectively. Note that
once created, a `Style` instance holds all its values as `Strings`; eg that `16` above will be stored
as `"16px"`; keep this in mind if editing a style that you made outside an element macro.

Styles and Attrs can be passed as refs as well, which is useful if you need to pass
the same one more than once:
```rust
 let item_style = style!{
        "margin-top" => 10;
        "font-size" => "1.2em"
    };

    div![
        ul![
            li![ &item_style, "Item 1", ],
            li![ &item_style, "Item 2", ],
        ]
    ]
```

Setting an InputElement's `checked` property is done through normal attributes:
```rust
input![ attrs!{"type" => "checkbox"; "checked" => true ]
```

To edit Attrs or Styles you've created, you can edit their .vals HashMap. To add
a new part to them, use their .add method:
```rust
let mut attributes = attrs!{};
attributes.add("class", "truckloads");
```

Example of the style tag, and how you can use pattern-matching in views:
```rust
fn view(model: Model) -> El<Msg> {
    div![ style!{
        "display" => "grid";
        "grid-template-columns" => "auto";
        "grid-template-rows" => "100px auto 100px"
        },
        section![ style!{"grid-row" => "1 / 2"},
            header(),
        ],
        section![ attrs!{"grid-row" => "2 / 3"},
            match model.page {
                Page::Guide => guide(),
                Page::Changelog => changelog(),
            },
        ],
        section![ style!{"grid-row" => "3 / 4"},
            footer()
        ]
    ]
}
```

Overall: we leverage of Rust's strict type system to flexibly-create the view
using normal Rust code.

### Events

Events are created by passing a a [Listener](https://docs.rs/seed/0.1.4/seed/dom_types/struct.Listener.html),
, or vec of Listeners, created using the following four functions exposed in the prelude: `simple_ev`,
`input_ev`, `keyboard_ev`, and `raw_ev`. The first is demonstrated in the example in the quickstart section,
and all are demonstrated in the todomvc example.

`simple_ev` takes two arguments: an event trigger (eg "click", "contextmenu" etc), and an instance
of your `Msg` enum. (eg Msg::Increment). The other three event-creation-funcs
take a trigger, and a [closure](https://doc.rust-lang.org/book/ch13-01-closures.html) (An anonymous function,
similar to an arrow func in JS) that returns a Msg enum.

`simple_ev` does not pass any information about the event, only that it fired.
Example: 
```rust
enum Msg {
    ClickClick
}
// ...
simple_ev("dblclick", Msg::ClickClick)`
```

`input_ev` passes the event target's value field, eg what a user typed in an input field.
Example: 
```rust
enum Msg {
    NewWords(String)
}
// ...
input_ev("input", Msg::NewWords)
```

`keyboard_ev` returns a [web_sys::KeyboardEvent](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html),
which exposes several getter methods like `key_code` and `key`.
Example:
```rust
enum Msg {
    PutTheHammerDown(web_sys::KeyboardEvent)
}
// ...
keyboard_ev("input", Msg::PutTheHammerDown)
```

Note that in the examples for input_ev and keyboard_ev, the syntax is simplified since
we're only passing the field text, and keyboard event respectively to the Msg. The input_ev
example is Rust shorthand for ```input_ev("input, |text| Msg::NewWords(text)```. If you were
to pass something other than, or more than just the input text (Or KeyboardEvent for keyboard_ev, 
or Event for raw_ev described below),
you can't use this shorthand, and would have to do something like this intead,
explicitly writing the closure:
```rust
enum Msg {
    NewWords(String, u32)
}
// ...
input_ev("input", move |text| Msg::NewWords(text, 0))
```

`raw_ev` returns a [web_sys::Event](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html). 
It lets you access any part of any type of
event, albeit with more verbose syntax.
If you wish to do something like prevent_default(), or anything not listed above, 
you need to take this approach. Note that for many common operations, like taking
the value of an input element after an `input` or `change` event, you have to deal with
casting from a generic event or target to the specific one. Seed provides convenience
functions to handle this. They wrap wasm-bindgen's .dyn_ref() and .dyn_into(), from its
[JsCast](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html) trait.

Example syntax showing how you might use raw_ev; processing an input and handling a keyboard
event, while using prevent_default:
```rust
// (in update func)
Msg::KeyPress(event) => {
    event.prevent_default();
    let code = seed::to_kbevent(&ev).key_code();
    // ..
    let target = event.target().unwrap();
    let text = seed::to_input(&target).value();
    
    // ...
    // In view
    raw_ev("input", Msg::KeyPress),
}
```
Seed also provides `to_textarea` and `to_select` functions, which you'd use as
`to_input`.

This extra step is caused by a conflict between Rust's type system, and the way DOM events
are handled. For example, you may wish to pull text from an input field by reading the event target's
value field. However, not all targets contain value; it may have to be represented as
an `HtmlInputElement`. (See [the web-sys ref](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html), 
and [Mdn ref](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget); there's no value field)) Another example:
If we wish to read the key_code of an event, we must first cast it as a KeyboardEvent; pure Events
(web_sys and DOM) do not contain this field.

It's likely you'll be able to do most of what you wish with the simpler event funcs.
If there's a type of event or use you think would benefit from a similar func, submit
an issue or PR. In the descriptions above for all event-creation funcs, we assumed minimal code in the closure,
and more code in the update func's match arms. For example, to process a keyboard event,
these two approaches are equivalent:

```rust
enum Msg {
    KeyDown(web_sys::KeyboardEvent)
}

// ... (in update)
KeyDown(event) => {
    let code = event.key_code()
    // ...
}

// ... In view
keyboard_ev("keydown", Msg::KeyDown
```
and
```rust
enum Msg {
    KeyDown(u32)
}

// ... (in update)
KeyDown(code) => {
    // ...
}

// ... In view
keyboard_ev("keydown", |ev| KeyDown(ev.key_code()))
```

You can pass more than one variable to the `Msg` enum via the closure, as long
as it's set up appropriate in `Msg`'s definition. Note that if you pass a value to the enum
other than what's between ||, you may receive an error about lifetimes. This is corrected by
making the closure a move type. Eg:
```rust
keyboard_ev("keydown", move |ev| Msg::EditKeyDown(id, ev.key_code()))
```
Where `id` is a value defined earlier.

Event syntax may be improved later with the addition of a single macro that infers what the type of event 
is based on the trigger, and avoids the use of manually creating a `Vec` to store the
`Listener`s. For examples of all of the above (except raw_ev), check out the [todomvc example](https://github.com/David-OConnor/seed/tree/master/examples/todomvc).

The [todomvc example](https://github.com/David-OConnor/seed/tree/master/examples/todomvc) has a number of event-handling examples, including use of raw_ev, 
where it handles text input triggered by a key press, and uses prevent_default().


### Element-creation macros, under the hood

The following code returns an `El` representing a few DOM elements displayed
in a flexbox layout:
```rust
    div![ style!{"display" => "flex"; "flex-direction" => "column"},
        h3![ "Some things" ],
        button![ "Click me!" ]
    ]
```

The only magic parts of this are the macros used to simplify syntax for creating these
things: text are [Options](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)
 of Rust borrowed Strings; `Listeners` are stored in Vecs; children are elements and/or Vecs of;
`Attr`s and `Style` are thinly-wrapped HashMaps. They can be created independently, and
passed to the macros separately. The following code is equivalent; it uses constructors
from the El struct. Note that `El` type is imported with the Prelude.

```rust
    use seed::dom_types::{El, Attrs, Style, Tag};
    
    // heading and button here show two types of element constructors
    let mut heading = El::new(
        Tag::H2, 
        Attrs::empty(), 
        Style::empty(), 
        Vec::new(),
        "Some things",
        Vec::New()
    );  
    
    let mut button = El::empty(Tag::Button);
    let children = vec![heading, button];
    
    let mut elements = El::empty(Tag::Div);
    elements.add_style("display", "flex");
    elements.add_style("flex-direction", "column");
    elements.children = children;
    
    elements
```

The following equivalent example shows creating the required structs without constructors,
to demonstrate that the macros and constructors above represent normal Rust structs,
and provides insight into what abstractions they perform:

```rust
// We didn't provide an example of a Listener/style: These are
// more complicated to show using literals.
use seed::dom_types::{El, Attrs, Style, Tag};

// Rust has no built-in HashMap literal syntax.
let mut style = HashMap::new();
style.insert("display", "flex");  
style.insert("flex-direction", "column");  

El {
    tag: Tag::Div,
    attrs: Attrs { vals: HashMap::new() },
    style: Style { vals: style },
    events: Events { vals: Vec::new() },
    text: None,
    children: vec![
        El {
            tag: Tag::H2,
            attrs: Attrs { vals: HashMap::new() },
            style: Style { vals: HashMap::new() },
            listeners: Vec::new();
            text: Some(String::from("Some Things")),
            children: Vec::new()
        },
        El {
            tag: Tag::button,
            attrs: Attrs { vals: HashMap::new() },
            style: Style { vals: HashMap::new() },
            listeners: Vec::new();
            text: None,
            children: Vec::new(),
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

For example, you could break up one of the above examples like this:

```rust
    fn text_display(text: &str) -> El<Msg> {
        h3![ text ]
    }  
    
    div![ style!{"display" => "flex"; flex-direction: "column"},
        text_display("Some things"),
        button![ simple_ev("click", Msg::SayHi), "Click me!" ]
    ]
```

The text_display() component returns a single El that is inserted into its parents'
`children` Vec; you can use this in patterns as you would in React. You can also use
functions that return Vecs of Els, which you can incorporate into other components
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
one, and add it to the parent's element macro; on its own like in the example below,
 or with other children, or Vecs of children.

```rust
fn cols() -> Vec<El<Msg>> {
    vec![
        td![ "1" ],
        td![ "2" ],
        td![ "3" ]
    ]
}

fn items() -> El<Msg> {
    table![
        tr![ cols() ]
    ]
}
```

### Dummy elements
When performing ternary and related operations instead an element macro, all
branches must return `El`s to satisfy Rust's type system. Seed provides the
`empty()` function, which creates a VDOM element that will not be rendered:

```rust
div![
    if model.count >= 10 { h2![ style!{"padding" => 50}, "Nice!" ] } else { seed::empty() }
]
```
For more complicated construsts, you may wish to create the `children` Vec separately,
push what components are needed, and pass it into the element macro.


### Initializing your app
To start your app, pass an instance of your model, the update function, the top-level component function 
(not its output), and id of the element (Usually a Div or Section) you wish to mount it to to the `seed::run` function:
```rust
#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main");
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
To output to the web browser's console (ie `console.log()` in JS), use `web_sys::console_log1`,
or the `log` macro that wraps it, which is imported in the seed prelude: 
`log!("On the shoulders of", 5, "giants".to_string())`


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

Seed will implement a high-level fetch API in the future, wrapping web-sys's.

### Local storage
You can store page state locally using web_sys's [Storage struct](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html)

Seed provides convenience functions `seed::storage::get_storage`, which returns 
the `web_sys::storage` object, and `seed::storage::store_data` to store an arbitrary
Rust data structure that implements serde's Serialize. Example use:

```rust
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// ...
#[derive(Serialize, Deserialize)]
struct Data {
    // Arbitrary data (All sub-structs etc must also implement Serialize and Deserialize)
}

let storage = seed::storage::get_storage();
seed::storage::store(storage, "my-data", Data::new());

// ...

let loaded_serialized = storage.get_item("my-data").unwrap().unwrap();
let data = serde_json::from_str(&loaded_serialized).unwrap();

```



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

### Debugging
There are two categories of error message you can receive: I'm using a different definition than
 used in [this section of the Rust book](https://doc.rust-lang.org/book/ch09-00-error-handling.html).
Compiler errors, and panics. 

1: Errors while building, which will be displayed in the terminal 
where you ran `cargo build`, or the build script. Rust's compiler usually provides
helpful messages, so try to work through these using the information available. Examples include
syntax errors, passing a func/struct etc the wrong type of item, and running afoul of the 
borrow checker.

2: Runtime [panics](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html). 
These show up as console errors in the web browser. Example:
`panicked at 'assertion failed: index < len',`, and provide a traceback. (For example, a problem while using `unwrap()`). 
 They're often associated with`unwrap()` or `expect()` calls. Try to use expect(), with a useful
 error message instead of unwrap(): It your message will show in the console.

### Reference
- [wasm-bindgen guide](https://rustwasm.github.io/wasm-bindgen/introduction.html)
- [Mozilla MDN web docs](https://developer.mozilla.org/en-US/)
- [web-sys api](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) (A good partner for the MDN docs - most DOM items have web-sys equivalents used internally)
- [Rust book](https://doc.rust-lang.org/book/index.html)
- [Rust standard library api](https://doc.rust-lang.org/std/)
- [Seed's API docs](https://docs.rs/seed)
- [Learn Rust](https://www.rust-lang.org/learn)

## About

### Goals
- Learning the syntax, creating a project, and building it should be easy - regardless
of your familiarity with Rust.

- Complete documentation that always matches the current version. Getting examples working, and
 starting a project should be painless, and require nothing beyond this guide.

- An API that's easy to read, write, and understand.


### A note on view syntax
This project takes a different approach to describing how to display DOM elements 
than others. It neither uses completely natural (ie macro-free) Rust code, nor
an HTML-like abstraction (eg JSX or templates). My intent is to make the code close 
to natural Rust, while streamlining the syntax in a way suited for creating 
a visual layout with minimal repetition. The macros used here are thin wrappers
for constructors, and don't conceal much. Specifically, the element-creation macros
allow for accepting a variable number of arguments, and the attrs/style marcros are 
essentially HashMap literals, with wrappers that let el macros know to distinguish
them.

The relative lack of resemblance to HTML be offputting at first, but the learning
curve is shallow, and I think the macro syntax used to create elements, attributes etc
is close-enough to normal Rust syntax that it's easy to reason about how the code
should come together, without compartmentalizing it into logic code and display code.
 This lack of separation
in particlar is a subjective, controversial decision, but I think the benefits 
are worth it.


### Where to start if you're familiar with existing frontend frameworks
The [todomvc example](https://github.com/David-OConnor/seed/tree/master/examples/todomvc) is an implementation of the [TodoMVC project](http://todomvc.com/),
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
with JS frameworks. You like the advantages of compile-time error-checking.

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
 - Alex Chrichton, for being extraodinarily helpful in the Rust / WASM community
 - The [Elm](https://elm-lang.org/) team: For creating and standardizing the Elm architecture
 - Denis Kolodin: for creating the inspirational [Yew framework](https://github.com/DenisKolodin/yew)
 - Utkarsh Kukreti, for through his [Draco repo](https://github.com/utkarshkukreti/draco), 
 helping me understand how wasm-bindgen's
 closure system can be used to update state.
 -Tim Robinson, for being very helpful on the [Rust Gitter](https://gitter.im/rust-lang/rust).


## Features to add
 - Router
 - High-level fetch API
 - Virtual DOM optimization 
 - Docs/tutorial website example to replace this readme
 - High-level CSS-grid/Flexbox API ?
 
 ## Bugs to fix
 - Text renders above children instead of below
