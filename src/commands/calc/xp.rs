use crate::{
    commands::Command,
    error::{OsrsError, OsrsResult},
    utils::{context::CommandContext, hiscore::HiscorePlayer, skill::Skill},
};
use colored::*;
use structopt::StructOpt;

/// A list of the XP total required for each level. The index is (level-1), so
/// it starts with level 1. Be careful with index conversions! Goes up to the
/// the first impossible level (the first one past 200m).
const LEVEL_TO_XP: &[usize] = &[
    // Trust me, this is easier than computing it
    0, 83, 174, 276, 388, 512, 650, 801, 969, 1154, 1358, 1584, 1833, 2107,
    2411, 2746, 3115, 3523, 3973, 4470, 5018, 5624, 6291, 7028, 7842, 8740,
    9730, 10824, 12031, 13363, 14833, 16456, 18247, 20224, 22406, 24815, 27473,
    30408, 33648, 37224, 41171, 45529, 50339, 55649, 61512, 67983, 75127,
    83014, 91721, 101333, 111945, 123660, 136594, 150872, 166636, 184040,
    203254, 224466, 247886, 273742, 302288, 333804, 368599, 407015, 449428,
    496254, 547953, 605032, 668051, 737627, 814445, 899257, 992895, 1096278,
    1210421, 1336443, 1475581, 1629200, 1798808, 1986068, 2192818, 2421087,
    2673114, 2951373, 3258594, 3597792, 3972294, 4385776, 4842295, 5346332,
    5902831, 6517253, 7195629, 7944614, 8771558, 9684577, 10692629, 11805606,
    13034431, 14391160, 15889109, 17542976, 19368992, 21385073, 23611006,
    26068632, 28782069, 31777943, 35085654, 38737661, 42769801, 47221641,
    52136869, 57563718, 63555443, 70170840, 77474828, 85539082, 94442737,
    104273167, 115126838, 127110260, 140341028, 154948977, 171077457,
    188884740, 208545572,
];

/// Convert the given level to an XP total. Returns an error if the given level
/// is outside the supported range.
fn level_to_xp(level: usize) -> OsrsResult<usize> {
    if 1 <= level && level <= LEVEL_TO_XP.len() {
        Ok(LEVEL_TO_XP[level - 1])
    } else {
        Err(OsrsError::InvalidLevel(level))
    }
}

/// Convert the XP total to a level. Returns an error if the given XP
/// is outside the supported range.
fn xp_to_level(xp: usize) -> usize {
    let index = match LEVEL_TO_XP.binary_search(&xp) {
        Ok(idx) => idx,
        Err(idx) => idx - 1,
    };
    index + 1
}

// Options that define the starting xp value. Exactly one of these should be
// defined! Warning: DO NOT make this a doc comment! It will override the
// help on the options struct.
#[derive(Debug, StructOpt)]
struct SourceOptions {
    /// The XP amount to start from.
    #[structopt(long = "--from-xp")]
    source_xp: Option<usize>,
    /// The level to start from.
    #[structopt(long = "--from-lvl")]
    source_level: Option<usize>,
    /// The player to pull a starting XP amount from. MUST be used in tandem
    /// with --skill. If not given, will use the default player in the config.
    #[structopt(short, long)]
    player: Vec<String>,
    /// The skill to pull a starting XP amount from. MUST be used in tandem
    /// with --player (unless a default player is defined in the config).
    #[structopt(short, long)]
    skill: Option<Skill>,
}

// Options that define the target xp value. Exactly one of these should
// be defined! Warning: DO NOT make this a doc comment! It will override the
// help on the options struct.
#[derive(Debug, StructOpt)]
struct DestOptions {
    /// The XP amount to calculate to.
    #[structopt(long = "--to-xp")]
    dest_xp: Option<usize>,
    /// The level to calculate to.
    #[structopt(long = "--to-lvl")]
    dest_level: Option<usize>,
}

/// Calculate the xp needed to get from a starting point to an ending point.
#[derive(Debug, StructOpt)]
pub struct CalcXpCommand {
    #[structopt(flatten)]
    source: SourceOptions,
    #[structopt(flatten)]
    dest: DestOptions,
}

impl CalcXpCommand {
    fn get_source_xp(
        context: &CommandContext,
        options: &SourceOptions,
    ) -> OsrsResult<usize> {
        match options {
            // Use a given xp value
            SourceOptions {
                source_xp: Some(source_xp),
                source_level: None,
                player,
                skill: None,
            } if player.is_empty() => Ok(*source_xp),

            // Use a level
            SourceOptions {
                source_xp: None,
                source_level: Some(source_level),
                player,
                skill: None,
            } if player.is_empty() => level_to_xp(*source_level),

            // Look up the source xp for a player/skill combo
            SourceOptions {
                source_xp: None,
                source_level: None,
                player,
                skill: Some(skill),
            } => {
                let player = HiscorePlayer::load(
                    context.http_client(),
                    context.config().get_username(&player)?,
                )?;
                Ok(player.skill(*skill).xp)
            }

            // Anything else is invalid input, freak out!
            _ => Err(OsrsError::ArgsError(
                "Must specify exactly one of \
                    --from-xp, --from-lvl, or (--player and --skill)"
                    .into(),
            )),
        }
    }

    fn get_dest_xp(options: &DestOptions) -> OsrsResult<usize> {
        match options {
            // Use a given xp value
            DestOptions {
                dest_xp: Some(dest_xp),
                dest_level: None,
            } => Ok(*dest_xp),

            // Look up the source xp for a player/skill combo
            DestOptions {
                dest_xp: None,
                dest_level: Some(dest_level),
            } => level_to_xp(*dest_level),

            // Anything else is invalid input, freak out!
            _ => Err(OsrsError::ArgsError(
                "Must specify exactly one of --to-xp or --to-lvl".into(),
            )),
        }
    }
}

impl Command for CalcXpCommand {
    fn execute(&self, context: &CommandContext) -> OsrsResult<()> {
        let source_xp = Self::get_source_xp(context, &self.source)?;
        let dest_xp = Self::get_dest_xp(&self.dest)?;
        println!(
            "{} XP (Level {}) => {} XP (Level {}) = {}",
            context.fmt_num(&source_xp),
            xp_to_level(source_xp),
            context.fmt_num(&dest_xp),
            xp_to_level(dest_xp),
            // TODO make this show negative numbers
            format!("{} XP", context.fmt_num(&dest_xp.wrapping_sub(source_xp)))
                .blue()
                .bold()
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_to_xp() {
        assert_eq!(
            level_to_xp(0).unwrap_err().to_string(),
            "Invalid level. Must be between 1 and 127, got: 0"
        );
        assert_eq!(level_to_xp(1).unwrap(), 0);
        assert_eq!(level_to_xp(2).unwrap(), 83);
        assert_eq!(level_to_xp(92).unwrap(), 6_517_253);
        assert_eq!(level_to_xp(99).unwrap(), 13_034_431);
        assert_eq!(level_to_xp(126).unwrap(), 188_884_740);
        assert_eq!(level_to_xp(127).unwrap(), 208_545_572);
        assert_eq!(
            level_to_xp(128).unwrap_err().to_string(),
            "Invalid level. Must be between 1 and 127, got: 128"
        );
    }

    #[test]
    fn test_xp_to_level() {
        assert_eq!(xp_to_level(0), 1);
        assert_eq!(xp_to_level(1), 1);
        assert_eq!(xp_to_level(82), 1);
        assert_eq!(xp_to_level(83), 2);
        assert_eq!(xp_to_level(37223), 39);
        assert_eq!(xp_to_level(37224), 40);
        assert_eq!(xp_to_level(6_517_253), 92);
        assert_eq!(xp_to_level(13_034_431), 99);
        assert_eq!(xp_to_level(200_000_000), 126);
        assert_eq!(xp_to_level(999_999_999), 127);
    }
}
