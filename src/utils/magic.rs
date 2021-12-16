//! Utilities related to magic, runes, and spells

use crate::utils::{
    item::{self, WIKI_ITEM_CLIENT},
    math,
};

/// Runes are used to cast spells
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)] // We'll probably need these variants at some point
pub enum Rune {
    Air,
    Water,
    Earth,
    Fire,
    // Deliberate leaving out combo runes, not needed for now
    Body,
    Mind,
    Chaos,
    Death,
    Blood,
    Wrath,
    Cosmic,
    Nature,
    Law,
    Astral,
    Soul,
}

impl Rune {
    /// Get the ID of the item associated with this rune
    pub fn item_id(self) -> usize {
        match self {
            Self::Air => item::ITEM_ID_AIR_RUNE,
            Self::Water => item::ITEM_ID_WATER_RUNE,
            Self::Earth => item::ITEM_ID_EARTH_RUNE,
            Self::Fire => item::ITEM_ID_FIRE_RUNE,
            Self::Body => item::ITEM_ID_BODY_RUNE,
            Self::Mind => item::ITEM_ID_MIND_RUNE,
            Self::Chaos => item::ITEM_ID_CHAOS_RUNE,
            Self::Death => item::ITEM_ID_DEATH_RUNE,
            Self::Blood => item::ITEM_ID_BLOOD_RUNE,
            Self::Wrath => item::ITEM_ID_WRATH_RUNE,
            Self::Cosmic => item::ITEM_ID_COSMIC_RUNE,
            Self::Nature => item::ITEM_ID_NATURE_RUNE,
            Self::Law => item::ITEM_ID_LAW_RUNE,
            Self::Astral => item::ITEM_ID_ASTRAL_RUNE,
            Self::Soul => item::ITEM_ID_SOUL_RUNE,
        }
    }
}

/// Yer a wizard, Harry
#[derive(Copy, Clone, Debug)]
pub enum Spell {
    ResurrectCrops,
}

impl Spell {
    /// The level required to cast this spell
    pub fn level(self) -> usize {
        match self {
            Self::ResurrectCrops => 78,
        }
    }

    /// Get the runes required to cast this spell, as a mapping of
    /// rune->quantity
    pub fn runes(self) -> &'static [(Rune, usize)] {
        match self {
            Self::ResurrectCrops => &[
                (Rune::Earth, 25),
                (Rune::Nature, 12),
                (Rune::Blood, 8),
                (Rune::Soul, 8),
            ],
        }
    }

    /// Get the cost of casting this spell. Assumes all runes are used, i.e.
    /// no staffs or similar items. Returns an error if a price lookup fails.
    pub async fn rune_cost(self) -> anyhow::Result<usize> {
        let mut sum = 0;
        for (rune, quantity) in self.runes() {
            let price = WIKI_ITEM_CLIENT.get_avg_price(rune.item_id()).await?;
            // A rune price lookup should never return nothing as there are no
            // untradeable runes, but if it does, just consider them free.
            sum += price.unwrap_or_default() * quantity;
        }
        Ok(sum)
    }

    /// Calculate the chance of success for the Resurrect Crops spell, given
    /// the player's magic level. Scales from 50% at 78 (required level) to
    /// 75% at 99.
    pub fn resurrect_crops_chance(level: usize) -> f64 {
        let required_level = Self::ResurrectCrops.level();
        // Can use the spell at all ya fuckin noob
        if level < required_level {
            0.0
        } else {
            math::map_to_range(
                level as f64,
                required_level as f64,
                99.0,
                0.50,
                0.75,
            )
        }
    }
}
