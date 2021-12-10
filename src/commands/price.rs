use crate::{
    commands::Command,
    utils::{
        context::CommandContext,
        fmt,
        item::{ItemPrice, WIKI_ITEM_CLIENT},
    },
};
use prettytable::{cell, row, Table};
use structopt::StructOpt;

/// Search for Grand Exchange item prices
#[derive(Debug, StructOpt)]
pub struct PriceCommand {
    /// Item search query
    #[structopt(required = true)]
    query: Vec<String>,
}

impl Command for PriceCommand {
    fn execute(&self, _context: &CommandContext) -> anyhow::Result<()> {
        let query = self.query.join(" ");
        let items: Vec<(String, ItemPrice)> = WIKI_ITEM_CLIENT
            .search_prices(&query)?
            .into_iter()
            // Filter out items that have no price. Unpack into a tuple here
            // too so we can enforce that the price is populated
            .filter_map(|item| Some((item.item.name, item.price?)))
            .collect();

        if items.is_empty() {
            println!("No results");
        } else {
            let mut table = Table::new();
            table.set_format(
                *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
            );
            table.set_titles(row![
                "Item",
                r->"Price",
            ]);

            for (name, price) in items {
                table.add_row(row![
                    &name,
                    r->fmt::fmt_price(price.avg())
                ]);
            }

            table.printstd();
        }

        Ok(())
    }
}
