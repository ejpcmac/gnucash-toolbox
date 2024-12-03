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

//! A library to read and update a GnuCash database.

pub mod model;

use diesel::{Connection, ConnectionError, SqliteConnection};
use thiserror::Error;

use gnc_helpers::tracing::LogResult;

/// A GnuCash database.
#[expect(
    missing_debug_implementations,
    reason = "SqliteConnection does not implement debug"
)]
pub struct Database {
    /// The connection to the SQLite database.
    connection: SqliteConnection,
}

/// Errors that can occur wen opening the database.
#[derive(Debug, Error)]
pub enum OpenError {
    /// An error has ocured when opening the connection.
    #[error("Failed to establish a connection to the database")]
    ConnectionError(ConnectionError),
}

impl Database {
    /// Opens a connection to the database.
    pub fn open(database_url: &str) -> Result<Self, OpenError> {
        let connection = SqliteConnection::establish(database_url)
            .map_err(OpenError::ConnectionError)
            .log_err()?;

        Ok(Self { connection })
    }
}
