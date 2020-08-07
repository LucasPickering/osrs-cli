use crate::{
    commands::Command,
    error::OsrsResult,
    utils::{context::CommandContext, hiscore::HiscorePlayer},
};
use prettytable::{cell, row, Table};
use structopt::StructOpt;

/// Load and print player data from the OSRS hiscores.
#[derive(Debug, StructOpt)]
pub struct HiscoreCommand {
    /// The name of the player that you want to look up
    #[structopt(required = true)]
    username: Vec<String>,
}

impl Command for HiscoreCommand {
    fn execute(&self, context: &CommandContext) -> OsrsResult<()> {
        let player = HiscorePlayer::load(
            context.http_client(),
            self.username.join(" "),
        )?;

        // Print a table for skills
        println!("Skills");
        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        table.set_titles(row!["Skill", r->"Rank", r->"Level", r->"XP"]);
        for skill in player.skills() {
            table.add_row(row![
                skill.skill,
                r->context.fmt_num(&skill.rank),
                r->context.fmt_num(&skill.level),
                r->context.fmt_num(&skill.xp),
            ]);
        }
        table.printstd();
        println!();

        // Print a table for minigames/bosses/etc.
        println!("Minigames");
        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        table.set_titles(row!["Minigame", r->"Rank", r->"Score"]);
        for minigame in player.minigames() {
            table.add_row(row![
                minigame.name,
                r->context.fmt_num(&minigame.rank),
                r->context.fmt_num(&minigame.score),
            ]);
        }
        table.printstd();

        Ok(())
    }
}
