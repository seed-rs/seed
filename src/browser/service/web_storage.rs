//! Allows use of the Web Storage API including both local and session storage.
//!
//! # References
//! * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/Storage)
//! * [web-sys docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html)
//! * [Example syntax](https://github.com/rustwasm/wasm-bindgen/blob/master/examples/todomvc/src/store.rs)

extern crate serde;
extern crate serde_json;
use crate::browser::util::window;
use web_sys::Storage;

pub type JsValue = wasm_bindgen::JsValue;

pub enum Mechanism {
    LocalStorage,
    SessionStorage,
}

pub struct WebStorage {
    pub mechanism: Mechanism,
    storage: Storage,
}

/// Things that can go wrong when trying to load data from storage.
pub enum LoadError {
    /// Could not connect to storage.
    CouldNotConnect(JsValue),
    /// The data could not be decoded from JSON.
    CouldNotDecode(serde_json::Error),
    /// There is no data for that key.
    NoData,
}

/// Things that can go wrong when trying to save data to storage.
pub enum SaveError {
    /// The browser denied saving to storage. Usually because the storage is full.
    /// See: https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem#Exceptions
    CouldNotSave(JsValue),
    /// Supplied data could not be encoded to json.
    CouldNotEncode(serde_json::Error),
}

impl WebStorage {
    /// Clear all data in storage
    pub fn clear(&self) -> bool {
        self.storage.clear().is_ok()
    }

    /// A vector of all the keys in storage
    ///
    /// # Errors
    ///
    /// Will return a `Err(JsValue)` if the storage length could not be retrieved.
    pub fn keys(storage: &Storage) -> Result<Vec<String>, JsValue> {
        let mut keys = vec![];
        let length = storage.length()?;
        for index in 0..length {
            if let Ok(Some(key)) = storage.key(index) {
                keys.push(key);
            }
        }
        Ok(keys)
    }

    /// Load a JSON deserializable data structure from storage.
    ///
    /// # Errors
    ///
    /// Will return a `Err(LoadError)` if the data could not be loaded
    pub fn load<T>(&self, key: &str) -> Result<T, LoadError>
    where
        T: serde::de::DeserializeOwned,
    {
        let item = self
            .storage
            .get_item(key)
            .map_err(LoadError::CouldNotConnect)?;

        match item {
            None => Err(LoadError::NoData),
            Some(d) => {
                let decoded = serde_json::from_str(&d);
                decoded.map_err(LoadError::CouldNotDecode)
            }
        }
    }

    /// Delete a key and associated data from storage
    pub fn delete(&self, key: &str) -> bool {
        self.storage.remove_item(key).is_ok()
    }

    /// Save a JSON serializable data structure to storage.
    ///
    /// # Errors
    ///
    /// Will return a `SaveError` if the data could not be saved
    pub fn save<T>(&self, key: &str, data: &T) -> Result<(), SaveError>
    where
        T: serde::Serialize,
    {
        let serialized = serde_json::to_string(&data).map_err(SaveError::CouldNotEncode)?;
        self.storage
            .set_item(key, &serialized)
            .map_err(SaveError::CouldNotSave)
    }
}

/// Get an instance of Local Storage
/// Local Storage maintains a storage area that persists even when the browser
/// is closed and reopened
pub fn get_local_storage() -> Option<WebStorage> {
    window()
        .local_storage()
        .unwrap_or(None)
        .map(|storage| WebStorage {
            mechanism: Mechanism::LocalStorage,
            storage,
        })
}

/// Get an instance of Session Storage
/// Session Storage maintains a storage area for the duration of the page session
/// (as long as the browser is open, including page reloads and restores)
pub fn get_session_storage() -> Option<WebStorage> {
    window()
        .session_storage()
        .unwrap_or(None)
        .map(|storage| WebStorage {
            mechanism: Mechanism::SessionStorage,
            storage,
        })
}
