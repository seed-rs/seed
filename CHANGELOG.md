# Changelog
[unreleased]
- Enabled all additional markdown [extensions](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/struct.Options.html).
- Added `markdown` example.
- Removed `'static` bound from `El` and `Node`.
- [BREAKING] Changed `perform_cmd` and `seed::browser::service::fetch` return type to `T` instead of `Result<T, T>`.
- Added Aria attributes.
- Added example `tea_component`.
- [BREAKING] `UpdateEl<T>` changed to `UpdateEl<Ms>` and `fn update(self, el: &mut T);` to `fn update_el(self, el: &mut El<Ms>);` (#370).
- Added trait `UpdateElForIterator<Ms>`.
- Added support for all `Iterator`s, `Option`, `u32`, `i32`, `usize`, `f64` and references in element creation macros (#365, #128).
- [BREAKING] `String` implements `UpdateEl<T>`. (References are now required for `String` properties, e.g. `div![&model.title]`.)
- Fixed `href` detection to ignore `use` elements (#384).
- Added methods `subscribe`, `subscribe_with_handle`, `perform_cmd_with_handle`, `stream`, `stream_with_handle` and `notify` into `Orders` (#130).
- Added `cmds::timeout`, `stream::interval`, `stream::window_event`, `subs::UrlChanged` and `subs::UrlRequested` (#131).
- Added example `subscribe`.
- Updated example `todomvc` to use subscription `UrlChanged` instead of `routes`.
- [BREAKING] Futures in `perform_cmd` and `perform_g_cmd` are executed immediately.
- Added `App` methods `notify` and `notify_with_notification`.
- [BREAKING] `App` method `process_cmd_and_msg_queue` renamed to `process_effect_queue`.
- [BREAKING] Url change listeners are always active (even if `routes` is not defined).
- Added `cmds`, `streams`, `subs`, `CmdHandle`, `SubHandle` and `StreamHandle` into the Seed's prelude.
- [BREAKING] Removed module `next_tick`.
- Added method `App::start` (alternative to `AppBuilder`) (#376, #382).
- Added trait `GetElement` + included in the `prelude` (alternative to `MountPoint`, used in `AppStart`).
- Derive `Debug` for `ElRef`.
- Added macros `C!` and `IF!` and helper `not` (#375).
- Added trait `ToClasses` + included in the `prelude`.
- Updated `todomvc` example to use `C!`, `IF!`, `matches!` and `App::start`.
- `ev` accepts handlers that return `Msg`, `Option<Msg>` or `()` (#394).
- [BREAKING] `EventHandler::new` accepts only handlers that return `Option<Msg>`.
- [BREAKING] `ev`-like functions and some `Orders` method require `'static` bound for generic types (temporary).
- `Orders::after_next_render` now accepts callbacks that return `Msg`, `Option<Msg>` or `()`.
- Updated examples `update_from_js` and `todomvc`.
- [deprecated] `View` is deprecated in favor of `IntoNodes`.
- [BREAKING] `View` isn't implemented for `El` and `Vec<El>`.
- [BREAKING] `Node::add_listener` renamed to `add_event_handler`.
- Rewritten `README.md`.
- Added new Fetch API module. See [`seed::browser::fetch`](https://docs.rs/seed/0.7.0/seed/browser/fetch/index.html) (#353)
- [deprecated] - `seed::browser::service::fetch` module is deprecated in favor of `seed::browser::fetch`.
- Added example `fetch`.
- Implemented `IntoNodes` for `Option<Node<Msg>>` and `Option<Vec<Node<Msg>>>`.
- Added example `graphql` (#400).
- Implemented `UpdateEl` for `i64` and `u64`.
- Reset properties `checked` and `value` on attribute remove (#405). 
- Added example `custom_elements`.

## v0.6.0
- Implemented `UpdateEl` for `Filter` and `FilterMap`.
- Added method `El::is_custom(&self)`.
- Fixed custom elements patching (#325).
- Removed unnecessary error message for comment nodes.
- [BREAKING] Removed deprecated `update` and `trigger_update_ev`.
- [BREAKING] Removed the remains of lifecycle hooks.
- Fixed `value` and `checked` setting for input elements (a bug in VirtualDOM patch algorithm).
- [BREAKING] Removed unpredictable internal input listeners - Seed will not longer react to external input value changes.
- [BREAKING] Use `EventHandler` instead of `Listener`. (`Listener` is now used as the internal DOM EventListener representation.)
- [deprecated] `raw_ev` is deprecated in favor of `ev`. Functionality is the same.
- Improved performance - rewritten event-handling and other refactors in VirtualDOM.
- Fixed processing of multiple event-handlers (#138).
- Added DOM Element references - see `ElRef` and examples (`canvas`, `user_media` or `todomvc`) (#115).
- Removed `Ms: Clone` restriction as much as possible.
- [BREAKING] Added or changed `Custom` variant from `Custom(String)` to `Custom(Cow<'static, str>)`
  in `Ev`, `Tag`, `At` and `St`. Use function `from` to create custom entities (e.g. `At::from("my-attribute")`) (#208).
- Added macro `nodes!`. It accepts `Node<Msg>` and `Vec<Node<Msg`, returns flattened `Vec<Node<Msg>`.
- Refactored all examples.
- Fixed and rewritten example `todomvc`.
- Renamed `counter` example to `counter_advanced`.
- Renamed `drop` example to `drop_zone`.
- Removed `server_interaction_detailed` example.
- Added a new simpler `counter` example.
- Changed example in the main `README.md`.
- Added flag `#![forbid(unsafe_code)]` so the Seed will be marked as a safe library by the Rust community tools.
- Removed `clone` restriction from the method `Effect::map_msg`.
- Implemented `UpdateEl` for `FlatMap`.
- Adapted to Rust 1.41.0.

## v0.5.1
- [BREAKING] `MessageMapper::map_message` changed to `MessageMapper::map_msg`.
- [BREAKING] `fetch` and `storage` moved to `seed::browser::service::{fetch, storage}`,
but reimported at the lib level. Ie: `seed::fetch`, and `seed::storage`.
- Added support for `Vec<Attr>` and `Vec<Style>` in view macros.
- `App` included in `prelude`.
- [BREAKING] Seed refactored to use `async/.await`. `fetch.rs` docs updated.
- Export `Attrs`, `Style`, `Listener`. ie, can import with `seed::Style` etc.
- Fixed a bug causing the docs not to build.

## v0.5.0
- Added helper `seed::canvas()`, and `seed::canvas_context()` helper functions.
- Fixed `Url` parsing (resolves issue with hash routing).
- [BREAKING] `From<String> for Url` changed to `TryFrom<String> for Url`.
- Fixed jumping cursor in inputs (#158) .
- Added method `orders.after_next_render(Option<RenderTimestampDelta>)` (#207).
- Fixed a bug with back/forward routing to the landing page (#296).
- Deprecated `Init` struct, replacing it with `BeforeMount` and `AfterMount` structs to
better denote state before and after mounting the `App` occurs.
- Added a new function `builder` which replaces `build` as part of deprecating `Init`.
- Added a new function `build_and_start` which replaces `finish` as part of deprecating `Init`.
- Added `IntoInit`and `IntoAfterMount` traits. It is possible to use these
in place of a closure or function to produce the corresponding `Init` and `AfterMount` structs.
- Messages sent from `IntoAfterMount` will now be run after the routing message.
- Added example `app_builder`.
- `events::Listener` is included in `prelude`.
- `()`s  have been replaced with structs - e.g. `GMs = ()` => `GMs = UndefinedGMs`.
- `WindowEvents` alias changed to `WindowEventsFn` for consistency with other `*Fn`.
- Commented builder and helper methods.

## v0.4.2
- Added an `Init` struct, which can help with initial routing (Breaking)
- The `routes` function now returns an `Option<Msg>` (Breaking)
- Updated `Tag::from()` to accept more input types
- `style!` now accepts also `Option<impl ToString>`
- Fixed a bug affecting element render order
- Added a `hashchange` listener
- Improved error-handling
- Tweaked bootstrap order so that `main_el_vdom` is initialized first (internal)
- Macro `custom!` checks if you set tag, and panics when you forget
- Fixed a bug with children being absent from cloned elements
- Improved debugging
- Fixed a namespace bug with adding children to `Svg` elements
- Fixed a bug affecting Safari
- Added `seed::html_document()` and `seed::cookies` convenience functions

## v0.4.1
- Added more SVG `At` variants
- Added the `St` enum, for style keys; similar to `At`
- Improved ergonomics of `add_child`, `add_attr`, `add_class`,
`add_style`, `replace_text`, and `add_text`, `Node` methods

## v0.4.0
- `ElContainer`, imported in prelude, renamed to `View`. (Breaking)
- Internal refactor of `El`: Now wrapped in `Node`, along with
`Empty` and `Text`. Creation macros return `Node(Element)`. (Breaking)
- Changed the way special attributes like `disabled`, `autofocus`, and
`checked` are handled (Breaking)
- `MessageMapper` now accepts closures
- `Orders` is a trait now instead of a struct. (Breaking)
- Significant changes to MessageMapper
- Orders has new methods, `clone_app` and `msg_mapper` which can allow access to app instance.
- Added more SVG element macros
- Several minor bux fixes
- Examples updated to reflect these changes
- Improvements to Fetch API, especially regarding error handling
and deserialization

## v0.3.7
- `routes` now accepts `Url` instead of `&Url` (Breaking)
- Improvements to fetch API
- Added `raw!`, `md!`, and `plain!` macros that alias `El::from_html`, `El::from_markdown`,
and `El::new_text` respectively
- `Attrs!` and `Style!` macros can now use commas and whitespace as separators,
in addition to semicolons
- Fixed typos in a few attributes (Breaking)
- Fixed a bug where an HTML namespace was applied to raw html/markdown elements
- New conditional syntax added in `class!` macro, similar to `Elm`'s `classList`
- `Listener` now implements `MessageMapper`
- `El methods` `add_child`, `add_style`, `add_attr`, and `set_text` now return the elements,
allowing chaining
- Fixed a bug with `set_text`. Renamed to `replace_text`. Added `add_text`, which adds
a text node, but doesn't remove existing ones. Added `add_class`. (Breaking)


## v0.3.6
- Fetch module and API heavily changed (breaking)
- Added support for `request​Animation​Frame`, which improves render performance,
especially for animations
- Styles no longer implicitly add `px`. Added `unit!` macro in its place
- `Map` can now be used directly in elements, without needing to annotate type and collect
(ie for child `Elements`, and `Listener`s)
- Significant changes to MessageMapper
- Orders hs new methods, `clone_app` and `msg_mapper` that allow access to app instance.
- Fixed a bug where `empty` elements at the top-level were rendering in the wrong order
- Added an `empty!` macro, which is similar to `seed::empty`
- Attributes and style now retain order

## v0.3.5
- Fixed a bug where view functions returning `Vec<El>` weren't rendering properly
- Fixed a typo with the `viewBox` attribute

## v0.3.4
- The `update` fn now accepts a (new) `Orders` struct, and returns nothing. Renders occur implicitly,
with the option to skip rendering, update with an additional message, or perform an asynchronous
action. (Breaking)
- `.mount()` now accepts elements. Deprecated `.mount_el()`
- The `log` function and macro now support items which implement `Debug`
- Removed deprecated `routing::push_path` function (breaking)

## v0.3.3
- Added `seed::update` function, which allows custom events, and updates from JS

## v0.3.2
- Top level view functions can now return `Vec<El<Ms>>`, `El<Ms>`, or something else implementing
the new ElContainer trait

## v0.3.1
- Top level view functions now return `Vec<El<Ms>>` instead of `El<Ms>`, mounted directly to
 the mount point. (Breaking)
- `push_route()` can now accept a `Vec<&str>`, depreciating `push_path()`
- Fixed a bug where window events couldn't be enabled on initialization

## v0.3.0
- `update` function now takes a mutable ref of the model. (Breaking)
- `Update` (update's return type) is now a struct. (Breaking)
- Async, etc events are now handled through messages, instead of passing `App`
through the view func. (Breaking)
- Fixed some bugs with empty elements
- Internal code cleanup
- Added commented-out release command to example build files
- Added more tests

## v0.2.10
- Routing can be triggered by clicking any element containing a `Href` attribute
with value as a relative link
- Internal links no longer trigger a page refresh
- Models no longer need to implement `Clone`
- Fixed a bug introduced in 0.2.9 for `select` elements

## v0.2.9
- Added a `RenderThen` option to `Update`, which allows chaining update messages
- Added a `.model` method to `Update`, allowing for cleaner recursion in updates
- Improved controlled-component (sync fields with model) logic

## v0.2.8
- Reflowed `El::from_html` and `El::from_markdown` to return `Vec`s of `El`s, instead of wrapping
them in a single span.
- Improved support for SVG and namespaces
- Added `set_timeout` wrapper

## v0.2.7
- Fixed a bug where `line!` macro interfered with builtin
- Fixed a bug with routing search (ie `?`)

## v0.2.6
- Fixed a bug where children would render out-of-order
- Improved vdom diffing logic

## v0.2.5
- Attributes and Events now can use `At` and `Ev` enums
- Routing overhauled; modelled after react-reason. Cleaner syntax, and more flexible
- Input, Textarea, and Select elements are now "controlled" - they always
stay in sync with the model
- index.html file updated in examples and quickstart to use relative paths,
which fixes landing-page routing

## v0.2.4
- Changed render func to use a new pattern (Breaking)
- Default mount point added: \"app\" for element id
- View func now takes a ref to the model instead of the model itself
- Routing refactored; now works dynamically
- Update function now returns an enum that returns Render or Skip,
to allow conditional rendering (Breaking)
- Elements can now store more than 1 text node

## V0.2.3
- Fixed a bug where initially-empty text won't update
- Added more tests
- Exposed web_sys Document and Window in top level of Seed create, with .expect
- Modified build scripts to keep the wasm output name fixed at 'package', simplifying example/quickstart renames
- Tests now work in Windows due to update in wasm-pack

## V0.2.2
- Overhaul of fetch module
- Added server-integration example

## V0.2.1
- Added support for custom tags
- Added `class!` and `id!` convenience macros for setting style

## v0.2.0

- Added high-level fetch api
- Added routing
- Added element lifecycles (did_mount, did_update, will_unmount)
- Added support for updating state outside events
- Added server_interaction, and homepage (this site) examples

## v0.1.0

- Initial release
