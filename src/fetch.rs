//! High-level interface for web_sys HTTP requests.
//! https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
//! See https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html
//! https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Request.html
//! https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/
//! https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Response.html

use futures::{Future, Poll};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::future_to_promise;
use web_sys;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;

// todo once this is polished, publish as a standalone crate.


/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Debug, Clone, Copy)]
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


#[derive(Debug)]
pub struct Request<'a> {
    url: &'a str,
    init: web_sys::RequestInit,
    headers: Option<web_sys::Headers>,
}

impl<'a> Request<'a> {
    pub fn new(url: &'a str) -> Self {
        Self {
            url,
            init: web_sys::RequestInit::new(),
            headers: None,
        }
    }

    #[inline]
    pub fn method(mut self, val: Method) -> Self {
        self.init.method(val.as_str());
        self
    }

    fn set_header(&mut self, name: &str, val: &str) {
        let headers = self.headers.get_or_insert_with(|| {
            web_sys::Headers::new().expect("Error with creating Headers")
        });

        headers.set(name, val).expect("Error with setting header");
    }

    #[inline]
    pub fn header(mut self, name: &str, val: &str) -> Self {
        self.set_header(name, val);
        self
    }

    #[inline]
    pub fn body(mut self, val: &JsValue) -> Self {
        self.init.body(Some(val));
        self
    }

    fn get_json<A: Serialize>(val: &A) -> JsValue {
        let json = serde_json::to_string(val).expect("Error serializing JSON");
        JsValue::from_str(&json)
    }

    #[inline]
    pub fn body_json<A: Serialize>(self, val: &A) -> Self {
        self.body(&Self::get_json(val))
    }

    #[inline]
    pub fn cache(mut self, val: web_sys::RequestCache) -> Self {
        self.init.cache(val);
        self
    }

    #[inline]
    pub fn credentials(mut self, val: web_sys::RequestCredentials) -> Self {
        self.init.credentials(val);
        self
    }

    #[inline]
    pub fn integrity(mut self, val: &str) -> Self {
        self.init.integrity(val);
        self
    }

    #[inline]
    pub fn mode(mut self, val: web_sys::RequestMode) -> Self {
        self.init.mode(val);
        self
    }

    #[inline]
    pub fn redirect(mut self, val: web_sys::RequestRedirect) -> Self {
        self.init.redirect(val);
        self
    }

    #[inline]
    pub fn referrer(mut self, val: &str) -> Self {
        self.init.referrer(val);
        self
    }

    #[inline]
    pub fn referrer_policy(mut self, val: web_sys::ReferrerPolicy) -> Self {
        self.init.referrer_policy(val);
        self
    }

    // Must be called before make_future
    fn make_controller(&mut self) -> web_sys::AbortController {
        let controller = web_sys::AbortController::new().expect("Error with creating AbortController");

        if let Some(ref headers) = self.headers {
            self.init.headers(headers.as_ref());
        }

        self.init.signal(Some(&controller.signal()));

        controller
    }

    // Must be called after make_controller
    fn make_future(&self) -> impl Future<Item = web_sys::Response, Error = JsValue> {
        let promise = web_sys::window()
            .expect("Can't find window")
            .fetch_with_str_and_init(self.url, &self.init);

        JsFuture::from(promise).map(|x| x.into())
    }

    pub fn fetch(mut self) -> impl Future<Item = web_sys::Response, Error = JsValue> {
        let controller = self.make_controller();
        let future = self.make_future();
        AbortFuture::new(controller, future)
    }

    pub fn fetch_string(mut self) -> impl Future<Item = String, Error = JsValue> {
        let controller = self.make_controller();
        let future = self.make_future();

        // TODO handle error codes like 404
        let future = future
            .and_then(|x| x.text())
            .and_then(JsFuture::from);

        AbortFuture::new(controller, future)
            .map(|x| {
                // TODO avoid copying somehow ?
                x.as_string().expect("Error when converting into string")
            })
    }

    pub fn fetch_json<A: DeserializeOwned>(self) -> impl Future<Item = A, Error = JsValue> {
        self.fetch_string()
            .map(|text| {
                serde_json::from_str(&text).expect("Error deserializing JSON")
            })
    }
}


// This will automatically abort the request when it is dropped
struct AbortFuture<A> {
    controller: web_sys::AbortController,
    future: A,
}

impl<A> AbortFuture<A> {
    #[inline]
    fn new(controller: web_sys::AbortController, future: A) -> Self {
        Self { controller, future }
    }
}

impl<A> Future for AbortFuture<A> where A: Future {
    type Item = A::Item;
    type Error = A::Error;

    #[inline]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

impl<A> Drop for AbortFuture<A> {
    #[inline]
    fn drop(&mut self) {
        self.controller.abort();
    }
}

pub fn spawn_local<F>(future: F) where F: Future<Item = (), Error = JsValue> + 'static {
    future_to_promise(future.map(|_| JsValue::UNDEFINED).map_err(|err| {
        web_sys::console::error_1(&err);
        err
    }));
}
