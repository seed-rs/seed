//! The Request interface of the Fetch API represents a resource request.
//!
//! See [developer.mozilla.org/en-US/docs/Web/API/Request](https://developer.mozilla.org/en-US/docs/Web/API/Request)

use super::{FetchError, Method, Result};
use gloo_timers::callback::Timeout;
use serde::Serialize;
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::JsValue;

/// Its methods configure the request, and handle the response. Many of them return the original
/// struct, and are intended to be used chained together.
#[derive(Debug, Clone, Default)]
pub struct Request {
    url: Cow<'static, str>,
    headers: Headers,
    method: Method,
    body: Option<JsValue>,
    cache: Option<web_sys::RequestCache>,
    credentials: Option<web_sys::RequestCredentials>,
    integrity: Option<String>,
    mode: Option<web_sys::RequestMode>,
    redirect: Option<web_sys::RequestRedirect>,
    referrer: Option<String>,
    referrer_policy: Option<web_sys::ReferrerPolicy>,
    timeout: Option<u32>,
    // controller: RequestController,
}

impl Request {
    /// Create new request based on the provided url.
    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            url: url.into(),
            ..Self::default()
        }
    }

    // TODO: remove when https://github.com/rust-lang/rust-clippy/issues/4979 will be fixed
    #[allow(clippy::missing_const_for_fn)]
    /// Set headers for this request.
    /// It will replace any existing headers.
    pub fn headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }

    /// Set specific header.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Set HTTP method. Default method is `GET`.
    pub const fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Set request body to provided `JsValue`. Consider using `json` or `text` methods instead.
    ///
    /// Note that a request using the GET or HEAD method cannot have a body.
    pub fn body(mut self, body: JsValue) -> Self {
        self.body = Some(body);

        #[cfg(debug_assertions)]
        match self.method {
            Method::Get | Method::Head => {
                error!("GET and HEAD requests shoudn't have a body");
            }
            _ => {}
        }

        self
    }

    /// Set request body by JSON encoding provided data.
    /// It will also set `Content-Type` header to `application/json; charset=utf-8`.
    ///
    /// # Errors
    ///
    /// This method can fail if JSON serialization fail. It will then
    /// return `FetchError::SerdeError`.
    pub fn json<T: Serialize + ?Sized>(mut self, data: &T) -> Result<Self> {
        self.headers.insert(
            "Content-Type".to_owned(),
            "application/json; charset=utf-8".to_owned(),
        );
        let body = serde_json::to_string(data).map_err(FetchError::SerdeError)?;
        self.body = Some(body.into());
        Ok(self)
    }

    /// Set request body to a provided string.
    /// It will also set `Content-Type` header to `text/plain; charset=utf-8`.
    pub fn text(mut self, text: impl AsRef<str>) -> Self {
        self.headers.insert(
            "Content-Type".to_owned(),
            "text/plain; charset=utf-8".to_owned(),
        );
        self.body = Some(JsValue::from(text.as_ref()));
        self
    }

    /// Set request mode.
    /// It can either be `cors`, `no-cors`, `same-origin`, or `navigate`.
    /// The default is `cors`.
    pub const fn mode(mut self, mode: web_sys::RequestMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set request credentials.
    /// It can either be `omit`, `same-origin`, or `include`.
    /// The default is `same-origin`.
    pub const fn credentials(mut self, credentials: web_sys::RequestCredentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Set request cache mode.
    pub const fn cache(mut self, cache: web_sys::RequestCache) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Set request redirect mode.
    /// It can either be `follow`, `error`, or `manual`.
    /// The default is `follow`.
    pub const fn redirect(mut self, redirect: web_sys::RequestRedirect) -> Self {
        self.redirect = Some(redirect);
        self
    }

    /// Set request referrer.
    /// It can be either `referrer`, `client`, or a `URL`.
    /// The default is `about:client`.
    pub fn referrer(mut self, referrer: String) -> Self {
        self.referrer = Some(referrer);
        self
    }

    /// Set request referrer policy.
    pub const fn referrer_policy(mut self, referrer_policy: web_sys::ReferrerPolicy) -> Self {
        self.referrer_policy = Some(referrer_policy);
        self
    }

    /// Set request subresource integrity.
    pub fn integrity(mut self, integrity: String) -> Self {
        self.integrity = Some(integrity);
        self
    }
}

impl From<Request> for web_sys::Request {
    fn from(request: Request) -> Self {
        let mut init = web_sys::RequestInit::new();

        // headers
        let headers = web_sys::Headers::new().expect("fetch: cannot create headers");
        for (name, value) in &request.headers {
            headers
                .append(name.as_str(), value.as_str())
                .expect("fetch: cannot create header")
        }
        init.headers(&headers);

        // method
        init.method(request.method.as_str());

        // body
        if let Some(body) = &request.body {
            init.body(Some(body));
        }

        // cache
        if let Some(cache) = request.cache {
            init.cache(cache);
        }

        // credentials
        if let Some(credentials) = request.credentials {
            init.credentials(credentials);
        }

        // integrity
        if let Some(integrity) = &request.integrity {
            init.integrity(integrity.as_str());
        }

        // mode
        if let Some(mode) = request.mode {
            init.mode(mode);
        }

        // redirect
        if let Some(redirect) = request.redirect {
            init.redirect(redirect);
        }

        // referrer
        if let Some(referrer) = &request.referrer {
            init.referrer(referrer.as_str());
        }

        // referrer_policy
        if let Some(referrer_policy) = request.referrer_policy {
            init.referrer_policy(referrer_policy);
        }

        // TODO: fixme
        // // timeout
        // if let Some(timeout) = &request.timeout {
        //     let abort_controller = request.controller.clone();
        //     *request.controller.timeout_handle.borrow_mut() = Some(
        //         // abort request on timeout
        //         Timeout::new(*timeout, move || abort_controller.abort()),
        //     );
        // }

        // // controller
        // // https://developer.mozilla.org/en-US/docs/Web/API/AbortController/signal
        // init.signal(Some(&request.controller.abort_controller.signal()));

        // It seems that the only reason why Request constructor can
        // fail is when Url contains credentials.  I assume that this
        // use case should be extremely rare, so to make api a bit
        // simplier let's just unwrap it here.
        //
        // See https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#Errors
        web_sys::Request::new_with_str_and_init(&request.url, &init)
            .expect("fetch: Cannot create request")
    }
}

// TODO cows?
/// Request headers.
pub type Headers = HashMap<String, String>;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
/// It allows to abort request or disable request's timeout.
/// You can get it by calling method `Request.controller`.
pub struct RequestController {
    abort_controller: Rc<web_sys::AbortController>,
    timeout_handle: Rc<RefCell<Option<Timeout>>>,
}

impl RequestController {
    /// Abort request and disable request's timeout.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)
    pub fn abort(&self) {
        // Cancel timeout by dropping it.
        self.timeout_handle.replace(None);
        self.abort_controller.abort();
    }
    /// Disable request's timeout.
    ///
    /// # Errors
    ///
    /// Will return error if timeout is already disabled.
    pub fn disable_timeout(&self) -> std::result::Result<(), &'static str> {
        // Cancel timeout by dropping it.
        match self.timeout_handle.replace(None) {
            Some(_) => Ok(()),
            None => Err("disable_timeout: already disabled"),
        }
    }
}

impl Default for RequestController {
    fn default() -> Self {
        Self {
            abort_controller: Rc::new(
                web_sys::AbortController::new().expect("fetch: create AbortController - failed"),
            ),
            timeout_handle: Rc::new(RefCell::new(None)),
        }
    }
}
