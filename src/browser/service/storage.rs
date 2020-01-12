//! Allows use of the Web Storage API / local storage.
//!
//! # References
//! * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/Storage)
//! * [web-sys docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html)
//! * [Example syntax](https://github.com/rustwasm/wasm-bindgen/blob/master/examples/todomvc/src/store.rs)

extern crate serde;
extern crate serde_json;

pub type Storage = web_sys::Storage;

#[allow(clippy::module_name_repetitions)]
pub fn get_storage() -> Option<Storage> {
    web_sys::window()
        .expect("get `window`")
        .local_storage()
        .ok()
        .flatten()
}

/// Create a new store, from a serializable data structure.
pub fn store_data<T>(storage: &Storage, name: &str, data: &T)
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_string(&data).expect("serialize for `LocalStorage`");
    storage
        .set_item(name, &serialized)
        .expect("save into `LocalStorage`");
}

/// Load a store, to a deserializable data structure.
pub fn load_data<T>(storage: &Storage, name: &str) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    storage
        .get_item(name)
        .expect("try to get item from `LocalStorage`")
        .map(|loaded_serialized| {
            serde_json::from_str(&loaded_serialized).expect("deserialize from `LocalStorage`")
        })
}
