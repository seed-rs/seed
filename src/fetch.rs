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
    pub fn as_str(&self) -> &str { // todo pub is temp
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
pub fn fetch(
    method: Method,
    url: &str,
    payload: Option<HashMap<&str, &str>>,
    headers: Option<HashMap<&str, &str>>,
    callback: Box<Fn(JsValue)>)
{
    let mut opts = web_sys::RequestInit::new();
    opts.method(method.as_str());
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/enum.RequestMode.html
    // We get a CORS error without this setting.
//    opts.mode(web_sys::RequestMode::NoCors);
    opts.mode(web_sys::RequestMode::Cors);

    let request = web_sys::Request::new_with_str_and_init(url, &opts)
        .expect("Problem with request");

    // Set headers:
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Headers.html
    // https://developer.mozilla.org/en-US/docs/Web/API/Headers/set
    if let Some(h) = headers {
        let req_headers = request.headers();
        for (name, value) in &h {
            req_headers.set(&name, &value).unwrap();
        }
    }

    let window = web_sys::window().expect("Can't find window");
    let request_promise = window.fetch_with_request(&request);

    let f = wasm_bindgen_futures::JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
//            assert!(resp_value.is_instance_of::<web_sys::Response>());  // todo wtf is this getting an error about El?
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

pub fn get(
    url: &str,
    payload: Option<HashMap<&str, &str>>,
    headers: Option<HashMap<&str, &str>>,
    callback: Box<Fn(JsValue)>)
{

    fetch(Method::Get, url, payload, headers, callback)
}

pub fn post(
    url: &str,
    payload: Option<HashMap<&str, &str>>,
    headers: Option<HashMap<&str, &str>>,
    callback: Box<Fn(JsValue)>)
{
    fetch(Method::Post, url, payload, headers, callback)
}