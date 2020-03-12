[![crates.io version](https://meritbadge.herokuapp.com/seed)](https://crates.io/crates/seed)
[![crates.io downloads](https://img.shields.io/crates/d/seed.svg)](https://crates.io/crates/seed)
[![docs.rs](https://docs.rs/seed/badge.svg)](https://docs.rs/seed)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

<p align="center">
  <img src="/seed_branding/seed_logo.svg" width="256" title="Seed logo">
</p>

### [Website](https://seed-rs.org) | [Forum](https://seed.discourse.group) | [Chat](https://discord.gg/JHHcHp5)
---
Seed is a front-end Rust framework for creating fast and reliable web apps with an elm-like architecture.

- All the benefits of Rust and macro based syntax.
- Minimal overhead, configuration, and boilerplate.
- Clear documentation made to be accessible regardless of your familiarity with Rust.
- Written without any [unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) code and works with `stable` Rust, no `nightly` required!

---

# Examples
The examples provided in this repository are a good place to get started. They also act as an integration testing suite we run before every commit to ensure there are no unintended breaking changes to the user space. Because of Rust's compile-time checking, testing is that much more robust and changes can be pushed confidently.

Run [examples](examples/) with `cargo make start example_name` from the Seed repository root. Here's our counter example:

```rust
use seed::{prelude::*, *};

// `Model` describes our app state.
type Model = i32;

// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// `view` describes what to display, and can implement the different `Msg`s you define.
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        class!["counter"],
        button![
            model.to_string(),
            ev(Ev::Click, |_| Msg::Increment),
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
```

# Quickstart
If you are proficient in a front-end framework, creating a standalone web app is painless. You'll notice minimal configuration:

```
├── Cargo.toml
├── Makefile.toml
├── index.html
└── src
    └── lib.rs
```

We currently have two template repositories:
- [Quickstart](https://github.com/seed-rs/seed-quickstart)
- [Webpack quickstart](https://github.com/seed-rs/seed-quickstart-webpack)

# Why Use Seed

### Rust
You may prefer writing in Rust and appreciate its benefits, including:
- Rust **safety**.
- Rust **compile-time error, type, and immutability checking**.
- Rust built-in testing.
- Rust speed.
- Cleaner code and less runtime errors.
- Cargo packages.
- Potential pipeline/stack language consistency.

### Developer Experience
Our main focus is on developer experience, the benefits of which are currently:
- Seed has a *batteries-included* approach, meaning less boilerplate and dependencies.
- Macro syntax removes the need for transpiling and integrates naturally and flexibly with the language. This also means all the pains of embedded HTML are avoided; linting, commenting, etc. all work out of the box.
- Built in elm-like architecture, no need for another state manager.
- Perhaps you find JS frameworks and tools difficult to set up.
- Very active development.

See more on our [about](https://seed-rs.org/guide/about) page.

# Why Not Use Seed
- Seed is under rapid development, so there may be breaking changes at times. However, Seed is more than stable enough for personal projects, and production apps are in development.
- Finding Rust/WASM/Seed help outside of [Discord](https://discord.gg/JHHcHp5) or [Discourse](https://seed.discourse.group) may be difficult, as tutorials and guides outside the official ones aren't yet prevalent.
- Seed doesn't have as many existing reusable components that more mature frameworks have (date-pickers, etc.), so you may need to implement them yourself, or adapt them from existing solutions.

# Documentation
- [Quickstart](https://seed-rs.org/guide/quickstart)
- [About](https://seed-rs.org/guide/about)
- [Code Comparison](https://seed-rs.org/guide/code-comparison)
- [Structure](https://seed-rs.org/guide/structure)
- [View Macros](https://seed-rs.org/guide/view)
- [Events](https://seed-rs.org/guide/events)
- [HTTP Requests](https://seed-rs.org/guide/http-requests-and-state)
- [Routing](https://seed-rs.org/guide/routing)
- [JavaScript Interaction](https://seed-rs.org/guide/javascript-interaction)
- [Release and Debugging](https://seed-rs.org/guide/release-and-debugging)
- [Server Integration](https://seed-rs.org/guide/server-integration)
- [Support](https://seed-rs.org/guide/support)
- [Troubleshooting] //TBA

# Resources
### Seed
- [Awesome-seed-rs](https://github.com/seed-rs/awesome-seed-rs): A curated list of resources
- [Seed Realworld](https://github.com/seed-rs/seed-rs-realworld): A detailed realworld example site
- [Engineering Rust Web Applications](https://erwabook.com/intro/): A book describing full-stack Rust web-development, using Seed for the front-end

### Rust
- [Rust Discord](https://discordapp.com/invite/rust-lang)
- [Rust IRC](https://www.irccloud.com/invite?channel=%23%23rust&hostname=chat.freenode.net&port=6697&ssl=1)

# Future
- [Roadmap](https://github.com/seed-rs/seed/milestones)
- New [Rust-only quickstart](https://github.com/MartinKavik/seeder)

# Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md).
