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

//! The Command Line Interface for gnucash-toolbox.

mod hello;
mod helpers;

use clap::{ArgAction, Parser, Subcommand};
use eyre::{Report, Result};
use tracing_subscriber::fmt::format::FmtSpan;

use self::hello::Hello;

/// The long version information.
const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\nrevision: ",
    env!("REVISION"),
    "\nfeatures: ",
    env!("FEATURES"),
    "\ntarget: ",
    env!("TARGET"),
    "\nprofile: ",
    env!("PROFILE"),
    "\nbuilt by: ",
    env!("BUILT_BY"),
);

/// A CLI toolbox to work with GnuCash databases.
#[derive(Debug, Parser)]
#[command(
    author,
    version = env!("VERSION_WITH_GIT"),
    long_version = LONG_VERSION,
)]
pub struct GnucashToolbox {
    /// The command to run.
    #[command(subcommand)]
    command: GnucashToolboxCommand,
    /// The verbosity level.
    #[arg(short = 'v', action = ArgAction::Count, global = true)]
    verbosity: u8,
}

/// The subcommands of `gnucash-toolbox`.
#[derive(Debug, Subcommand)]
pub enum GnucashToolboxCommand {
    /// Say hello.
    Hello(Hello),
}

/// A command.
trait Command {
    /// Runs the command.
    fn run(&self) -> Result<()>;
}

impl GnucashToolbox {
    /// Runs gnucash-toolbox.
    pub fn run() -> Result<()> {
        let args = Self::parse();
        setup_tracing(args.verbosity);

        match args.command.run() {
            Err(error) => handle_errors(error),
            Ok(()) => Ok(()),
        }
    }
}

impl GnucashToolboxCommand {
    /// Runs the given command.
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Hello(hello) => hello.run(),
        }
    }
}

/// Configures the tracing subscriber given the verbosity.
fn setup_tracing(verbosity: u8) {
    tracing_subscriber::fmt()
        .with_env_filter(env_filter(verbosity))
        .with_span_events(span_events(verbosity))
        .init();
}

/// Returns the trace filter to apply given the verbosity.
fn env_filter(verbosity: u8) -> &'static str {
    match verbosity {
        0 => "off",
        1 => "gnucash_toolbox=info",
        2 => "gnucash_toolbox=debug",
        3_u8..=u8::MAX => "gnucash_toolbox=trace",
    }
}

/// Returns the span events to enable given the verbosity.
fn span_events(verbosity: u8) -> FmtSpan {
    match verbosity {
        0..=3 => FmtSpan::NONE,
        4..=u8::MAX => FmtSpan::ACTIVE,
    }
}

// /// How to handle the error.
// enum ErrorHandling {
//     /// Return the report.
//     Return(Report),
//     /// Exit the program with the given status code.
//     Exit(i32),
// }

/// Handles typical usage errors to enhance their output.
fn handle_errors(error: Report) -> Result<()> {
    // let handling = if let Some(error) = error.downcast_ref::<ErrorType>() {
    //     handle_error_type(error)
    // } else {
    //     ErrorHandling::Return(error)
    // };

    // match handling {
    //     ErrorHandling::Return(error) => Err(error),
    //     ErrorHandling::Exit(code) => {
    //         #[expect(
    //             clippy::exit,
    //             reason = "this function is purposefully written to handle \
    //                 errors, write a useful message and exit with an error code"
    //         )]
    //         std::process::exit(code);
    //     }
    // }
    Err(error)
}

// fn handle_error_type(error: &ErrorType) -> ErrorHandling {
//     match error {
//         ErrorType::ErrorKind => {
//             error!("{error}");
//             hint!("Some help message.");
//         }
//     }

//     ErrorHandling::Exit(exitcode::USAGE)
// }
