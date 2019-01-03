//! High-level interface for web_sys HTTP requests.
//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! See https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html
//! https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Request.html
//! https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/
//! https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Response.html

use std::collections::HashMap;
use::std::hash::BuildHasher;
use futures::{future, Future};

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures;
use web_sys;

use serde::Serialize;
use serde_json;

// todo once this is polished, publish as a standalone crate.


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

/// Higher-level wrapper for web_sys::RequestInit.
/// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RequestInit.html#method.mode
#[derive(Clone)]
pub struct RequestOpts {
    // todo: Macro for this?
    pub payload: Option<String>,
    pub headers: HashMap<String, String>,
    pub credentials: HashMap<String, String>,
    pub mode: web_sys::RequestMode,
}

impl RequestOpts {
    pub fn new() -> Self {
        Self {
            payload: None,
            headers: HashMap::new(),
            credentials: HashMap::new(),
            // https://rustwasm.github.io/wasm-bindgen/api/web_sys/enum.RequestMode.html
            mode: web_sys::RequestMode::Cors,
        }
    }
}


/// A wrapper over web_sys's fetch api, to simplify code
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
fn fetch_helper(
    method: Method,
    url: &str,
    request_opts: Option<RequestOpts>,
//) -> Future<Item=js_sys::Promise, Error=JsValue> {
//) -> Future<Item=js_sys::Promise> {
) -> web_sys::Request {

    let mut opts = web_sys::RequestInit::new();
    opts.method(method.as_str());
    opts.mode(web_sys::RequestMode::Cors);  // default

    if let Some(o) = request_opts.clone() {
        if let Some(p) = o.payload {
            opts.body(Some(&JsValue::from_str(&p)));
        }
        opts.mode(o.mode);
    }

    let request = web_sys::Request::new_with_str_and_init(url, &opts.clone())
        .expect("Problem with request");

    if let Some(o) = request_opts {
        // Set headers.
        //  https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Headers.html
        // https://developer.mozilla.org/en-US/docs/Web/API/Headers/set
        for (name, value) in &o.headers {
            request.headers().set(&name, &value).unwrap();
        }
    }
    request
}

/// Allow access to a web_sys::Response inside a callback. Suitable for general use,
/// eg we can get the status code etc. Currently, accessing json from inside this
/// may be difficult.
pub fn fetch(
    method: Method,
    url: &str,
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(web_sys::Response)>
) {
    let request = fetch_helper(method, url, request_opts);
    let window = web_sys::window().expect("Can't find window");

    // We could get the fetch_request_with_init promise directly from the func,
    // but am running into issues with futures etc.
    let request_promise = window.fetch_with_request(&request);
    let f = wasm_bindgen_futures::JsFuture::from(request_promise)

        .and_then(move |resp_value| {
        // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<web_sys::Response>());
        let resp: web_sys::Response = resp_value.dyn_into()
            .expect("Problem casting response as Reponse.");

        callback(resp);
        future::ok(JsValue::null())
    });

    wasm_bindgen_futures::future_to_promise(f);
}

// todo fetch_text?

/// Allow access to a json object, inside a callback.
pub fn fetch_json(
    method: Method,
    url: &str,
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(JsValue)>
) {
    let request = fetch_helper(method, url, request_opts);
    let window = web_sys::window().expect("Can't find window");

    // We could get the fetch_request_with_init promise directly from the func,
    // but am running into issues with futures etc. So, repeated code here between
    // this and fetch.
    let request_promise = window.fetch_with_request(&request);
    let f = wasm_bindgen_futures::JsFuture::from(request_promise)

            .and_then(move |resp_value| {
        // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<web_sys::Response>());
            let resp: web_sys::Response = resp_value.dyn_into()
                .expect("Problem casting response as Reponse.");

            resp.json()
//          resp.text()
        })
        .and_then(|json_value| {
            // Convert this other `Promise` into a rust `Future`.
            wasm_bindgen_futures::JsFuture::from(json_value)
        })
        .and_then(move |v| {
            callback(v);
            future::ok(JsValue::null())
        });

    wasm_bindgen_futures::future_to_promise(f);
}

/// Convenience function for fetch, for get requests.
pub fn get(
    url: &str,
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(web_sys::Response)>)
{

    fetch(Method::Get, url, request_opts, callback)
}

/// Convenience function for fetch for get requests, that exposes json in the callback.
pub fn get_json(
    url: &str,
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(JsValue)>)
{
    fetch_json(Method::Get, url, request_opts, callback)
}

/// A wrapper for fetch that serializes the payload, and allows access to a web_sys::Response
/// struct in its callback.
pub fn post<S: Serialize>(
    url: &str,
    payload: S,
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(web_sys::Response)>
) {
    let serialized_payload = serde_json::to_string(&payload).expect("Problem serializing payload");

    let updated_opts = match request_opts {
        Some(o) => RequestOpts{payload: Some(serialized_payload), ..o},
        None => {
            let mut opts = RequestOpts::new();
            opts.payload = Some(serialized_payload);
            opts
        }
    };
    fetch(Method::Post, url, Some(updated_opts), callback)
}


/// A wrapper for fetch that serializes the payload, and exposes json in the callback.
pub fn post_json<S: Serialize>(
    url: &str,
    payload: S,
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(JsValue)>
) {
    let serialized_payload = serde_json::to_string(&payload).expect("Problem serializing payload");

    let updated_opts = match request_opts {
        Some(o) => RequestOpts{payload: Some(serialized_payload), ..o},
        None => {
            let mut opts = RequestOpts::new();
            opts.payload = Some(serialized_payload);
            opts
        }
    };
    fetch_json(Method::Post, url, Some(updated_opts), callback)
}
