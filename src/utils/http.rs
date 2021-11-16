//! Utilities related to HTTP requests

use serde::de::DeserializeOwned;
use std::{
    ops::Deref,
    sync::{RwLock, RwLockReadGuard, TryLockError},
};
use ureq::{Agent, AgentBuilder};

/// Get an HTTP agent, for making requests. This should be used for *all* HTTP
/// requests, because it provides important configuration on the agent.
pub fn agent() -> Agent {
    AgentBuilder::new()
        // The OSRS Wiki requests we set this for any requests to their API, but
        // we might as well just put it on all requests for consistency
        // https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#Please_set_a_descriptive_User-Agent!
        .user_agent(&format!("osrs-cli/{}", env!("CARGO_PKG_VERSION")))
        .build()
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
    pub fn load(&self) -> anyhow::Result<CacheGuard<T>> {
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
            let response = agent().get(&self.url).call()?.into_json()?;
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
