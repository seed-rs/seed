use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen as swb;
use wasm_bindgen::JsValue;

pub fn from_js_value<T>(v: &JsValue) -> Result<T, JsValue>
where
    T: DeserializeOwned,
{
    Ok(swb::from_value(v.into())?)
}

pub fn to_js_value<T>(v: &T) -> Result<JsValue, JsValue>
where
    T: Serialize + ?Sized,
{
    Ok(v.serialize(&swb::Serializer::json_compatible())?)
}
