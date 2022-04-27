use crate::{
    commands::Command,
    utils::{
        context::CommandContext, fmt, hiscore::HiscorePlayer, table::TableExt,
    },
};
use async_trait::async_trait;
use comfy_table::{presets, CellAlignment, Table};
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
        table
            .load_preset(presets::ASCII_BORDERS_ONLY_CONDENSED)
            .set_aligned_header([
                ("Skill", CellAlignment::Left),
                ("Rank", CellAlignment::Right),
                ("Level", CellAlignment::Right),
                ("XP", CellAlignment::Right),
            ]);
        for col in [1, 2, 3] {
            let column = table.get_column_mut(col).unwrap();
            column.set_cell_alignment(CellAlignment::Right);
        }
        for skill in player.skills {
            table.add_row(vec![
                skill.name.to_string(),
                fmt::fmt_int(&skill.rank),
                fmt::fmt_int(&skill.level),
                fmt::fmt_int(&skill.xp),
            ]);
        }
        context.print_table(&table)?;
        context.println("")?;

        // Print a table for minigames/bosses/etc.
        context.println("Minigames")?;
        let mut table = Table::new();
        table
            .load_preset(presets::ASCII_BORDERS_ONLY_CONDENSED)
            .set_aligned_header([
                ("Minigame", CellAlignment::Left),
                ("Rank", CellAlignment::Right),
                ("Score", CellAlignment::Right),
            ]);
        for minigame in player.minigames {
            table.add_row(vec![
                minigame.name,
                fmt::fmt_int(&minigame.rank),
                fmt::fmt_int(&minigame.score),
            ]);
        }
        context.print_table(&table)?;

        Ok(())
    }
}
