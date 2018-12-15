//! High-level interface for web_sys HTTP requests.
//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! See https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html
//! https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Request.html
//! https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/

//#[macro_use]
//extern crate serde_derive;

use std::collections::HashMap;
use::std::hash::BuildHasher;

use futures::{future, Future};
//use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
//use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures;
use web_sys::{Request, RequestInit, RequestMode, Response};


// todo debuggins
#[derive(Clone, Serialize, Deserialize)]
struct Data{
    val: i32,
    text: String,
}


/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch
}

impl Method {
    fn as_str(&self) -> &str {
        match *self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
        }
    }
}

/// A wrapper over web_sys's fetch api, to simplify code
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub fn fetch<S>(method: Method, url: &str, payload: Option<String>,
//             headers: Option<HashMap<&str, &str>>,
//             cl: impl FnMut(wasm_bindgen::JsValue) -> future::FutureResult) -> js_sys::Promise {
//             headers: Option<HashMap<&str, &str, S>>, cl: impl FnMut(wasm_bindgen::JsValue)) -> js_sys::Promise
             headers: Option<HashMap<&str, &str, S>>) -> js_sys::Promise
//             headers: Option<HashMap<&str, &str>>) -> wasm_bindgen_futures::JsFuture {
//             headers: Option<HashMap<&str, &str>>) -> wasm_bindgen_futures::JsFuture {
    where S: BuildHasher
{
    let mut opts = RequestInit::new();
    opts.method(method.as_str());
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts).unwrap();

    // Set headers:
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Headers.html
    // https://developer.mozilla.org/en-US/docs/Web/API/Headers/set
    if let Some(h) = headers {
        let req_headers = request.headers();
        for (name, value) in &h {
            req_headers.set(&name, &value).unwrap();
        }
    }
//    req_headers.unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = wasm_bindgen_futures::JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: js_sys::Promise| {
            // Convert this other `Promise` into a rust `Future`.
            wasm_bindgen_futures::JsFuture::from(json_value)
        })

//        .and_then(cl);


        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            let data: Data = json.into_serde().unwrap();


            crate::log(data.text.clone());



            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(JsValue::from_serde(&data).unwrap())
        });

    // Convert this Rust `Future` back into a JS `Promise`.

//    future
    wasm_bindgen_futures::future_to_promise(future)
}



//
///// A wrapper over web_sys's fetch api, to simplify code
///// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//pub fn fetch<T>(method: ReqMethod, url: &str, payload: Option<String>,
//         headers: HashMap<String, String>, ex: T) -> T
//    where T: serde::Serialize + serde::Deserialize
//{
//    let mut opts = RequestInit::new();
//    opts.method(method.as_str());
//    opts.mode(RequestMode::Cors);
//
//    let request = Request::new_with_str_and_init(url, &opts).unwrap();
//
//    // Set headers:
//    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Headers.html
//    // https://developer.mozilla.org/en-US/docs/Web/API/Headers/set
//    let req_headers = request.headers();
//    for (name, value) in &headers {
//        req_headers.set(&name, &value);
//    }
////    req_headers.unwrap();
//
//    let window = web_sys::window().unwrap();
//    let request_promise = window.fetch_with_request(&request);
//
//
//    let result: T;
//
//    let future = JsFuture::from(request_promise)
//        .and_then(|resp_value| {
//            // `resp_value` is a `Response` object.
//            assert!(resp_value.is_instance_of::<Response>());
//            let resp: Response = resp_value.dyn_into().unwrap();
//            resp.json()
//        })
//        .and_then(|json_value: js_sys::Promise| {
//            // Convert this other `Promise` into a rust `Future`.
//            JsFuture::from(json_value)
//        })
//        .and_then(|json| {
//            // Use serde to parse the JSON into a struct.
//            result = json.into_serde().unwrap();
//
//            // Send the `Branch` struct back to JS as an `Object`.
////            future::ok(JsValue::from_serde(&branch_info).unwrap())
//        });
//    result
//
//    // Convert this Rust `Future` back into a JS `Promise`.
////    wasm_bindgen_futures::future_to_promise(future)
//}







// todo add these back once you've stabilized your api.
//pub fn get(url: &str, payload: Option<String>, headers: HashMap<String, String>) {
//    fetch(ReqMethod::Post, url, payload, headers)
//}
//
//pub fn post(url: &str, payload: Option<String>, headers: HashMap<String, String>) {
//    fetch(ReqMethod::Post, url, payload, headers)
//}