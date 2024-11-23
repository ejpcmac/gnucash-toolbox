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

//! The `hello` subcommand.

use clap::Parser;
use eyre::Result;

/// Arguments for `gnc hello`.
#[derive(Debug, Parser)]
pub struct Hello {
    /// Who to say hello to.
    name: Option<String>,
}

impl super::Command for Hello {
    #[tracing::instrument(name = "hello", level = "trace", skip_all)]
    fn run(&self) -> Result<()> {
        tracing::info!(params = ?self, "running hello");

        let Self { name } = self;

        let name = name.as_deref().unwrap_or("world");
        println!("Hello, {name}!");

        Ok(())
    }
}
