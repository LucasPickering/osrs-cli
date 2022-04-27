use crate::{
    commands::Command,
    utils::{
        context::CommandContext,
        fmt,
        item::{ItemPrice, WIKI_ITEM_CLIENT},
        table::TableExt,
    },
};
use async_trait::async_trait;
use comfy_table::{presets, CellAlignment, Table};
use std::io::Write;
use structopt::StructOpt;

/// Search for Grand Exchange item prices
#[derive(Debug, StructOpt)]
pub struct PriceCommand {
    /// Item search query
    #[structopt(required = true)]
    query: Vec<String>,
}

#[async_trait(?Send)]
impl<O: Write> Command<O> for PriceCommand {
    async fn execute(
        &self,
        mut context: CommandContext<O>,
    ) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        let query = self.query.join(" ");
        let items: Vec<(String, ItemPrice)> = WIKI_ITEM_CLIENT
            .search_prices(&query)
            .await?
            .into_iter()
            // Filter out items that have no price. Unpack into a tuple here
            // too so we can enforce that the price is populated
            .filter_map(|item| Some((item.item.name, item.price?)))
            .collect();

        if items.is_empty() {
            context.println("No results")?;
        } else {
            let mut table = Table::new();
            table
                .load_preset(presets::ASCII_BORDERS_ONLY_CONDENSED)
                .set_aligned_header([
                    ("Item", CellAlignment::Left),
                    ("Price", CellAlignment::Right),
                ]);
            for (name, price) in items {
                table.add_row(vec![&name, &fmt::fmt_price(price.avg())]);
            }

            context.print_table(&table)?;
        }

        Ok(())
    }
}
