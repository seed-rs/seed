# Intersection Observer Example

This example uses the [Intersection Observer API](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API) through web-sys.

The intersection observer is created. We tell it to watch the red box by giving the element reference to the observer.
When the red box is entirely in view, the label "Is completely visible" will display a value of `true`.

To run, use the following commands from the `intersection_observer` directory.

Using `trunk`:
```
trunk serve
```

Using `cargo-make`:
```
cargo make start
```