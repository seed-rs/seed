//! The Request of the Fetch API.

use super::form_data::FormData;
use super::{fetch, FetchError, Header, Headers, Method, Response, Result};
use crate::browser::Url;
use gloo_timers::callback::Timeout;
use js_sys::Uint8Array;
use serde::Serialize;
use serde_wasm_bindgen as swb;
use std::{borrow::Cow, cell::RefCell, rc::Rc};
use wasm_bindgen::JsValue;

/// Its methods configure the request, and handle the response. Many of them return the original
/// struct, and are intended to be used chained together.
#[derive(Debug, Clone, Default)]
pub struct Request<'a> {
    url: Cow<'a, str>,
    headers: Headers<'a>,
    method: Method,
    body: Option<Cow<'a, JsValue>>,
    cache: Option<web_sys::RequestCache>,
    credentials: Option<web_sys::RequestCredentials>,
    integrity: Option<String>,
    mode: Option<web_sys::RequestMode>,
    redirect: Option<web_sys::RequestRedirect>,
    referrer: Option<String>,
    referrer_policy: Option<web_sys::ReferrerPolicy>,
    timeout: Option<u32>,
    controller: RequestController,
}

impl<'a> Request<'a> {
    /// Create new request based on the provided url.
    ///
    /// To get a [`Response`](./struct.Response.html) you need to pass
    /// `Request` to the [`fetch`](./fn.fetch.html) function.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request)
    pub fn new(url: impl Into<Cow<'a, str>>) -> Self {
        Self {
            url: url.into(),
            ..Self::default()
        }
    }

    // TODO: remove when https://github.com/rust-lang/rust-clippy/issues/4979 will be fixed
    #[allow(clippy::missing_const_for_fn)]
    /// Set headers for this request.
    /// It will replace any existing headers.
    pub fn headers(mut self, headers: Headers<'a>) -> Self {
        self.headers = headers;
        self
    }

    /// Set specific header.
    pub fn header(mut self, header: Header<'a>) -> Self {
        self.headers.set(header);
        self
    }

    /// Set HTTP method. Default method is `GET`.
    pub const fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Set request body to provided `JsValue`. Consider using `json`, `text`, or `bytes` methods instead.
    ///
    /// ## Panics
    /// This method will panic when request method is GET or HEAD.
    pub fn body(mut self, body: &'a JsValue) -> Self {
        self.body = Some(Cow::Borrowed(body));

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
        let body = swb::to_value(data)?;
        self.body = Some(Cow::Owned(body));
        Ok(self.header(Header::content_type("application/json; charset=utf-8")))
    }

    /// Set request body to a provided string.
    /// It will also set `Content-Type` header to `text/plain; charset=utf-8`.
    pub fn text(mut self, text: impl AsRef<str>) -> Self {
        self.body = Some(Cow::Owned(JsValue::from(text.as_ref())));
        self.header(Header::content_type("text/plain; charset=utf-8"))
    }

    /// Set request body to the provided bytes.
    /// It will also set `Content-Type` header to `application/octet-stream`.
    pub fn bytes(mut self, bytes: impl AsRef<[u8]>) -> Self {
        self.body = Some(Cow::Owned(Uint8Array::from(bytes.as_ref()).into()));
        self.header(Header::content_type("application/octet-stream"))
    }

    /// Set request body to the provided form data object.
    /// It will also set `Content-Type` header to `multipart/form-data`.
    pub fn form_data(mut self, form_data: FormData) -> Self {
        self.body = Some(Cow::Owned(form_data.into()));
        self
    }

    /// Set request mode.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/mode)
    pub const fn mode(mut self, mode: web_sys::RequestMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Set request credentials.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)
    pub const fn credentials(mut self, credentials: web_sys::RequestCredentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Set request cache mode.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)
    pub const fn cache(mut self, cache: web_sys::RequestCache) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Set request redirect mode.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)
    pub const fn redirect(mut self, redirect: web_sys::RequestRedirect) -> Self {
        self.redirect = Some(redirect);
        self
    }

    /// Set request referrer.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)
    pub fn referrer(mut self, referrer: &impl ToString) -> Self {
        self.referrer = Some(referrer.to_string());
        self
    }

    /// Set request referrer policy.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)
    pub const fn referrer_policy(mut self, referrer_policy: web_sys::ReferrerPolicy) -> Self {
        self.referrer_policy = Some(referrer_policy);
        self
    }

    /// Set request subresource integrity.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity)
    pub fn integrity(mut self, integrity: &impl ToString) -> Self {
        self.integrity = Some(integrity.to_string());
        self
    }

    /// Set request timeout in milliseconds.
    pub const fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Get the request controller that allows to abort request or disable request's timeout.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let (request, controller) = Request::new("http://example.com").controller();
    /// ```
    pub fn controller(self) -> (Self, RequestController) {
        let controller = self.controller.clone();
        (self, controller)
    }

    /// Fetch request. It's a chainable alternative to `fetch(request)`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// orders.perform_cmd({
    ///     let message = model.new_message.clone();
    ///     async { Msg::Fetched(send_message(message).await) }
    /// });
    /// ...
    /// async fn send_message(new_message: String) -> fetch::Result<shared::SendMessageResponseBody> {
    ///     Request::new(get_request_url())
    ///         .method(Method::Post)
    ///         .json(&shared::SendMessageRequestBody { text: new_message })?
    ///         .fetch()
    ///         .await?
    ///         .check_status()?
    ///         .json()
    ///         .await
    /// }
    /// ```
    ///
    /// ## Errors
    ///
    /// `fetch` will return `Err` only on network errors. This means that
    /// even if you get `Ok` from this function, you still need to check
    /// `Response` status for HTTP errors.
    pub async fn fetch(self) -> Result<Response> {
        fetch(self).await
    }
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for Request<'a> {
    fn from(s: T) -> Request<'a> {
        Request::new(s)
    }
}

impl<'a> From<Url> for Request<'a> {
    fn from(url: Url) -> Request<'a> {
        Request::new(url.to_string())
    }
}

impl TryFrom<Request<'_>> for web_sys::Request {
    type Error = FetchError;
    fn try_from(request: Request) -> std::result::Result<Self, Self::Error> {
        let mut init = web_sys::RequestInit::new();

        // headers
        let headers = web_sys::Headers::new().map_err(FetchError::RequestError)?;
        for header in request.headers {
            headers
                .append(&header.name, &header.value)
                .map_err(FetchError::RequestError)?;
        }
        init.headers(&headers);

        // method
        init.method(request.method.as_str());

        // body
        if let Some(body) = request.body {
            init.body(Some(&body));
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

        // timeout
        if let Some(timeout) = &request.timeout {
            let abort_controller = request.controller.clone();
            request.controller.timeout_handle.replace(Some(
                // abort request on timeout
                Timeout::new(*timeout, move || abort_controller.abort()),
            ));
        }

        // controller
        // https://developer.mozilla.org/en-US/docs/Web/API/AbortController/signal
        init.signal(Some(&request.controller.abort_controller.signal()));

        // It seems that the only reason why Request constructor can
        // fail is when Url contains credentials.  I assume that this
        // use case should be extremely rare, so to make api a bit
        // simplier let's just unwrap it here.
        //
        // See https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#Errors
        web_sys::Request::new_with_str_and_init(&request.url, &init)
            .map_err(FetchError::RequestError)
    }
}

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

#[cfg(test)]
mod tests {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use super::*;

    #[wasm_bindgen_test]
    async fn request_bytes() {
        let request = Request::new("").bytes([6, 2, 8, 3, 1, 8]);
        assert_eq!(
            request
                .body
                .unwrap()
                .dyn_ref::<Uint8Array>()
                .unwrap()
                .to_vec(),
            Vec::from([6, 2, 8, 3, 1, 8])
        )
    }
}
