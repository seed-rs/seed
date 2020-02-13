//! A simple, cliché example demonstrating structure and syntax.
//! Inspired by [Elm example](https://guide.elm-lang.org/architecture/buttons.html).

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};
use seed::browser::fetch::*;
use futures::future::FutureExt;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    foo: Option<Foo>,
}

// ------ ------
//    Update
// ------ ------

#[derive(serde::Deserialize)]
struct Foo {
    bar: usize,
}

#[derive(serde::Deserialize, Debug)]
struct FooValidationError {
    reason: String,
}

#[derive(Debug)]
enum FooError {
    ServerError,
    SerdeError,
    StatusError,
    ValidationError(FooValidationError),
}

enum Msg {
    SendRequest,
    Fetched(Result<Foo, FooError>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            orders.skip().perform_cmd(fetch_foo().map(Msg::Fetched));
        },
        Msg::Fetched(Ok(foo)) => {
            model.foo = Some(foo);
        },
        Msg::Fetched(Err(FooError::ValidationError(FooValidationError{reason}))) => {
            log!("Invalid foo: {}", reason);
        },
        Msg::Fetched(Err(err)) => {
            error!("It's dead Jim");
        }
    }
}

async fn fetch_foo() -> Result<Foo, FooError> {
    let response = fetch("/foo.json").await.map_err(|_| FooError::ServerError)?;

    match response.status() {
        Status{code: 200, ..} => {
            response.json::<Foo>().await.map_err(|_| FooError::SerdeError)
        },
        Status{code: 418, ..} => {
            let validation_error = response.json::<FooValidationError>().await.map_err(|_| FooError::SerdeError)?;
            Err(FooError::ValidationError(validation_error))
        }
        _ => Err(FooError::StatusError),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::SendRequest), "Fetch"],
        if let Some(foo) = &model.foo {
            div![format!("Bar: {}", foo.bar)]
        } else {
            empty![]
        }
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
