## Examples
For specific details see corresponding READMEs.

Consider first looking at [Counter](./counter).

Most of the examples can be run by simply:
```sh
cd examples/$EXAMPLE_DIR
cargo make start
```

### [Homepage repo](https://github.com/seed-rs/seed-rs.org)
The Seed homepage, also serving as an example. Includes simple
interactions, markdown elements, routing, and view markup.

### [Animation](animation)
How to make a basic animation with random generators.

### [Bunnies](bunnies)
Intended as a demo of [Shipyard](https://github.com/leudz/shipyard) (Entity Component System) integration.

### [Canvas](canvas)
How to make a canvas element and use `ElRef`s.

## [Component builder](component_builder)
How to write reusable views / components with builder pattern.

### [Counter](counter)
Intended as a demo of basic functionality.

### [Custom Elements](custom_elements)
How to create and use custom elements.

### [Drop Zone](drop_zone)
How to create a drop-zone.

### [Element Key](el_key)
How to control a DOM update using element keys and empty nodes.

### [I18N](i18n)

How to support multiple languages in your web app based on [Fluent][url_project_fluent]. 
Includes a language selector, some sample text and [FTL strings][url_ftl_syntax_guide] 
demonstrating the simplicity and power of [Seed][url_project_seed] 
powered by [Fluent's Rust crate][url_crate_fluent].

[url_project_fluent]: https://projectfluent.org/
[url_crate_fluent]: https://docs.rs/fluent/
[url_ftl_syntax_guide]: https://projectfluent.org/fluent/guide/
[url_project_seed]: https://seed-rs.org/

### [Markdown](markdown)
How to render markdown.

### [Pages](pages)
How to create and browse multiple pages in your app.

### [Pages with hash routing](pages_hash_routing)
How to create and browse multiple pages in your app.
This example uses hash routing.

### [Pages that keep their state](pages_keep_state)
How to create and browse multiple pages in your app.
Pages keep their state.

### [Subscribe](subscribe)
How to create and use subscriptions, streams, notifications and commands.

### [TEA component](tea_component)
How to write a component in The Elm architecture.
You'll also learn how to pass messages to the parent component.

### [Fetch](fetch)
How to make HTTP request using Fetch API.

### [Todo MVC](todomvc)
Classic TodoMVC example with Local Storage.

### [Unsaved changes](unsaved_changes)
How to prevent navigating away when there are unsaved changes on the website.

### [Update from JS](update_from_js)
How to trigger `update` function from Javascript world.
You'll also see how to call JS functions from Rust.

### [Url](url)
Intended as a demo of Url functions and browser navigation.

### [UserMedia](user_media)
How to show your webcam output in `video` element.

### [Window Events](window_events)
A demonstration of event-handlers attached to the `window`.

## Server
Backend server integration & interaction examples.

### [Auth](auth)
How to implement login / logout.

### [GraphQL](graphql)
How to communicate with a GraphQL backend.

### [Integration](server_integration)
Example of a workspace with [Actix](https://actix.rs/) server.

### [JWT](jwt)
How to implement user sessions with JSON web tokens.

### [Websocket Chat](websocket)
Example of communicating with a server using Websockets.
