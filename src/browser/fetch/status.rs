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

#[allow(clippy::module_name_repetitions)]
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
