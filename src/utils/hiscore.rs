//! Utilities for fetching player data from the OSRS hiscores.

use crate::{
    config::OsrsConfig,
    error::OsrsError,
    utils::{http, skill::Skill},
};
use serde::Deserialize;

/// One skill for a player in the hiscores.
#[derive(Clone, Debug, Deserialize)]
pub struct HiscoreSkill {
    /// The skill name.
    pub name: Skill,
    /// The player's rank in this skill (higher is better). -1 if unranked
    pub rank: isize,
    /// The player's level in the skill.
    pub level: usize,
    /// The player's total xp in the skill.
    pub xp: usize,
}

/// A minigame/boss/other stat tracked on the hiscores. This captures everything
/// other than skills.
#[derive(Clone, Debug, Deserialize)]
pub struct HiscoreActivity {
    /// The minigame/boss name
    pub name: String,
    /// The player's rank in this minigame. -1 if unranked
    pub rank: isize,
    /// The minigame score/completion count/kill count. -1 if unranked
    pub score: isize,
}

/// Hiscore results for a player.
#[derive(Clone, Debug, Deserialize)]
pub struct HiscorePlayer {
    /// Data on all skills for the player. Missing skills (ones that the
    /// hiscores didn't provide data on) will be excluded here
    pub skills: Vec<HiscoreSkill>,
    /// Data on all minigames/bosses for the player. Missing minigames (ones
    /// that the hiscores didn't provide data on) will be excluded here
    pub activities: Vec<HiscoreActivity>,
}

impl HiscorePlayer {
    /// Load a player's data from the hiscore.
    pub async fn load(username: &str) -> anyhow::Result<Self> {
        let mut data: Self = http::get(
            "https://secure.runescape.com/m=hiscore_oldschool/index_lite.json",
            &[("player", username)],
        )
        .await?;

        // Filter out activities with no history. This matches the behavior of
        // the official hiscores site
        data.activities.retain(|activity| activity.rank >= 0);

        Ok(data)
    }

    /// Load a player's stats from a combination of a command line argument
    /// and the config. If a name was supplied on the command line, use that,
    /// otherwise fall back to the config. If there's no username present there
    /// either, then return an error.
    ///
    /// This is useful for many commands that accept a `--player` argument.
    pub async fn load_from_args(
        cfg: &OsrsConfig,
        username_override: &[String],
    ) -> anyhow::Result<Self> {
        let username: String = match (username_override, &cfg.default_player) {
            // No arg provided, empty default - error
            (&[], None) => Err(anyhow::Error::from(OsrsError::ArgsError(
                "No player given".into(),
            ))),
            // No arg provided, but we have a default - use the default
            (&[], Some(default_player)) => Ok(default_player.clone()),
            // Arg was provided, return that
            (&[_, ..], _) => Ok(username_override.join(" ")),
        }?;
        Self::load(&username).await
    }

    /// Get data for a single skill from the player. Return `None` if we have
    /// no data for that skill. This is rare, but possible.
    pub fn skill(&self, skill: Skill) -> Option<&HiscoreSkill> {
        self.skills.iter().find(|s| s.name == skill)
    }
}
