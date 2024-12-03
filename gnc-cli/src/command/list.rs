// gnucash-toolbox - A toolbox to work with GnuCash databases.
// Copyright (C) 2024 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The `list` subcommand.

use clap::Parser;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use eyre::Result;

use gnc_database::{model::Price, Database};

/// Arguments for `gnucash-toolbox list`.
#[derive(Debug, Parser)]
pub struct List {
    /// The symbol to list.
    symbol: String,
}

impl super::Command for List {
    #[tracing::instrument(name = "list", level = "trace", skip_all)]
    fn run(&self) -> Result<()> {
        tracing::info!(params = ?self, "running list");

        let Self { symbol } = self;

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);

        table.set_header(vec!["Date", "Commodity", "Price", "Source", "Type"]);

        let mut db = Database::open("sqlite://Personnel.gnucash")?;
        for (price, commodity, currency) in
            Price::list_by_symbol(symbol, &mut db)?
        {
            let value = price.value().round(currency.scale_quote());
            let cur = currency.mnemonic();

            table.add_row(vec![
                &price.date().to_string(),
                commodity.mnemonic(),
                &format!("{value} {cur}"),
                price.source_str(),
                price.type_str(),
            ]);
        }

        println!("{table}");
        Ok(())
    }
}
