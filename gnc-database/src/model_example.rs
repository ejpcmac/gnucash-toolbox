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

//! Model of the GnuCash database.

use diesel::prelude::*;

// TODO: Document this, then start writing the actual empty API, then test.

/// A commodity.
#[derive(Debug, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = gnc_db_schema::commodities)]
#[diesel(primary_key(guid))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Commodity {
    /// The GUID of the commodity.
    pub guid: String,
    /// The namespace of the commodity.
    pub namespace: String,
    /// The mnemonic.
    pub mnemonic: String,
    /// The full name of the commodity.
    pub fullname: Option<String>,
    /// The CUSIP.
    pub cusip: Option<String>,
    /// The fraction into which the commodity can be divided.
    pub fraction: i32,
    /// TODO
    pub quote_flag: i32,
    /// TODO
    pub quote_source: Option<String>,
    /// TODO
    pub quote_tz: Option<String>,
}

/// A price.
#[derive(
    Debug, PartialEq, Identifiable, Associations, Queryable, Selectable,
)]
#[diesel(table_name = gnc_db_schema::prices)]
#[diesel(primary_key(commodity_guid, currency_guid, date))]
#[diesel(belongs_to(Commodity, foreign_key = commodity_guid))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Price {
    /// The commodity which is priced.
    pub commodity_guid: String,
    /// The currency used for the value.
    pub currency_guid: String,
    // TODO: chrono
    /// The date of the price.
    pub date: String,
    /// The source of the price.
    pub source: Option<String>,
    /// The type of price.
    pub type_: Option<String>,
    /// The numerator of the price.
    pub value_num: i64,
    /// The denominator of the price.
    pub value_denom: i64,
}

// /// The lock table.
// #[derive(Debug, PartialEq, Queryable, Selectable)]
// #[diesel(table_name = super::schema::gnclock)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
// pub struct GncLock {
//     /// The hostname of the lock taker.
//     #[diesel(select_expression = super::schema::gnclock::Hostname)]
//     pub hostname: Option<String>,
//     /// The pid of the lock taker.
//     #[diesel(select_expression = super::schema::gnclock::PID)]
//     pub pid: Option<i32>,
// }

// /// A key-value store.
// #[derive(Debug, Queryable, Selectable)]
// #[diesel(table_name = super::schema::entries)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
// pub struct Entry {
//     /// The primary key.
//     pub id: i32,
//     /// The key.
//     pub key: String,
//     /// The value.
//     pub value: String,
// }

// #[derive(Debug, Insertable)]
// #[diesel(table_name = super::schema::entries)]
// pub struct NewEntry<'a> {
//     /// The key.
//     pub key: &'a str,
//     /// The value.
//     pub value: &'a str,
// }
