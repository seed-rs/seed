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

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}


/// A wrapper over web_sys's fetch api, to simplify code
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub fn fetch<S>(method: Method, url: &str, payload: Option<String>,
//         headers: Option<HashMap<&str, &str, S>>) -> js_sys::Promise
//         headers: Option<HashMap<&str, &str, S>>)
//         headers: Option<HashMap<&str, &str, S>>) -> impl Future<Item = JsValue>
         headers: Option<HashMap<&str, &str, S>>, cb: Box<Fn(String)>)
    where S: BuildHasher
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

    let future = wasm_bindgen_futures::JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<web_sys::Response>());
            let resp: web_sys::Response = resp_value.dyn_into()
                .expect("Problem casting response as Reponse.");

//            resp.json()
            resp.text()
        })
        .and_then(|json_value: js_sys::Promise| {
            // Convert this other `Promise` into a rust `Future`.
            wasm_bindgen_futures::JsFuture::from(json_value)
//            future::ok(temp)
        })

        // todo ideally, here is where we'd like to split.

        .and_then(move |json| {
//            // Use serde to parse the JSON into a struct.
            cb(json.as_string().expect("Problem converting JSON to String."));
            future::ok(JsValue::null())
//
        });
    wasm_bindgen_futures::future_to_promise(future);
}


pub fn get<S>(url: &str, payload: Option<String>,
         headers: Option<HashMap<&str, &str, S>>, cb: Box<Fn(String)>)
    where S: BuildHasher
{
    fetch(Method::Get, url, payload, headers, cb)
}

pub fn post<S>(url: &str, payload: Option<String>,
         headers: Option<HashMap<&str, &str, S>>, cb: Box<Fn(String)>)
    where S: BuildHasher
{
    fetch(Method::Post, url, payload, headers, cb)
}