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

//! Model of the GnuCash database.

use diesel::prelude::*;

/// A key-value store.
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = super::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    /// The primary key.
    pub id: i32,
    /// The key.
    pub key: String,
    /// The value.
    pub value: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = super::schema::entries)]
pub struct NewEntry<'a> {
    /// The key.
    pub key: &'a str,
    /// The value.
    pub value: &'a str,
}
