//! Perform certain JSON (de)serialization sanity tests.

use js_sys::JSON;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen as swb;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct User {
    name: String,
    age: usize,
    cool: bool,
}

impl User {
    fn new(name: String, age: usize, cool: bool) -> Self {
        Self { name, age, cool }
    }
}

#[wasm_bindgen_test]
fn roundtrip_int() {
    roundtrip(1);
}

#[wasm_bindgen_test]
fn roundtrip_struct() {
    let user = User::new("Jack".to_string(), 8, true);
    roundtrip(user);
}

#[wasm_bindgen_test]
fn roundtrip_string() {
    let foo: String = "This is a string value".to_string();
    roundtrip(foo);
}

#[wasm_bindgen_test]
async fn serialize_json_request() {
    use seed::fetch::{Method, Request};
    use serde::{Deserialize, Serialize};
    use wasm_bindgen_futures::JsFuture;

    #[derive(Deserialize, Serialize)]
    pub struct FormEmail {
        email: String,
    }
    let email_form = FormEmail {
        email: "foo@bar.com".to_string(),
    };
    let request = Request::new("/api/email")
        .method(Method::Post)
        .json(&email_form)
        .expect("Serialization failed");
    let request: web_sys::Request = request.try_into().unwrap();
    let json = JsFuture::from(request.json().unwrap()).await.unwrap();
    let obj: js_sys::Object = json.try_into().unwrap();
    let email = js_sys::Reflect::get(&obj, &JsValue::from("email")).unwrap();
    assert_eq!(email, JsValue::from("foo@bar.com"));
}

fn roundtrip<T>(data: T)
where
    T: Serialize + DeserializeOwned + std::fmt::Debug + std::cmp::PartialEq,
{
    let a = roundtrip_serde_json(&data).unwrap();
    let b = roundtrip_swb(&data).unwrap();
    let c = roundtrip_swb_string(&data).unwrap();
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, data);
}

fn roundtrip_serde_json<T>(data: &T) -> Option<T>
where
    T: Serialize + DeserializeOwned,
{
    let v = serde_json::to_string(data).ok()?;
    serde_json::from_str(&v).ok()
}

fn roundtrip_swb<T>(data: &T) -> Option<T>
where
    T: Serialize + DeserializeOwned,
{
    let v = swb::to_value(data).ok()?;
    swb::from_value(v).ok()
}

fn roundtrip_swb_string<T>(data: &T) -> Option<T>
where
    T: Serialize + DeserializeOwned,
{
    let v: JsValue = swb::to_value(data).ok()?;
    let s: String = JSON::stringify(&v).ok()?.into();
    swb::from_value(JSON::parse(&s).ok()?).ok()
}
