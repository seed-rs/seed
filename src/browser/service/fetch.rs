#![deprecated(
    since = "0.7.0",
    note = "Please use [`seed::browser::fetch`](../../fetch/index.html) module instead"
)]

//! High-level interface for `web_sys` HTTP requests.
use std::future::Future;

use gloo_timers::callback::Timeout;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{borrow::Cow, cell::RefCell, collections::HashMap, convert::identity, rc::Rc};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys;

// ---------- Aliases for foreign types ----------

pub type DomException = web_sys::DomException;

// ---------- Aliases ----------

/// Return type for `FetchObject.response()`.
pub type ResponseResult<T> = Result<Response<T>, FailReason<T>>;

/// Return type for `FetchObject.response_data()`.
pub type ResponseDataResult<T> = Result<T, FailReason<T>>;

/// Type for `FetchObject.result`.
#[allow(clippy::module_name_repetitions)]
pub type FetchResult<T> = Result<ResponseWithDataResult<T>, RequestError>;

/// Type for `ResponseWithDataResult.data`.
pub type DataResult<T> = Result<T, DataError>;

type Json = String;

// ---------- FetchObject ----------

#[derive(Debug, Clone)]
/// Return type for `Request.fetch*` methods.
#[allow(clippy::module_name_repetitions)]
pub struct FetchObject<T> {
    pub request: Request,
    pub result: FetchResult<T>,
}

impl<T> FetchObject<T> {
    #[allow(clippy::missing_errors_doc)]
    /// Get successful `Response` (status code 100-399) or `FailReason`.
    pub fn response(self) -> ResponseResult<T> {
        let response = match self.result {
            // `request_error` means that request was aborted, timed out, there was network error etc.
            Err(ref request_error) => {
                return Err(FailReason::RequestError(request_error.clone(), self))
            }
            Ok(ref response) => response,
        };

        if response.status.is_error() {
            // Response status code is in range 400-599.
            return Err(FailReason::Status(response.status.clone(), self));
        }

        if let Err(ref data_error) = response.data {
            // Converting body data to required type (String, JSON...) failed.
            return Err(FailReason::DataError(data_error.clone(), self));
        }

        let response = self.result.unwrap();
        Ok(Response {
            raw: response.raw,
            status: response.status,
            data: response.data.unwrap(),
        })
    }

    #[allow(clippy::missing_errors_doc)]
    /// Get successful `Response` data or `FailReason`.
    pub fn response_data(self) -> ResponseDataResult<T> {
        self.response().map(|response| response.data)
    }
}

// ---------- Fails ----------

#[derive(Debug, Clone)]
// @TODO use https://github.com/rust-lang-nursery/failure?
pub enum FailReason<T> {
    RequestError(RequestError, FetchObject<T>),
    Status(Status, FetchObject<T>),
    DataError(DataError, FetchObject<T>),
}

#[derive(Debug, Clone)]
pub enum RequestError {
    DomException(web_sys::DomException),
}

#[derive(Debug, Clone)]
pub enum DataError {
    DomException(web_sys::DomException),
    SerdeError(Rc<serde_json::Error>, Json),
}

// ---------- RequestController ----------

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

// ---------- Response Status ----------

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCategory {
    /// Code 1xx
    Informational,
    /// Code 2xx
    Success,
    /// Code 3xx
    Redirection,
    /// Code 4xx
    ClientError,
    /// Code 5xx
    ServerError,
    /// Code < 100 || Code >= 600
    Unknown,
}

#[derive(Debug, Clone)]
/// Response status.
///
/// It's intended to create `Status` from `web_sys::Response` - eg: `Status::from(&raw_response)`.
pub struct Status {
    /// Code examples: 200, 404, ...
    pub code: u16,
    /// Text examples: "OK", "Not Found", ...
    pub text: String,
    pub category: StatusCategory,
}

#[allow(dead_code)]
impl Status {
    /// Is response status category `ClientError` or `ServerError`? (Code 400-599)
    pub fn is_error(&self) -> bool {
        match self.category {
            StatusCategory::ClientError | StatusCategory::ServerError => true,
            _ => false,
        }
    }
    /// Is response status category `Success`? (Code 200-299)
    pub fn is_ok(&self) -> bool {
        self.category == StatusCategory::Success
    }
}

impl From<&web_sys::Response> for Status {
    fn from(response: &web_sys::Response) -> Self {
        let text = response.status_text();
        match response.status() {
            code @ 100..=199 => Status {
                code,
                text,
                category: StatusCategory::Informational,
            },
            code @ 200..=299 => Status {
                code,
                text,
                category: StatusCategory::Success,
            },
            code @ 300..=399 => Status {
                code,
                text,
                category: StatusCategory::Redirection,
            },
            code @ 400..=499 => Status {
                code,
                text,
                category: StatusCategory::ClientError,
            },
            code @ 500..=599 => Status {
                code,
                text,
                category: StatusCategory::ServerError,
            },
            code => Status {
                code,
                text,
                category: StatusCategory::Unknown,
            },
        }
    }
}

// ---------- Response ----------

#[derive(Debug, Clone)]
pub struct Response<T> {
    pub raw: web_sys::Response,
    pub status: Status,
    pub data: T,
}

#[derive(Debug, Clone)]
pub struct ResponseWithDataResult<T> {
    pub raw: web_sys::Response,
    pub status: Status,
    pub data: DataResult<T>,
}

// ---------- Method ----------

/// HTTP Method types.
///
/// [https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
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
    Patch,
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

impl Default for Method {
    fn default() -> Self {
        Method::Get
    }
}

// ---------- Request ----------

/// Request is the entry point for all fetch requests.
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
    controller: RequestController,
}

impl Request {
    // ------ PUBLIC ------

    pub fn new(url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            url: url.into(),
            ..Self::default()
        }
    }

    /// Set the HTTP method.
    /// Default is GET.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
    pub const fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Add a single header.
    /// String multiple calls to this together to add multiple ones.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers)
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn body(mut self, body: JsValue) -> Self {
        self.body = Some(body);
        self
    }

    /// Serialize a Rust data structure as JSON; eg the payload in a POST request.
    /// _Note_: If you want to setup `Content-Type` header automatically, use method `send_json`.
    pub fn body_json<T: Serialize>(self, body_json: &T) -> Self {
        let json =
            serde_json::to_string(body_json).expect("fetch: serialize body to JSON - failed");
        let json_as_js_value = JsValue::from_str(&json);
        self.body(json_as_js_value)
    }

    /// Set body to serialized `data`
    /// and set header `Content-Type` to `application/json; charset=utf-8`.
    pub fn send_json<T: Serialize>(self, data: &T) -> Self {
        self.header("Content-Type", "application/json; charset=utf-8")
            .body_json(data)
    }

    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)
    pub fn cache(mut self, cache: web_sys::RequestCache) -> Self {
        self.cache = Some(cache);
        self
    }

    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)
    pub fn credentials(mut self, request_credentials: web_sys::RequestCredentials) -> Self {
        self.credentials = Some(request_credentials);
        self
    }

    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity)
    pub fn integrity(mut self, integrity: &str) -> Self {
        self.integrity = Some(integrity.into());
        self
    }

    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/mode)
    pub fn mode(mut self, mode: web_sys::RequestMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)
    pub fn redirect(mut self, redirect: web_sys::RequestRedirect) -> Self {
        self.redirect = Some(redirect);
        self
    }

    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)
    pub fn referrer(mut self, referrer: String) -> Self {
        self.referrer = Some(referrer);
        self
    }

    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)
    pub fn referrer_policy(mut self, referrer_policy: web_sys::ReferrerPolicy) -> Self {
        self.referrer_policy = Some(referrer_policy);
        self
    }

    /// Enable request timeout and set it to given milliseconds.
    pub fn timeout(mut self, millis: u32) -> Self {
        self.timeout = Some(millis);
        self
    }

    /// Get request controller through callback function.
    /// You can use controller to abort request or disable timeout.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn send_request(
    ///    request_controller: &mut Option<fetch::RequestController>
    ///) -> impl Future<Item=Msg, Error=Msg> {
    ///    fetch::Request::new(get_request_url())
    ///        .controller(|controller| *request_controller = Some(controller))
    ///        .fetch_string(Msg::Fetched)
    ///}
    /// ```
    pub fn controller(self, controller_transferrer: impl FnOnce(RequestController)) -> Self {
        controller_transferrer(self.controller.clone());
        self
    }

    /// Fetch.
    ///
    /// It never fails. Use callback `f` to map `FetchObject<()>`.
    /// E.g.: You can use `std::convert::identity` as `f`
    /// to return `Result<FetchObject<()>, FetchObject<()>`.
    ///
    /// It's lazy - fetching is started when `Future` is executed.
    ///
    /// It always set `FetchObject.result->ResponseWithDataResult` field `data` to `Ok(())` -
    /// if you want to get body data, you have to use field `raw` to get raw `web_sys::Response`.
    /// (Or use methods like `fetch_string` / `fetch_json`.)
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///async fn send_request() -> Result<Msg, Msg> {
    ///    fetch::Request::new(get_request_url())
    ///        .fetch(Msg::Fetched)
    ///        .await
    ///}
    /// ```
    pub async fn fetch<U>(self, f: impl FnOnce(FetchObject<()>) -> U) -> U
    where
        U: 'static,
    {
        let fetch_result = self
            .send_request()
            .await
            .map(|raw_response: web_sys::Response| ResponseWithDataResult {
                status: Status::from(&raw_response),
                raw: raw_response,
                data: Ok(()),
            })
            .map_err(|js_value_error| RequestError::DomException(js_value_error.into()));
        f(FetchObject {
            request: self,
            result: fetch_result,
        })
    }

    /// Same as method `fetch`, but try to convert body to `String` and insert it into `Response` field `data`.
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Body/text]
    pub async fn fetch_string<U>(self, f: impl FnOnce(FetchObject<String>) -> U) -> U
    where
        U: 'static,
    {
        let fetch_object = self.fetch(identity).await;
        let fetch_result = fetch_object.result;
        let request = fetch_object.request;

        let fetch_object = match fetch_result {
            // There was problem with fetching - just change generic parameter from () to String.
            Err(request_error) => FetchObject::<String> {
                request,
                result: Err(request_error),
            },
            Ok(response) => {
                match response.raw.text() {
                    // Converting body to String failed.
                    Err(js_value_error) => FetchObject::<String> {
                        request,
                        result: Ok(ResponseWithDataResult {
                            raw: response.raw,
                            status: response.status,
                            data: Err(DataError::DomException(js_value_error.into())),
                        }),
                    },
                    Ok(promise) => {
                        let js_future_result = JsFuture::from(promise).await;
                        match js_future_result {
                            // Converting `promise` to `JsFuture` failed.
                            Err(js_value_error) => FetchObject::<String> {
                                request,
                                result: Ok(ResponseWithDataResult {
                                    raw: response.raw,
                                    status: response.status,
                                    data: Err(DataError::DomException(js_value_error.into())),
                                }),
                            },
                            Ok(js_value) => {
                                // Converting from body.text() result to String should never fail,
                                // so `expect` should be enough.
                                let text = js_value
                                    .as_string()
                                    .expect("fetch: cannot convert js_value to string");
                                FetchObject::<String> {
                                    request,
                                    result: Ok(ResponseWithDataResult {
                                        raw: response.raw,
                                        status: response.status,
                                        data: Ok(text),
                                    }),
                                }
                            }
                        }
                    }
                }
            }
        };
        f(fetch_object)
    }

    /// Fetch and then convert body to `String`. It passes `ResponseDataResult<String>` into callback `f`.
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Body/text)
    pub fn fetch_string_data<U>(
        self,
        f: impl FnOnce(ResponseDataResult<String>) -> U,
    ) -> impl Future<Output = U>
    where
        U: 'static,
    {
        self.fetch_string(|fetch_object| f(fetch_object.response_data()))
    }

    /// Same as method `fetch`, but try to deserialize body and insert it into `Response` field `data`.
    pub async fn fetch_json<T, U>(self, f: impl FnOnce(FetchObject<T>) -> U) -> U
    where
        T: DeserializeOwned + 'static,
        U: 'static,
    {
        let fetch_object = self.fetch_string(identity).await;
        let fetch_result = fetch_object.result;
        let request = fetch_object.request;

        let fetch_object = match fetch_result {
            // There was problem with fetching - just change generic parameter from String to T.
            Err(request_error) => FetchObject::<T> {
                request,
                result: Err(request_error),
            },
            Ok(response) => {
                match response.data {
                    // There was problem with converting to String
                    // - just change generic parameter from String to T.
                    Err(data_error) => FetchObject::<T> {
                        request,
                        result: Ok(ResponseWithDataResult {
                            raw: response.raw,
                            status: response.status,
                            data: Err(data_error),
                        }),
                    },
                    Ok(text) => {
                        match serde_json::from_str(&text) {
                            // Deserialization failed.
                            Err(serde_error) => FetchObject::<T> {
                                request,
                                result: Ok(ResponseWithDataResult {
                                    raw: response.raw,
                                    status: response.status,
                                    data: Err(DataError::SerdeError(Rc::new(serde_error), text)),
                                }),
                            },
                            Ok(value) => FetchObject::<T> {
                                request,
                                result: Ok(ResponseWithDataResult {
                                    raw: response.raw,
                                    status: response.status,
                                    data: Ok(value),
                                }),
                            },
                        }
                    }
                }
            }
        };
        f(fetch_object)
    }

    /// Fetch and then deserialize body to `T`. It passes `ResponseDataResult<T>` into callback `f`.
    pub fn fetch_json_data<T, U>(
        self,
        f: impl FnOnce(ResponseDataResult<T>) -> U,
    ) -> impl Future<Output = U>
    where
        T: DeserializeOwned + 'static,
        U: 'static,
    {
        self.fetch_json(|fetch_object| f(fetch_object.response_data()))
    }

    // ------ PRIVATE ------

    async fn send_request(&self) -> Result<web_sys::Response, JsValue> {
        let request_init = self.init_request_and_start_timeout();

        let fetch_promise = web_sys::window()
            .expect("fetch: cannot find window")
            .fetch_with_str_and_init(&self.url, &request_init);

        JsFuture::from(fetch_promise).await.map(Into::into)
    }

    fn init_request_and_start_timeout(&self) -> web_sys::RequestInit {
        let mut init = web_sys::RequestInit::new();

        // headers
        let headers = web_sys::Headers::new().expect("fetch: cannot create headers");
        for (name, value) in &self.headers {
            headers
                .append(name.as_str(), value.as_str())
                .expect("fetch: cannot create header")
        }
        init.headers(&headers);

        // method
        init.method(self.method.as_str());

        // body
        if let Some(body) = &self.body {
            init.body(Some(body));
        }

        // cache
        if let Some(cache) = self.cache {
            init.cache(cache);
        }

        // credentials
        if let Some(credentials) = self.credentials {
            init.credentials(credentials);
        }

        // integrity
        if let Some(integrity) = &self.integrity {
            init.integrity(integrity.as_str());
        }

        // mode
        if let Some(mode) = self.mode {
            init.mode(mode);
        }

        // redirect
        if let Some(redirect) = self.redirect {
            init.redirect(redirect);
        }

        // referrer
        if let Some(referrer) = &self.referrer {
            init.referrer(referrer.as_str());
        }

        // referrer_policy
        if let Some(referrer_policy) = self.referrer_policy {
            init.referrer_policy(referrer_policy);
        }

        // timeout
        if let Some(timeout) = &self.timeout {
            let abort_controller = self.controller.clone();
            *self.controller.timeout_handle.borrow_mut() = Some(
                // abort request on timeout
                Timeout::new(*timeout, move || abort_controller.abort()),
            );
        }

        // controller
        // https://developer.mozilla.org/en-US/docs/Web/API/AbortController/signal
        init.signal(Some(&self.controller.abort_controller.signal()));

        init
    }
}
