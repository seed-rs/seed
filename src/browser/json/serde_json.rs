use super::*;

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Self {
        Error(JsValue::from(err.to_string()))
    }
}

pub fn from_js_value<T>(v: &JsValue) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(v.into_serde()?)
}

pub fn to_js_value<T>(v: &T) -> Result<JsValue>
where
    T: Serialize + ?Sized,
{
    Ok(JsValue::from_serde(v)?)
}
