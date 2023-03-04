use super::*;
use serde_wasm_bindgen as swb;

impl From<swb::Error> for Error {
    fn from(err: swb::Error) -> Self {
        Error(err.into())
    }
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
