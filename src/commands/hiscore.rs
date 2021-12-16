use crate::{
    commands::Command,
    utils::{context::CommandContext, fmt, hiscore::HiscorePlayer},
};
use async_trait::async_trait;
use prettytable::{row, Table};
use std::io::Write;
use structopt::StructOpt;

/// Load and print player data from the OSRS hiscores.
#[derive(Debug, StructOpt)]
pub struct HiscoreCommand {
    /// The name of the player that you want to look up. If not given, will
    /// use the default player in the config.
    username: Vec<String>,
}

#[async_trait(?Send)]
impl<O: Write> Command<O> for HiscoreCommand {
    async fn execute(
        &self,
        mut context: CommandContext<O>,
    ) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        let player =
            HiscorePlayer::load_from_args(context.config(), &self.username)
                .await?;

        // Print a table for skills
        context.println("Skills")?;
        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        table.set_titles(row!["Skill", r->"Rank", r->"Level", r->"XP"]);
        for skill in player.skills {
            table.add_row(row![
                skill.name,
                r->fmt::fmt_int(&skill.rank),
                r->fmt::fmt_int(&skill.level),
                r->fmt::fmt_int(&skill.xp),
            ]);
        }
        context.print_table(&table)?;
        context.println("")?;

        // Print a table for minigames/bosses/etc.
        context.println("Minigames")?;
        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        table.set_titles(row!["Minigame", r->"Rank", r->"Score"]);
        for minigame in player.minigames {
            table.add_row(row![
                minigame.name,
                r->fmt::fmt_int(&minigame.rank),
                r->fmt::fmt_int(&minigame.score),
            ]);
        }
        context.print_table(&table)?;

        Ok(())
    }
}
