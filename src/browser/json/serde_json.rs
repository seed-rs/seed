use super::*;
use js_sys::JsString;

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Self {
        Error::Serde(JsValue::from(err.to_string()))
    }
}

pub fn to_string<T>(v: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    Ok(::serde_json::to_string(v)?)
}

pub fn to_js_string<T>(v: &T) -> Result<JsString>
where
    T: Serialize + ?Sized,
{
    let v = to_string(v)?;
    let js_string = JsString::from(v);
    Ok(js_string)
}

pub fn from_str<T>(v: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let v = ::serde_json::from_str(v)?;
    Ok(v)
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
