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

//! The `add` subcommand.

use clap::Parser;
use eyre::Result;

use crate::database::Database;

/// Arguments for `gnucash-toolbox add`.
#[derive(Debug, Parser)]
pub struct Add {
    /// The key to add.
    key: String,
    /// The value to associate to the key.
    value: String,
}

impl super::Command for Add {
    #[tracing::instrument(name = "add", level = "trace", skip_all)]
    fn run(&self) -> Result<()> {
        tracing::info!(params = ?self, "running add");

        let Self { key, value } = self;

        let mut db = Database::open("sqlite://db.sqlite")?;
        db.add_entry(key, value)?;

        Ok(())
    }
}
