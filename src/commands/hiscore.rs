use crate::{commands::Command, utils::hiscore::HiscorePlayer};
use num_format::{SystemLocale, ToFormattedString};
use prettytable::{cell, row, Table};
use structopt::StructOpt;

/// Load and print player data from the OSRS hiscores.
#[derive(Debug, StructOpt)]
pub struct HiscoreOptions {
    /// The name of the player that you want to look up
    #[structopt(required = true)]
    username: Vec<String>,
}

pub struct HiscoreCommand;

impl Command for HiscoreCommand {
    type Options = HiscoreOptions;

    fn execute(&self, options: &Self::Options) -> anyhow::Result<()> {
        let player = HiscorePlayer::load(options.username.join(" "))?;

        // TODO move this code elsewhere. might be worth just writing our own
        // minimal table formatter and getting rid of prettytable
        let locale = SystemLocale::default().unwrap();
        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        table.set_titles(row!["Skill", r->"Rank", r->"Level", r->"XP"]);
        for skill in player.skills() {
            table.add_row(row![
                skill.name,
                r->skill.rank.to_formatted_string(&locale),
                r->skill.level.to_formatted_string(&locale),
                r->skill.xp.to_formatted_string(&locale),
            ]);
        }
        table.printstd();

        Ok(())
    }
}
