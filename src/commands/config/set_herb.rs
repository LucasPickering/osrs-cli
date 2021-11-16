use crate::{
    commands::Command,
    config::FarmingHerbsConfig,
    utils::{
        console,
        context::CommandContext,
        farm::{AnimaPlant, Compost, HerbPatch},
    },
};
use structopt::StructOpt;

/// Interactive configurator for herb patches. Use this to configure your
/// herb patches and gear, then use the `calc farm herb` subcommand to view
/// statistics.
#[derive(Debug, StructOpt)]
pub struct ConfigSetHerbCommand {}

impl Command for ConfigSetHerbCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        let current_herb_config = &context.config().farming.herbs;

        // Ask the user a bunch of questions about global buffs
        let magic_secateurs = console::confirm(
            "Magic secateurs?",
            current_herb_config.magic_secateurs,
        )?;
        let farming_cape = console::confirm(
            "Farming cape?",
            current_herb_config.farming_cape,
        )?;
        let bottomless_bucket = console::confirm(
            "Bottomless bucket?",
            current_herb_config.bottomless_bucket,
        )?;
        let compost = console::enum_select::<Compost>(
            "Compost",
            current_herb_config.compost,
        )?;
        let anima_plant = console::enum_select::<AnimaPlant>(
            "Anima plant",
            current_herb_config.anima_plant,
        )?;

        // Show the user a list of patch names and let them update it
        let patches = console::enum_multi_select(
            "Patches",
            &current_herb_config.patches,
        )?;

        // Conditionally ask questions related to the individual selected
        // patches. This will tell use about which buffs they have for each
        // selected field
        let falador_diary = if patches.contains(&HerbPatch::Falador) {
            console::enum_select(
                "What level of Falador achievement diary have you completed?",
                current_herb_config.falador_diary,
            )?
        } else {
            current_herb_config.falador_diary
        };
        let kandarin_diary = if patches.contains(&HerbPatch::Catherby) {
            console::enum_select(
                "What level of Kandarin achievement diary have you completed?",
                current_herb_config.kandarin_diary,
            )?
        } else {
            current_herb_config.kandarin_diary
        };
        let kourend_diary = if patches.contains(&HerbPatch::FarmingGuild)
            || patches.contains(&HerbPatch::Hosidius)
        {
            console::enum_select(
                "What level of Kourend achievement diary have you completed?",
                current_herb_config.kourend_diary,
            )?
        } else {
            current_herb_config.kourend_diary
        };
        let hosidius_fifty_favor = if patches.contains(&HerbPatch::Hosidius) {
            console::confirm(
                "Do you have 50%+ Hosidius favor?",
                current_herb_config.hosidius_fifty_favor,
            )?
        } else {
            current_herb_config.hosidius_fifty_favor
        };

        let new_herb_config = FarmingHerbsConfig {
            patches,

            bottomless_bucket,
            farming_cape,
            magic_secateurs,
            compost,
            anima_plant,

            falador_diary,
            kandarin_diary,
            hosidius_fifty_favor,
            kourend_diary,
        };
        println!(
            "Setting herb patch config to: {}",
            serde_json::to_string_pretty(&new_herb_config)?
        );

        // Save the new config values
        let mut config = context.config().clone();
        config.farming.herbs = new_herb_config;
        config.save()?;

        Ok(())
    }
}
