//! Utilities for fetching player data from the OSRS hiscores.

use crate::utils::skill::{Skill, SKILLS};
use anyhow::Context;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::{collections::HashMap, convert::TryInto};

/// URL of the hiscore. Must also provider a ?player=<username> param.
const HISCORE_URL: &str =
    "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";

/// The list of minigames tracked in the hiscore. This order is very important
/// because it corresponds to the order they are in the response.
const MINIGAMES: &[&str] = &[
    // I'm *pretty sure* these first 3 are just placeholders to delimit skills
    // vs minigames
    "1?",
    "2?",
    "3?",
    "Clue Scroll (All)",
    "Clue Scroll (Beginner)",
    "Clue Scroll (Easy)",
    "Clue Scroll (Medium)",
    "Clue Scroll (Hard)",
    "Clue Scroll (Elite)",
    "Clue Scroll (Master)",
    "LMS - Rank",
    "Soul Wars Zeal",
    "Abyssal Sire",
    "Alchemical Hydra",
    "Barrows Chests",
    "Bryophyta",
    "Callisto",
    "Cerberus",
    "Chambers of Xeric",
    "Chambers of Xeric: Challenge Mode",
    "Chaos Elemental",
    "Chaos Fanatic",
    "Commander Zilyana",
    "Corporeal Best",
    "Dagannoth Prime",
    "Dagannoth Rex",
    "Dagannoth Supreme",
    "Crazy Archaeologist",
    "Deranged Archaeologist",
    "General Graardor",
    "Giant Mole",
    "Grotesque Guardians",
    "Hespori",
    "Kalphite Queen",
    "King Black Dragon",
    "Kraken",
    "Kree'Arra",
    "K'ril Tsutsaroth",
    "Mimic",
    "Nightmare",
    "Phosani's Nightmare",
    "Obor",
    "Sarachnis",
    "Scorpia",
    "Skotizo",
    "Tempoross",
    "The Guantlet",
    "The Corrupted Guantlet",
    "Theatre of Blood",
    "Theatre of Blood: Hard Mode",
    "Thermonuclear Smoke Devil",
    "TzKal-Zuk",
    "TzTok-Jad",
    "Venenatis",
    "Vet'ion",
    "Vorkath",
    "Wintertodt",
    "Zalcano",
    "Zulrah",
];

/// One row in the hiscores CSV response.
#[derive(Copy, Clone, Debug, Deserialize)]
struct HiscoreItem {
    // These are isize instead of usize because Jagex uses -1 for "missing"
    /// Player's rank in the category.
    rank: isize,
    /// For skills, the level. For everything else, the completion #.
    score: isize,
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

/// A minigame/boss/other stat tracked on the hiscores. This captures everything
/// other than skills.
#[derive(Clone, Debug)]
pub struct HiscoreMinigame {
    /// The minigame/boss name
    pub name: &'static str,
    /// The player's rank in this minigame
    pub rank: usize,
    /// The minigame score/completion count/kill count
    pub score: usize,
}

/// Hiscore results for a player.
#[derive(Clone, Debug)]
pub struct HiscorePlayer {
    /// Data on all skills for the player, keyed by skill name
    skills: HashMap<Skill, HiscoreSkill>,
    /// Data on all minigames/bosses for the player, keyed by minigame/boss
    /// name
    minigames: HashMap<&'static str, HiscoreMinigame>,
}

impl HiscorePlayer {
    /// Load a player's data from the hiscore.
    pub fn load(username: &str) -> anyhow::Result<Self> {
        // It's important that we convert to an iterator *now*, so that the
        // two blocks below use the same iterator, and each row will only be
        // consumed once
        let mut items = load_hiscore_items(username)?.into_iter();

        let skills: HashMap<Skill, HiscoreSkill> = SKILLS
            .iter()
            .zip(&mut items)
            .map(|(&skill, item)| {
                (
                    skill,
                    HiscoreSkill {
                        skill,
                        // These values should ALWAYS be >0 for skills
                        rank: item.rank.try_into().unwrap(),
                        level: item.score.try_into().unwrap(),
                        xp: item.xp.unwrap().try_into().unwrap(),
                    },
                )
            })
            .collect();

        let minigames: HashMap<&'static str, HiscoreMinigame> = MINIGAMES
            .iter()
            .zip(&mut items)
            .filter_map(|(&name, item)| {
                // Convert the rank+score from isize to usize. If it fails, that
                // means it's a placeholder value, so we don't want to include
                // this minigame
                match (item.rank.try_into(), item.score.try_into()) {
                    (Ok(rank), Ok(score)) => {
                        Some((name, HiscoreMinigame { name, rank, score }))
                    }
                    _ => None,
                }
            })
            .collect();

        Ok(Self { skills, minigames })
    }

    /// Get data for a single skill from the player
    pub fn skill(&self, skill: Skill) -> &HiscoreSkill {
        self.skills.get(&skill).unwrap()
    }

    /// Get a list of all skills for this player, in the standard order (i.e.)
    /// the order shown in the hiscores/in-game skill panel)
    pub fn skills(&self) -> Vec<&HiscoreSkill> {
        // We can't just use self.skills.values() because they have to be in
        // the correct order
        SKILLS
            .iter()
            .map(|skill| self.skills.get(skill).unwrap())
            .collect()
    }

    /// Get a list of minigame scores for the player. Any minigame for which
    /// the player has no score will not be included here.
    pub fn minigames(&self) -> Vec<&HiscoreMinigame> {
        MINIGAMES
            .iter()
            // Any minigame that the user has no entry for will be missing here
            .filter_map(|name| self.minigames.get(name))
            .collect()
    }
}

/// Load a list of hiscore entries for a player from the OSRS API. The API
/// response is a list of CSV entries formatted as `rank,level,xp` for skills
/// followed by `rank,score` for minigames/bosses. Entries are unlabeled so
/// each oen is identified only by its position in the list.
fn load_hiscore_items(username: &str) -> anyhow::Result<Vec<HiscoreItem>> {
    // Fetch data from the API
    let body = ureq::get(HISCORE_URL)
        .query("player", username)
        .call()?
        .into_string()?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(body.as_bytes());
    rdr.deserialize()
        // Iterator magic to convert Vec<Result> -> Result<Vec>
        // If any item fails, this whole thing will fail
        .collect::<Result<Vec<HiscoreItem>, csv::Error>>()
        .context("Error parsing hiscore data")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Make sure our parsing logic lines up with the current response format
    /// of the hiscores. We expect this test to break any time they add more
    /// lines to the hiscore response (which is typically when they release a
    /// new minigame/boss). Typically the fix is as easy as adding the new row
    /// to the `MINIGAMES` constant
    #[test]
    fn test_hiscore_response_parse() {
        let username = "Hey Jase"; // Sorry buddy you're the guinea pig

        // Load the raw CSV data
        let raw_response = load_hiscore_items(username).unwrap();
        // Also load via our parsing logic
        let player = HiscorePlayer::load("Hey Jase").unwrap();

        assert_eq!(
            SKILLS.len() + MINIGAMES.len(),
            raw_response.len(),
            "Unexpected number of rows in hiscore response. \
            Skill or minigame list needs to be updated."
        );

        // Make sure that the skill values all line up correctly
        for (i, skill) in player.skills().into_iter().enumerate() {
            let raw_row = raw_response[i];
            assert_eq!(
                skill.rank as isize, raw_row.rank,
                "Incorrect rank for skill {}",
                skill.skill
            );
            assert_eq!(
                skill.level as isize, raw_row.score,
                "Incorrect level for skill {}",
                skill.skill
            );
            assert_eq!(
                Some(skill.xp as isize),
                raw_row.xp,
                "Incorrect XP for skill {}",
                skill.skill
            );
        }

        // Make sure each minigame *that has data* appears in the player data.
        // Minigames with an insufficient score will appear as `-1` instead of
        // being populated, and we expect those to be excluded from the parsed
        // data. We want to skip over those in our check here.
        let parsed_minigames = player.minigames();
        let mut skipped = 0;
        for (i, raw_row) in raw_response[SKILLS.len()..].iter().enumerate() {
            dbg!(&raw_row);
            if raw_row.rank == -1 {
                skipped += 1;
            } else {
                let parsed_minigame = parsed_minigames[i - skipped];
                assert_eq!(
                    parsed_minigame.rank as isize, raw_row.rank,
                    "Incorrect rank for minigame {}",
                    parsed_minigame.name
                );
                assert_eq!(
                    parsed_minigame.score as isize, raw_row.score,
                    "Incorrect score for minigame {}",
                    parsed_minigame.name
                );
            }
        }
    }
}
