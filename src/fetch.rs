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

/// Higher-level wrapper for web_sys::RequestInit.
/// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RequestInit.html#method.mode
pub struct RequestOpts {
    // todo: Macro for this?
    pub payload: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub credentials: Option<HashMap<String, String>>,
//    mode: web_sys::RequestMode
}

// todo once this is polished, publish as a standalone crate.

// todo: We want to expose the web_sys::Response object, not just response.json().

/// A wrapper over web_sys's fetch api, to simplify code
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub fn fetch(
    method: Method,
    url: &str,
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(JsValue)>)
{

    // todo let user pass integrity, headers, credientials etc as a wrapped
    // web_sys::RequestInit.
    let mut opts = web_sys::RequestInit::new();
    opts.method(method.as_str());
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/enum.RequestMode.html
    // We get a CORS error without this setting.
//    opts.mode(web_sys::RequestMode::NoCors);
    opts.mode(web_sys::RequestMode::Cors);

    if let Some(o) = request_opts {
        if let Some(p) = o.payload {
            let mut payload_str = String::from("{");
            for (key, val) in &p {
                payload_str += &format!("\"{}\": \"{}\", ", key, val);
            }
            payload_str.truncate(payload_str.len() - 2);  // Remove trailing command space.
            payload_str += "}";
            crate::log(format!("{:?}", &payload_str));

            opts.body(Some(&JsValue::from_str(&payload_str)));
        }
    }

    let request = web_sys::Request::new_with_str_and_init(url, &opts)
        .expect("Problem with request");


    // Set headers:
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Headers.html
    // https://developer.mozilla.org/en-US/docs/Web/API/Headers/set
//    if let Some(h) = request_opts.headers {
////        let req_headers = request.headers();
//        for (name, value) in &h {
//            crate::log(format!("{:?} {:?}", &name, &value));
//            request.headers().set(&name, &value).unwrap();
//        }
//
//    }
//    request.headers().set("Content-Type", "application/json;charset=UTF-8").unwrap();
//    request.headers().set("Accept", "application/vnd.github.v3+json").unwrap();
//    request.headers().set("Accept-Language", "en-us").unwrap();

    crate::log(format!("CT: {:?}", request.headers().get("Content-Type").unwrap()));
    crate::log(format!("A: {:?}", request.headers().get("Accept").unwrap()));
    crate::log(format!("AL: {:?}", request.headers().get("Accept-Language").unwrap()));



    let window = web_sys::window().expect("Can't find window");
    let request_promise = window.fetch_with_request(&request);

    let f = wasm_bindgen_futures::JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
//            assert!(resp_value.is_instance_of::<web_sys::Response>());  // todo wtf is this getting an error about El?
            let resp: web_sys::Response = resp_value.dyn_into()
                .expect("Problem casting response as Reponse.");

//            resp
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
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(JsValue)>)
{

    fetch(Method::Get, url, request_opts, callback)
}

pub fn post(
    url: &str,
    request_opts: Option<RequestOpts>,
    callback: Box<Fn(JsValue)>)
{
    fetch(Method::Post, url, request_opts, callback)
}