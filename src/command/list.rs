// gnucash-toolbox - A CLI toolbox to work with GnuCash databases.
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
use eyre::Result;

use crate::database::{model::Entry, Database};

/// Arguments for `gnucash-toolbox list`.
#[derive(Debug, Parser)]
pub struct List;

impl super::Command for List {
    #[tracing::instrument(name = "list", level = "trace", skip_all)]
    fn run(&self) -> Result<()> {
        tracing::info!(params = ?self, "running list");

        let mut db = Database::open("sqlite://db.sqlite")?;
        for entry in db.list_entries()? {
            let Entry { key, value, .. } = entry;
            println!("- {key}: {value}");
        }

        Ok(())
    }
}
