use crate::{
    commands::Command,
    utils::{context::CommandContext, hiscore::HiscorePlayer},
};
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

    fn execute(
        &self,
        context: &CommandContext,
        options: &Self::Options,
    ) -> anyhow::Result<()> {
        let player = HiscorePlayer::load(
            context.http_client(),
            options.username.join(" "),
        )?;

        // TODO move this code elsewhere. might be worth just writing our own
        // minimal table formatter and getting rid of prettytable
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

        Ok(())
    }
}
