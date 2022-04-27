use crate::error::OsrsError;
use serde::Deserialize;
use std::{fmt::Display, str::FromStr};

/// A macro to reduce copy-pasta for defining the list of all skills
macro_rules! skills {
    ($(($skill:ident, $aliases:expr)),+ $(,)?) => {
        /// One player skill (e.g. Attack, Woodcutting)
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
        pub enum Skill {
            $(
                $skill,
            )+
        }

        /// An array of: (skill, full name, list of aliases). This can be used
        /// to map to/from strings.
        const SKILLS_TO_NAMES: &[(Skill, &str, &[&str])] = &[
            $(
                (Skill::$skill, stringify!($skill), $aliases),
            )+
        ];
    };
}

skills! {
    // The full name will be used for display/parsing automatically. We only
    // need to specify other aliases.
    (Total, &[]),
    (Attack, &["atk"]),
    (Defence, &["defense", "def"]),
    (Strength, &["str"]),
    (Hitpoints, &["hp"]),
    (Ranged, &["range", "ranging"]),
    (Prayer, &["pray"]),
    (Magic, &["mage"]),
    (Cooking, &["cook"]),
    (Woodcutting, &["wc", "woodcut"]),
    (Fletching, &["fletch"]),
    (Fishing, &["fish"]),
    (Firemaking, &["fm", "fming"]),
    (Crafting, &["craft"]),
    (Smithing, &["smith"]),
    (Mining, &["mine"]),
    (Herblore, &["herb"]),
    (Agility, &["agi"]),
    (Thieving, &["thieve", "thief"]),
    (Slayer, &["slay"]),
    (Farming, &["farm"]),
    (Runecrafting, &["rc", "runecraft"]),
    (Hunter, &["hunt"]),
    (Construction, &["con", "cons"]),
}

impl FromStr for Skill {
    type Err = OsrsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        for (skill, primary_name, aliases) in SKILLS_TO_NAMES {
            if lower == primary_name.to_lowercase()
                || aliases.contains(&lower.as_str())
            {
                return Ok(*skill);
            }
        }

        Err(OsrsError::UnknownSkill(s.to_string()))
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (skill, primary_name, _) in SKILLS_TO_NAMES {
            if skill == self {
                return f.write_str(primary_name);
            }
        }
        // Impossible because the macro creates an entry for all skills
        panic!("Could not format name for skill: {self:?}");
    }
}
