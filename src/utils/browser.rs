//! Utilities for running in the browser. Only inclued for Wasm targets.

use crate::error::OsrsError;
use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::{Storage, Window};

/// Access the `Window` object from JS
pub fn window() -> anyhow::Result<Window> {
    web_sys::window().ok_or_else(|| {
        OsrsError::UnsupportedEnvironment("Cannot access window".into()).into()
    })
}

/// Convert a JS error object to an anyhow error
pub fn js_to_anyhow(error: wasm_bindgen::JsValue) -> anyhow::Error {
    // The error *should* be a JS object (specfically an instance of Error),
    // so try to cast it and call .toString()
    let message: String = match error.dyn_ref::<Object>() {
        Some(object) => object.to_string().into(),
        // Well, at least we tried
        None => "Unknown error".into(),
    };
    anyhow::anyhow!(message)
}

/// A wrapper around browser local storage
pub struct LocalStorage {
    storage: Storage,
}

impl LocalStorage {
    pub fn new() -> anyhow::Result<Self> {
        // Boy that's a lot of fallibility. In reality none of this should ever
        // be fallible if we're running in a browser
        let storage = window()?
            .local_storage()
            .map_err(js_to_anyhow)?
            .ok_or_else::<anyhow::Error, _>(|| {
            OsrsError::UnsupportedEnvironment(
                "Cannot access local storage".into(),
            )
            .into()
        })?;
        Ok(Self { storage })
    }

    /// Get a value from local storage
    pub fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        self.storage.get(key).map_err(js_to_anyhow)
    }

    /// Set a value in local storage
    pub fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        self.storage.set(key, value).map_err(js_to_anyhow)
    }
}
