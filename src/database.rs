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

//! Access to the GnuCash database.

pub mod model;
pub mod schema;

use diesel::{Connection, ConnectionError, SqliteConnection};
use model::{Entry, NewEntry};
use thiserror::Error;

use crate::tracing::LogResult;

/// A GnuCash database.
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

// TODO: Better messages (maybe with a variable?)

/// Errors that can occur wen listing objects.
#[derive(Debug, Error)]
pub enum ListError {
    /// An error has ocured when listing objects.
    #[error("Failed to list")]
    QueryError(diesel::result::Error),
}

/// Errors that can occur wen adding objects.
#[derive(Debug, Error)]
pub enum AddError {
    /// An error has ocured when inserting a new object.
    #[error("Failed to add")]
    QueryError(diesel::result::Error),
}

impl Database {
    /// Opens a connection to the database.
    pub fn open(database_url: &str) -> Result<Self, OpenError> {
        let connection = SqliteConnection::establish(database_url)
            .map_err(OpenError::ConnectionError)
            .log_err()?;

        Ok(Self { connection })
    }

    /// Lists existing entries.
    pub fn list_entries(&mut self) -> Result<Vec<Entry>, ListError> {
        use self::schema::entries::dsl::*;
        use diesel::prelude::*;

        entries
            .select(Entry::as_select())
            .load(&mut self.connection)
            .map_err(ListError::QueryError)
            .log_err()
    }

    /// Adds a new entry.
    pub fn add_entry(
        &mut self,
        key: &str,
        value: &str,
    ) -> Result<(), AddError> {
        use diesel::prelude::*;

        let new_entry = NewEntry { key, value };
        diesel::insert_into(self::schema::entries::table)
            .values(&new_entry)
            .returning(Entry::as_returning())
            .get_result(&mut self.connection)
            .map_err(AddError::QueryError)
            .log_err()?;

        Ok(())
    }
}
