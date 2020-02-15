//! The Request interface of the Fetch API represents a resource request.
//!
//! See https://developer.mozilla.org/en-US/docs/Web/API/Request

use super::Method;
use gloo_timers::callback::Timeout;
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::JsValue;

/// Its methods configure the request, and handle the response. Many of them return the original
/// struct, and are intended to be used chained together.
#[derive(Debug, Clone, Default)]
pub struct Request {
    url: Cow<'static, str>,
    headers: HashMap<String, String>,
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
    pub fn new(url: &'static str) -> Self {
        Self {
            url: Cow::from(url),
            ..Self::default()
        }
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
    pub fn disable_timeout(&self) -> Result<(), &'static str> {
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
