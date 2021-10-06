//! Utilities for fetching player data from the OSRS hiscores.

use crate::utils::skill::{Skill, SKILLS};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::{collections::HashMap, convert::TryInto};

/// URL of the hiscore. Must also provider a ?player=<username> param.
const HISCORE_URL: &str =
    "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws";

/// The list of minigames tracked in the hiscore. This order is very important
/// because it corresponds to the order they are in the response.
/// TODO fill this list out
const MINIGAMES: &[&str] = &[
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
    "11?",
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
    "Obor",
    "Sarachnis",
    "Scorpia",
    "Skotizo",
    "The Guantlet",
    "The Corrupted Guantlet",
    "Theatre of Blood",
    "Thermonuclear Smoke Devil",
    "TzKal-Zuk",
    "TzTok-Jab",
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
    minigames: HashMap<&'static str, HiscoreMinigame>,
}

impl HiscorePlayer {
    /// Load a player's data from the hiscore.
    pub fn load(username: String) -> anyhow::Result<Self> {
        // Fetch data from the API
        let body = ureq::get(HISCORE_URL)
            .query("player", &username)
            .call()?
            .into_string()?;

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(body.as_bytes());
        let mut items = rdr
            .deserialize()
            // Iterator magic to convert Vec<Result> -> Result<Vec>
            // If any item fails, this whole thing will fail
            .collect::<Result<Vec<HiscoreItem>, csv::Error>>()?
            .into_iter();

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

    /// Get a list of all skills for this player, in the standard order.
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
