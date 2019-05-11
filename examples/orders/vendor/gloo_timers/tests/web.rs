//! Test suite for the Web and headless browsers.

#![cfg(all(target_arch = "wasm32", feature = "futures"))]

use futures_rs::{
    prelude::*,
    sync::{mpsc, oneshot},
};
use gloo_timers::{
    callback::{Interval, Timeout},
    future::{IntervalStream, TimeoutFuture},
};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(async)]
fn timeout() -> impl Future<Item = (), Error = wasm_bindgen::JsValue> {
    let (sender, receiver) = oneshot::channel();
    Timeout::new(1, || sender.send(()).unwrap()).forget();
    receiver.map_err(|e| e.to_string().into())
}

#[wasm_bindgen_test(async)]
fn timeout_cancel() -> impl Future<Item = (), Error = wasm_bindgen::JsValue> {
    let cell = Rc::new(Cell::new(false));

    let t = Timeout::new(1, {
        let cell = cell.clone();
        move || {
            cell.set(true);
            panic!("should have been cancelled");
        }
    });
    t.cancel();

    let (sender, receiver) = oneshot::channel();

    Timeout::new(2, move || {
        sender.send(()).unwrap();
        assert_eq!(cell.get(), false);
    })
    .forget();

    receiver.map_err(|e| e.to_string().into())
}

#[wasm_bindgen_test(async)]
fn timeout_future() -> impl Future<Item = (), Error = wasm_bindgen::JsValue> {
    TimeoutFuture::new(1).map_err(|_| "impossible".into())
}

#[wasm_bindgen_test(async)]
fn timeout_future_cancel() -> impl Future<Item = (), Error = wasm_bindgen::JsValue> {
    let cell = Rc::new(Cell::new(false));

    let a = TimeoutFuture::new(1).map({
        let cell = cell.clone();
        move |_| {
            assert_eq!(cell.get(), false);
            1
        }
    });

    let b = TimeoutFuture::new(2).map({
        let cell = cell.clone();
        move |_| {
            cell.set(true);
            2
        }
    });

    a.select(b)
        .map_err(|_| "impossible".into())
        .and_then(|(who, other)| {
            assert_eq!(who, 1);

            // Drop `b` so that its timer is canceled.
            drop(other);

            TimeoutFuture::new(3).map_err(|_| "impossible".into())
        })
        .map(move |_| {
            // We should never have fired `b`'s timer.
            assert_eq!(cell.get(), false);
        })
}

#[wasm_bindgen_test(async)]
fn interval() -> impl Future<Item = (), Error = wasm_bindgen::JsValue> {
    let (mut sender, receiver) = mpsc::channel(1);
    let i = Interval::new(1, move || {
        if !sender.is_closed() {
            sender.try_send(()).unwrap()
        }
    });

    receiver
        .take(5)
        .map_err(|_| "impossible".into())
        .collect()
        .and_then(|results| {
            drop(i);
            assert_eq!(results.len(), 5);
            Ok(())
        })
}

#[wasm_bindgen_test(async)]
fn interval_cancel() -> impl Future<Item = (), Error = wasm_bindgen::JsValue> {
    let (mut i_sender, i_receiver) = mpsc::channel(1);

    let i = Interval::new(1, move || i_sender.try_send(()).unwrap());
    i.cancel();

    // This keeps us live for long enough that if any erroneous Interval callbacks fired, we'll have seen them.
    let (mut sender, receiver) = mpsc::channel(1);
    Timeout::new(50, move || {
        sender.try_send(()).unwrap();
    })
    .forget();

    receiver
        .merge(i_receiver)
        .take(2)
        .map_err(|_| "impossible".into())
        .collect()
        .and_then(|results| {
            // Should only be 1 item - and that's from the timeout. Anything more means interval spuriously fired.
            assert_eq!(results.len(), 1);
            Ok(())
        })
}

#[wasm_bindgen_test(async)]
fn interval_stream() -> impl Future<Item = (), Error = wasm_bindgen::JsValue> {
    IntervalStream::new(1)
        .take(5)
        .map_err(|_| "impossible".into())
        .collect()
        .and_then(|results| {
            assert_eq!(results.len(), 5);
            Ok(())
        })
}
