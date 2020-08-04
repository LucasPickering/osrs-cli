//! Utilities for fetching player data from the OSRS hiscores.

use crate::{
    error::OsrsResult,
    utils::skill::{Skill, SKILLS},
};
use csv::ReaderBuilder;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{collections::HashMap, convert::TryInto};

/// URL of the hiscore. Must also provider a ?player=<username> param.
const HISCORE_URL: &str =
    "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";

/// One row in the hiscores CSV response.
#[derive(Clone, Debug, Deserialize)]
struct HiscoreItem {
    // These are isize instead of usize because Jagex uses -1 for "missing"
    /// Player's rank in the category.
    rank: isize,
    /// Skill level, or score for non-skill items.
    level: isize,
    /// Total experience points. Only present for skills.
    #[serde(default)]
    xp: Option<isize>,
}

/// One skill for a player in the hiscores.
#[derive(Clone, Debug)]
pub struct HiscoreSkill {
    /// The skill name.
    pub skill: Skill,
    /// The player's rank in this skill (higher is better).
    pub rank: usize,
    /// The player's level in the skill.
    pub level: usize,
    /// The player's total xp in the skill.
    pub xp: usize,
}

/// Hiscore results for a player.
#[derive(Clone, Debug)]
pub struct HiscorePlayer {
    /// Player's name
    username: String,
    /// Data on all skills for the player, keyed by skill name
    skills: HashMap<Skill, HiscoreSkill>,
}

impl HiscorePlayer {
    /// Load a player's data from the hiscore.
    pub fn load(http_client: &Client, username: String) -> OsrsResult<Self> {
        // Fetch data from the API
        let body = http_client
            .get(HISCORE_URL)
            .query(&[("player", &username)])
            .send()?
            .text()?;

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(body.as_bytes());
        let items = rdr
            .deserialize()
            // Iterator magic to convert Vec<Result> -> Result<Vec>
            // If any item fails, this whole thing will fail
            .collect::<Result<Vec<HiscoreItem>, csv::Error>>()?;

        let skills: HashMap<Skill, HiscoreSkill> = SKILLS
            .iter()
            .zip(items)
            .map(|(&skill, item)| {
                (
                    skill,
                    HiscoreSkill {
                        skill,
                        // These values should ALWAYS be >0 for skills
                        rank: item.rank.try_into().unwrap(),
                        level: item.level.try_into().unwrap(),
                        xp: item.xp.unwrap().try_into().unwrap(),
                    },
                )
            })
            .collect();

        Ok(Self { username, skills })
    }

    pub fn skill(&self, skill: Skill) -> &HiscoreSkill {
        self.skills.get(&skill).unwrap()
    }

    /// Get a list of all skills for this player, in the standard order.
    pub fn skills(&self) -> Vec<&HiscoreSkill> {
        // We can't just use self.skills.values() because they have to be in
        // the correct order
        SKILLS
            .iter()
            .map(|skill| self.skills.get(skill).unwrap())
            .collect()
    }
}
