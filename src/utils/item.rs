//! This module contains utilities related to items. This includes functions
//! as well as a list of item IDs that we statically reference for certain
//! utilities within the codebase.

use crate::utils::http::HttpCache;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            (Some(high), Some(low)) => Some((high + low) / 2),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_avg() {
        // No price data available
        assert_eq!(
            ItemPrice {
                high: None,
                high_time: None,
                low: None,
                low_time: None,
            }
            .avg(),
            None
        );

        // Only high price available
        assert_eq!(
            ItemPrice {
                high: Some(1000),
                high_time: Some(0),
                low: None,
                low_time: None,
            }
            .avg(),
            Some(1000)
        );

        // Only low price available
        assert_eq!(
            ItemPrice {
                high: None,
                high_time: None,
                low: Some(1000),
                low_time: Some(0),
            }
            .avg(),
            Some(1000)
        );

        // Both available - average them (should round down)
        assert_eq!(
            ItemPrice {
                high: Some(1000),
                high_time: Some(0),
                low: Some(995),
                low_time: Some(0),
            }
            .avg(),
            Some(997)
        );
    }
}
