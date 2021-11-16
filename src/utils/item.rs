//! This module contains utilities related to items. This includes functions
//! as well as a list of item IDs that we statically reference for certain
//! utilities within the codebase.

use crate::utils::http::HttpCache;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// IDs are pulled from https://www.osrsbox.com/tools/item-search/

pub const ITEM_ID_COMPOST: usize = 6032;
pub const ITEM_ID_SUPERCOMPOST: usize = 6034;
pub const ITEM_ID_ULTRACOMPOST: usize = 21483;

pub const ITEM_ID_GRIMY_GUAM_LEAF: usize = 199;
pub const ITEM_ID_GUAM_SEED: usize = 5291;
pub const ITEM_ID_GRIMY_MARRENTILL: usize = 201;
pub const ITEM_ID_MARRENTILL_SEED: usize = 5292;
pub const ITEM_ID_GRIMY_TARROMIN: usize = 203;
pub const ITEM_ID_TARROMIN_SEED: usize = 5293;
pub const ITEM_ID_GRIMY_HARRALANDER: usize = 205;
pub const ITEM_ID_HARRALANDER_SEED: usize = 5294;
pub const ITEM_ID_GOUTWEED: usize = 3261;
pub const ITEM_ID_GOUT_TUBER: usize = 6311;
pub const ITEM_ID_GRIMY_RANARR_WEED: usize = 207;
pub const ITEM_ID_RANARR_SEED: usize = 5295;
pub const ITEM_ID_GRIMY_TOADFLAX: usize = 3049;
pub const ITEM_ID_TOADFLAX_SEED: usize = 5296;
pub const ITEM_ID_GRIMY_IRIT: usize = 209;
pub const ITEM_ID_IRIT_SEED: usize = 5297;
pub const ITEM_ID_GRIMY_AVANTOE: usize = 211;
pub const ITEM_ID_AVANTOE_SEED: usize = 5298;
pub const ITEM_ID_GRIMY_KWUARM: usize = 213;
pub const ITEM_ID_KWUARM_SEED: usize = 5299;
pub const ITEM_ID_GRIMY_SNAPDRAGON: usize = 3051;
pub const ITEM_ID_SNAPDRAGON_SEED: usize = 5300;
pub const ITEM_ID_GRIMY_CADANTINE: usize = 215;
pub const ITEM_ID_CADANTINE_SEED: usize = 5301;
pub const ITEM_ID_GRIMY_LANTADYME: usize = 2485;
pub const ITEM_ID_LANTADYME_SEED: usize = 5302;
pub const ITEM_ID_GRIMY_DWARF_WEED: usize = 217;
pub const ITEM_ID_DWARF_WEED_SEED: usize = 5303;
pub const ITEM_ID_GRIMY_TORSTOL: usize = 219;
pub const ITEM_ID_TORSTOL_SEED: usize = 5304;

lazy_static! {
    /// We share a single client here for the whole program so that it can
    /// persist caches for the data it loads.
    ///
    /// We could hypothetically put this in the command context, but I didn't
    /// feel like it because then you have to plumb it everywhere.
    pub static ref WIKI_ITEM_CLIENT: WikiItemClient = WikiItemClient::new();
}

/// A client for fetching item and item price data from the [OSRS Wiki's
/// Real-time Prices API](https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices).
pub struct WikiItemClient {
    item_mapping: HttpCache<Vec<Item>>,
    prices: HttpCache<ItemPriceResponse>,
}

impl WikiItemClient {
    fn new() -> Self {
        Self {
            item_mapping: HttpCache::new(
                "https://prices.runescape.wiki/api/v1/osrs/mapping".into(),
            ),
            prices: HttpCache::new(
                "https://prices.runescape.wiki/api/v1/osrs/latest".into(),
            ),
        }
    }

    /// Get price data for a singular item, by its ID. Price data is cached for
    /// all items after the first lookup, so this will only ever make a request
    /// once for the program's lifetime.
    pub fn get_price(
        &self,
        item_id: usize,
    ) -> anyhow::Result<Option<ItemPrice>> {
        let item_price = self.prices.load()?.data.get(&item_id).copied();
        Ok(item_price)
    }

    /// Get a singular price value for an item. This will take the recent high
    /// and low, and average them.
    pub fn get_avg_price(
        &self,
        item_id: usize,
    ) -> anyhow::Result<Option<usize>> {
        Ok(self.get_price(item_id)?.map(|price| price.avg()).flatten())
    }

    /// Search items by name. This will do a caseless substring match, and
    /// return all items that match.
    pub fn search(&self, query: &str) -> anyhow::Result<Vec<Item>> {
        let items = self.item_mapping.load()?;

        // We want caseless search, so convert everything to lowercase
        // If this turns out to be really slow we could use a regex instead
        let query = query.to_lowercase();
        Ok(items
            .iter()
            .filter(|item| item.name.to_lowercase().contains(&query))
            // Unfortunately this is necessary, since the values are owned by
            // the cache
            .cloned()
            .collect())
    }

    /// Search items by name with price data. This uses the same search criteria
    /// as [Self::search].
    pub fn search_prices(
        &self,
        query: &str,
    ) -> anyhow::Result<Vec<ItemWithPrice>> {
        let items = self.search(query)?;
        let item_prices = &self.prices.load()?.data;
        // Join price data in for each item
        let items_with_prices = items
            .into_iter()
            .map(|item| {
                let price = item_prices.get(&item.id).copied();
                ItemWithPrice { item, price }
            })
            .collect();
        Ok(items_with_prices)
    }
}

/// An in-game item. This doesn't include price data, just static data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub examine: String,
    pub members: bool,
    #[serde(rename = "lowalch")]
    pub low_alch: Option<usize>,
    #[serde(rename = "highalch")]
    pub high_alch: Option<usize>,
    pub limit: Option<usize>,
    pub value: usize,
}

/// Current price data for an in-game item
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPrice {
    pub high: Option<usize>,
    pub high_time: Option<usize>,
    pub low: Option<usize>,
    pub low_time: Option<usize>,
}

impl ItemPrice {
    /// Get the average of the recent high and low prices.
    pub fn avg(&self) -> Option<usize> {
        match (self.high, self.low) {
            (Some(high), Some(low)) => Some(high + low / 2),
            (Some(value), None) | (None, Some(value)) => Some(value),
            (None, None) => None,
        }
    }
}

/// An item's core data paired with its current price data. Price data will be
/// `None` if it hasn't been traded recently.
#[derive(Clone, Debug)]
pub struct ItemWithPrice {
    pub item: Item,
    pub price: Option<ItemPrice>,
}

/// Response for the `/latest` endpoint of the price API
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ItemPriceResponse {
    data: HashMap<usize, ItemPrice>,
}
