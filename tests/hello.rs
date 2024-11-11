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

//! CLI tests for `gnucash-toolbox hello`.

// NOTE: rexpect is only compatible with Unix-like systems, so let’s just not
// compile the CLI tests on Windows.
#![cfg(not(target_os = "windows"))]
#![allow(clippy::pedantic, clippy::restriction)]

use std::process::Command;

use assert_cmd::cargo::cargo_bin;
use eyre::Result;
use rexpect::session::spawn_command;

const TIMEOUT: Option<u64> = Some(1_000);

////////////////////////////////////////////////////////////////////////////////
//                                  Helpers                                   //
////////////////////////////////////////////////////////////////////////////////

fn gnucash_toolbox_hello() -> Result<Command> {
    let mut cmd = Command::new(cargo_bin("gnc"));
    cmd.env("NO_COLOR", "true").arg("hello");
    Ok(cmd)
}

////////////////////////////////////////////////////////////////////////////////
//                                   Hello                                    //
////////////////////////////////////////////////////////////////////////////////

#[test]
fn says_hello_world_by_default() -> Result<()> {
    let mut process = spawn_command(gnucash_toolbox_hello()?, TIMEOUT)?;

    process.exp_string("Hello, world!")?;
    process.exp_eof()?;

    Ok(())
}

#[test]
fn says_hello_with_name() -> Result<()> {
    let mut command = gnucash_toolbox_hello()?;
    command.arg("Steve");

    let mut process = spawn_command(command, TIMEOUT)?;

    process.exp_string("Hello, Steve!")?;
    process.exp_eof()?;

    Ok(())
}
