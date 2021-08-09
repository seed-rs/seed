use crate::browser::util::window;
use js_sys::JSON;
use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen as swb;
use wasm_bindgen::JsValue;
use web_sys::Storage;

/// Convenient type alias.
pub type Result<T> = std::result::Result<T, WebStorageError>;

// ------ WebStorageError ------

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum WebStorageError {
    GetStorageError(JsValue),
    StorageNotFoundError,
    ClearError(JsValue),
    GetLengthError(JsValue),
    GetKeyError(JsValue),
    KeyNotFoundError,
    RemoveError(JsValue),
    GetError(JsValue),
    InsertError(JsValue),
    SerdeError(swb::Error),
    ParseError(JsValue),
    ConversionError,
}

impl From<swb::Error> for WebStorageError {
    fn from(v: swb::Error) -> Self {
        Self::SerdeError(v)
    }
}

// ------ LocalStorage ------

/// Local Storage  maintains a separate storage area for each given origin
/// that persists even when the browser is closed and reopened.
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
pub struct LocalStorage;

impl WebStorage for LocalStorage {
    fn storage() -> Result<Storage> {
        window()
            .local_storage()
            .map_err(WebStorageError::GetStorageError)?
            .ok_or(WebStorageError::StorageNotFoundError)
    }
}

// ------ SessionStorage ------

/// - Session Storage maintains a separate storage area for each given origin
/// that's available for the duration of the page session
/// (as long as the browser is open, including page reloads and restores).
///
/// - Opening multiple tabs/windows with the same URL creates sessionStorage for each tab/window.
///
/// - Data stored in sessionStorage is specific to the protocol of the page.
/// In other words, _`http://example.com`_ will have separate storage than _`https://example.com`_.
///
/// - Storage limit is larger than a cookie (at most 5MB).
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
pub struct SessionStorage;

impl WebStorage for SessionStorage {
    fn storage() -> Result<Storage> {
        window()
            .session_storage()
            .map_err(WebStorageError::GetStorageError)?
            .ok_or(WebStorageError::StorageNotFoundError)
    }
}

// ------ WebStorage ------

/// Web Storage API.
///
/// `LocalStorage` and `SessionStorage` implement this trait.
///
/// (If you think some important methods are missing,
/// please create an [issue](https://github.com/seed-rs/seed/issues/new))
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API)
pub trait WebStorage {
    /// Get a native `Storage` instance.
    ///
    /// This method is used internally by other methods.
    ///
    /// (If you need to call it often,
    /// please create an [issue](https://github.com/seed-rs/seed/issues/new))
    ///
    /// # Errors
    ///
    /// Returns error if we cannot get access to the storage - security errors,
    /// browser does not have given storage, user denied access for the current origin, etc.
    ///
    /// - [MDN ref for Local Storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    /// - [MDN ref for Session Storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
    fn storage() -> Result<Storage>;

    /// Clear all data in the storage.
    ///
    /// # Errors
    ///
    /// Returns error if we cannot get access to the storage or clear the storage.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)
    fn clear() -> Result<()> {
        Self::storage()?
            .clear()
            .map_err(WebStorageError::ClearError)
    }

    /// Get the number of stored data items.
    ///
    /// # Errors
    ///
    /// Returns error if we cannot get access to the storage or read the storage length.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)
    fn len() -> Result<u32> {
        Self::storage()?
            .length()
            .map_err(WebStorageError::GetLengthError)
    }

    /// Returns the key in the given position.
    ///
    /// # Errors
    ///
    /// Returns error if we cannot get access to the storage or the key does not exist.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/key)
    fn key(index: u32) -> Result<String> {
        Self::storage()?
            .key(index)
            .map_err(WebStorageError::GetKeyError)?
            .ok_or(WebStorageError::KeyNotFoundError)
    }

    /// Removes a key.
    ///
    /// If there is no item associated with the given key, this method will do nothing.
    ///
    /// # Errors
    ///
    /// Returns error if we cannot get access to the storage or remove the existing key.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)
    fn remove(key: impl AsRef<str>) -> Result<()> {
        Self::storage()?
            .remove_item(key.as_ref())
            .map_err(WebStorageError::RemoveError)
    }

    /// Returns a deserialized value corresponding to the key.
    ///
    /// # Errors
    ///
    /// Returns error if we cannot get access to the storage
    /// or find the key or deserialize the value.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)
    fn get<S, T>(key: S) -> Result<T>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
    {
        let item: String = Self::storage()?
            .get_item(key.as_ref())
            .map_err(WebStorageError::GetError)?
            .ok_or(WebStorageError::KeyNotFoundError)?;
        let js: JsValue = JSON::parse(&item).map_err(WebStorageError::ParseError)?;

        Ok(swb::from_value(js)?)
    }

    /// Insert a key-value pair. The value will be serialized.
    ///
    /// If the key already exists, the value will be updated.
    ///
    /// # Errors
    ///
    /// Returns error if we cannot get access to the storage
    /// or serialize the value or insert/update the pair.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)
    fn insert<S, T>(key: S, value: &T) -> Result<()>
    where
        S: AsRef<str>,
        T: Serialize,
    {
        let value = swb::to_value(value)?
            .as_string()
            .ok_or(WebStorageError::ConversionError)?;

        Self::storage()?
            .set_item(key.as_ref(), &value)
            .map_err(WebStorageError::InsertError)
    }
}

// ------ ------ Tests ------ ------

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // ------ LocalStorage tests ------

    #[wasm_bindgen_test]
    fn local_storage_insert_get() {
        LocalStorage::clear().unwrap();

        let value: String = {
            LocalStorage::insert(&"a_key", &"a_value").unwrap();
            LocalStorage::get("a_key").unwrap()
        };
        assert_eq!("a_value", value)
    }

    #[wasm_bindgen_test]
    fn local_storage_length_clear() {
        LocalStorage::clear().unwrap();
        assert_eq!(0, LocalStorage::len().unwrap());

        LocalStorage::insert(&"key_1", &"a_value").unwrap();
        LocalStorage::insert(&"key_2", &"a_value").unwrap();
        assert_eq!(2, LocalStorage::len().unwrap());

        LocalStorage::clear().unwrap();
        assert_eq!(0, LocalStorage::len().unwrap());
    }

    #[wasm_bindgen_test]
    fn local_storage_key() {
        SessionStorage::clear().unwrap();

        SessionStorage::insert(&"a_key", &"a_value").unwrap();
        assert_eq!("a_key", SessionStorage::key(0).unwrap());
    }

    #[wasm_bindgen_test]
    fn local_storage_remove() {
        SessionStorage::clear().unwrap();

        SessionStorage::insert(&"a_key", &"a_value").unwrap();
        SessionStorage::remove("a_key").unwrap();
        assert_eq!(0, SessionStorage::len().unwrap());
    }

    // ------ SessionStorage tests ------

    #[wasm_bindgen_test]
    fn session_storage_insert_get() {
        SessionStorage::clear().unwrap();

        let value: String = {
            SessionStorage::insert(&"a_key", &"a_value").unwrap();
            SessionStorage::get("a_key").unwrap()
        };
        assert_eq!("a_value", value)
    }

    #[wasm_bindgen_test]
    fn session_storage_length_clear() {
        SessionStorage::clear().unwrap();
        assert_eq!(0, SessionStorage::len().unwrap());

        SessionStorage::insert(&"key_1", &"a_value").unwrap();
        SessionStorage::insert("&key_2", &"a_value").unwrap();
        assert_eq!(2, SessionStorage::len().unwrap());

        SessionStorage::clear().unwrap();
        assert_eq!(0, SessionStorage::len().unwrap());
    }

    #[wasm_bindgen_test]
    fn session_storage_key() {
        SessionStorage::clear().unwrap();

        SessionStorage::insert(&"a_key", &"a_value").unwrap();
        assert_eq!("a_key", SessionStorage::key(0).unwrap());
    }

    #[wasm_bindgen_test]
    fn session_storage_remove() {
        SessionStorage::clear().unwrap();

        SessionStorage::insert(&"a_key", &"a_value").unwrap();
        SessionStorage::remove("a_key").unwrap();
        assert_eq!(0, SessionStorage::len().unwrap());
    }
}
