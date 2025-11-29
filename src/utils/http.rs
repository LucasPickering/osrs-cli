//! Utilities related to HTTP requests

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;
use std::{
    ops::Deref,
    sync::{RwLock, RwLockReadGuard, TryLockError},
};

/// Perform an HTTP GET request. The response is assumed to be JSON (as that's
/// what all current usages return).
pub async fn get<T: DeserializeOwned>(
    path: &str,
    query_params: &[(&str, &str)],
) -> anyhow::Result<T> {
    let response = http_client()?.get(path).query(query_params).send().await?;
    let body = response.error_for_status()?.text().await?;
    let json = serde_json::from_str(body.as_str())?;
    Ok(json)
}

/// Build a URL from a base path and list of query params. Each param's value
/// will be encoded
pub fn url(path: &str, query_params: &[(&str, &str)]) -> String {
    let params_vec: Vec<String> = query_params
        .iter()
        .map(|(param, value)| {
            format!("{}={}", param, urlencoding::encode(value))
        })
        .collect();
    format!("{}?{}", path, params_vec.join(","))
}

/// Build an HTTP client, for making requests. This client should be used for
/// *all* requests from the app, so we guarantee consistency of HTTP headers.
fn http_client() -> anyhow::Result<Client> {
    let builder = Client::builder();

    // All requests atm are JSON so set that here
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    let builder = builder.default_headers(headers);

    // The OSRS Wiki requests we set this for any requests to their API, but we
    // might as well just put it on all requests for consistency. Reqwest
    // doesn't support setting User-Agent in Wasm though, since the browser sets
    // that field itself. https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#Please_set_a_descriptive_User-Agent!
    #[cfg(not(target_family = "wasm"))]
    let builder =
        builder.user_agent(concat!("osrs-cli/", env!("CARGO_PKG_VERSION")));

    Ok(builder.build()?)
}

/// A write-once cache for an HTTP URL. The first time the value is requested,
/// it will be fetched from the URL. All subsequent requests will be fetched
/// from the cache. This guarantees that only one request will ever be made for
/// the lifetime of this struct.
pub struct HttpCache<T> {
    url: String,
    /// This will always be `Some` after the first request
    data: RwLock<Option<T>>,
}

impl<T: DeserializeOwned> HttpCache<T> {
    /// Create a new cache wrapper for the given URL
    pub fn new(url: String) -> Self {
        Self {
            url,
            data: RwLock::new(None),
        }
    }

    /// Load the value via HTTP if necessary, then return it. The returned
    /// value will be wrapped in a guard value that implements `Deref` to
    /// expose its inner value, meaning it can only be obtained by reference.
    pub async fn load(&self) -> anyhow::Result<CacheGuard<'_, T>> {
        /// TryLockError doesn't implement Send because it carries the guard,
        /// which is really annoying. To get around that we throw the error
        /// itself away and just hold the string. This is kinda shitty but
        /// we shouldn't ever actually hit this error because the program is
        /// single-threaded, so the lock can't be blocked or poisoned.
        fn map_lock_err<T>(err: TryLockError<T>) -> anyhow::Error {
            anyhow::Error::msg(err.to_string())
        }

        // Check if the data is populated. We'll immediately release the lock,
        // which will let us populated the cache if it isn't already.
        let is_loaded = self.data.try_read().map_err(map_lock_err)?.is_some();

        if !is_loaded {
            // Load the data from HTTP, then store it in the cache
            let response = get(&self.url, &[]).await?;
            let mut data_ref = self.data.try_write().map_err(map_lock_err)?;
            *data_ref = Some(response);
        }

        let guard = self.data.try_read().map_err(map_lock_err)?;
        // Now that we know the cache is populated, we can return a guard with
        // it. This lets the guard freely unwrap the inner cache value.
        Ok(CacheGuard(guard))
    }
}

/// A thin wrapper around the RwLock guard that will mask the fact that the
/// inner value is an option. By the time this guard gets created, we should
/// know for a fact that the cache is populated. That allows us to unwrap the
/// option in this guard rather than forcing the caller to do it. This is a
/// little annoying but there are no other options since `RwLockReadGuard`
/// doesn't have a `map` function.
pub struct CacheGuard<'a, T>(RwLockReadGuard<'a, Option<T>>);

impl<T> Deref for CacheGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // This guard should only ever be constructed around a populated cache,
        // so this unwrap is safe
        self.0.as_ref().unwrap()
    }
}
