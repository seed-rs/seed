use super::*;
use js_sys::{JsString, JSON};
use serde_wasm_bindgen as swb;

impl From<swb::Error> for Error {
    fn from(err: swb::Error) -> Self {
        Error::Serde(err.into())
    }
}

pub fn to_string<T>(v: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    Ok(to_js_string(v)?.into())
}

pub fn to_js_string<T>(v: &T) -> Result<JsString>
where
    T: Serialize + ?Sized,
{
    let v = to_js_value(v)?;
    let js_string = JSON::stringify(&v).map_err(Error::Stringify)?;
    Ok(js_string)
}

pub fn from_str<T>(data: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let v = JSON::parse(data).map_err(Error::Parse)?;
    let v = from_js_value(&v)?;
    Ok(v)
}

pub fn from_js_value<T>(v: &JsValue) -> Result<T>
where
    T: DeserializeOwned,
{
    let v = swb::from_value(v.into())?;
    Ok(v)
}

pub fn to_js_value<T>(v: &T) -> Result<JsValue>
where
    T: Serialize + ?Sized,
{
    Ok(v.serialize(&swb::Serializer::json_compatible())?)
}
