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

//! Helpers for writing CLIs.

/// Prints a success.
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        let message = indoc::formatdoc!($($arg)*).green().bold();
        println!("{message}");
    }};
}

/// Prints a warning.
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        use colored::Colorize;

        let log_message = $crate::helpers::uncapitalise(&format!($($arg)*));
        let log_message = log_message.trim_end_matches(".");
        tracing::warn!("{log_message}");

        let message = indoc::formatdoc!($($arg)*).yellow().bold();
        eprintln!("{message}");
    }};
}

/// Prints an error.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        let message = indoc::formatdoc!($($arg)*);
        let message = $crate::helpers::uncapitalise(&message);
        let message = format!("Error: {message}").red().bold();
        eprintln!("{message}");
    }};
}

/// Prints a hint.
#[macro_export]
macro_rules! hint {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        let message = indoc::formatdoc!($($arg)*).blue();
        eprintln!("{message}");
    }};
}
