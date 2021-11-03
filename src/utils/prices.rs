//! Utilities for loading item price data from the
//! [OSRS Wiki's Real-time Prices API](https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices).

use crate::utils::http;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

const GE_URL: &str = "https://prices.runescape.wiki/api/v1/osrs/latest";

lazy_static! {
    /// We cache the full item price data (for *all* items) after the first
    /// time it's requested, since we don't expect these prices to change
    /// throughout the lifetime of this process. This will be populated after
    /// the first requested, and never again after that.
    ///
    /// We could hypothetically put this in the command context, but since we
    /// don't need it outside this module it's easier to put it here. Since this
    /// only ever needs to be written to once we use an RwLock, but in reality
    /// this program is single-threaded so right now a Mutex would be
    /// sufficient. This makes it a bit more future-proof though.
    static ref ITEM_PRICE_CACHE: RwLock<Option<HashMap<usize, Item>>> = RwLock::new(None);
}

/// Response for the `/latest` endpoint of the price API
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ItemPriceResponse {
    data: HashMap<usize, Item>,
}

/// An in-game item, in the context of the Grand Exchange. This contains data
/// about the item's recent price data on the GE.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub high: Option<usize>,
    pub high_time: Option<usize>,
    pub low: Option<usize>,
    pub low_time: Option<usize>,
}

impl Item {
    /// Load price data for a particular item. If price data hasn't been loaded
    /// yet during this process lifetime, then price data for *all* items will
    /// be loaded and cached. Subsequent price lookups throughout the lifetime
    /// of the process will used this cached data. If there is no price data
    /// for the given item ID, return `None`.
    pub fn load(item_id: usize) -> anyhow::Result<Option<Self>> {
        // We need to do this in a few discrete steps:
        // - Check if the cache is populated, and if so, grab the item from it
        // - If *not*, we need to drop the guard *first* so we're not holding
        //   the lock during the request
        // The outer option here indicates whether or not the cache is
        // populated. The inner option is whether or not the item is present in
        // the cache.
        let item_opt_opt: Option<Option<Item>> = {
            // Use some lexical scoping to make extra sure the guard is dropped
            ITEM_PRICE_CACHE
                .read()
                .unwrap()
                .as_ref()
                .map(|cache| cache.get(&item_id).copied())
        };

        // Now we check if the cache was present at all
        let item_opt = match item_opt_opt {
            // Cache exists, just use the value from there
            Some(item_opt) => item_opt,
            // No cache, grab the latest price data from the API
            None => {
                let response: ItemPriceResponse =
                    http::agent().get(GE_URL).call()?.into_json()?;
                // Grab this item out first, since we're about to do a move
                let item = response.data.get(&item_id).copied();

                // Write the fetcehd data for *all* items to the cache
                *ITEM_PRICE_CACHE.write().unwrap() = Some(response.data);
                item
            }
        };

        // If the item wasn't present, we just return nothing
        Ok(item_opt)
    }

    /// Get the latest "high" price for an item, which is the latest price at
    /// which it insta-bought.
    pub fn latest_high_price(item_id: usize) -> anyhow::Result<Option<usize>> {
        Ok(Self::load(item_id)?.map(|item| item.high).flatten())
    }
}
