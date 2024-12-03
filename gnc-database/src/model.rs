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

use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use strum_macros::{Display, EnumString};
use thiserror::Error;

use gnc_helpers::tracing::LogResult;

use super::Database;

// TODO: Re-do a pass here, then continue in the API.

/// A commodity.
#[derive(Debug, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = gnc_db_schema::commodities)]
#[diesel(primary_key(guid))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Commodity {
    /// The GUID of the commodity.
    guid: String,
    /// The namespace of the commodity.
    ///
    /// This can be something like "CURRENCY", "NASDAQ", but also user-defined
    /// ones.
    namespace: String,
    /// The mnemonic of the commodity.
    ///
    /// For currencies, this is the ISO 4217 alphabetic code.
    mnemonic: String,
    /// The full name of the commodity.
    fullname: Option<String>,
    /// The CUSIP or other identifying code of the commodity.
    ///
    /// For currencies, this is the ISO 4217 numeric code.
    cusip: Option<String>,
    /// The fraction into which the commodity can be divided.
    ///
    /// For instance, this is 100 if the commodity can be divided in cents.
    fraction: i32,
    /// Whether to get price quotes for this commodity.
    ///
    /// This is a C boolean: 0 means false and any other value means true.
    quote_flag: i32,
    /// The source to use to get price quotes.
    ///
    /// Valid values are defined in GnuCash, in the `quote_sources` array in
    /// `gnc-ui-utils.c`.
    quote_source: Option<String>,
    /// The timezone used for price quotes.
    quote_tz: Option<String>,
}

/// A price.
#[derive(
    Debug, PartialEq, Associations, Identifiable, Queryable, Selectable,
)]
#[diesel(table_name = gnc_db_schema::prices)]
#[diesel(primary_key(commodity_guid, currency_guid, date))]
#[diesel(belongs_to(Commodity, foreign_key = commodity_guid))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Price {
    /// The commodity which is priced.
    commodity_guid: String,
    /// The currency used for the value.
    currency_guid: String,
    /// The date of the price.
    ///
    /// The date is stored in UTC, following the `YYYY-MM-DD HH:MM:SS` format.
    date: String,
    /// The source of the price.
    ///
    /// Valid values are defined in GnuCash, in the `source_names` arary in
    /// `gnc-pricedb.cpp`.
    source: Option<String>,
    /// The type of price.
    ///
    /// Valid values are "bid", "ask", "last", "nav", "transaction" and
    /// "unknown".
    type_: Option<String>,
    /// The numerator of the price.
    value_num: i64,
    /// The denominator of the price.
    value_denom: i64,
}

/// The sources of prices.
///
/// Valid values are defined in GnuCash, in the `source_names` arary in
/// `gnc-pricedb.cpp`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, Display)]
pub enum PriceSource {
    /// The value comes from the price editor.
    #[strum(serialize = "user:price-editor")]
    PriceEditor,
    /// The value comes from `Finance::Quote`.
    #[strum(serialize = "Finance::Quote")]
    FinanceQuote,
    /// The value comes from a user-entered price in a transaction.
    #[strum(serialize = "user:price")]
    UserPrice,
    /// The value comes from the transfer dialog during a transaction.
    #[strum(serialize = "user:xfer-dialog")]
    XferDialog,
    /// The value comes from a transaction.
    #[strum(serialize = "user:split-register")]
    SplitRegister,
    /// The value comes from an imported transaction.
    #[strum(serialize = "user:split-import")]
    SplitImport,
    /// The value comes from the stock split.
    #[strum(serialize = "user:stock-split")]
    StockSplit,
    /// The price comes from a stock transaction.
    #[strum(serialize = "user:stock-transaction")]
    StockTransaction,
    /// The price comes from an invoice (legacy, kept for compatibility).
    #[strum(serialize = "user:invoice-post")]
    Invoice,
    /// Temporary price (not actually used).
    #[strum(serialize = "temporary")]
    Temporary,
    /// Invalid price (not actually used).
    #[strum(serialize = "invalid")]
    Invalid(String),
}

/// The types of price quotes.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum PriceType {
    /// The price represents the current bid.
    #[strum(serialize = "bid")]
    Bid,
    /// The price represents the current ask.
    #[strum(serialize = "ask")]
    Ask,
    /// The price represent the last public price.
    #[strum(serialize = "last")]
    Last,
    /// The price represents the net value of the asset.
    #[strum(serialize = "nav")]
    NetValue,
    /// The price comes from a transaction in GnuCash.
    #[strum(serialize = "transaction")]
    Transaction,
    /// The source is unknown.
    #[strum(serialize = "unknown")]
    Unknown,
}

// TODO: Better messages (maybe with a variable?)

/// Errors that can occur wen listing objects.
#[derive(Debug, Error)]
pub enum ListError {
    /// An error has ocured when listing objects.
    #[error("Failed to list")]
    QueryError(diesel::result::Error),
}

impl Commodity {
    /// Lists commodities.
    pub fn list(&mut self, db: &mut Database) -> Result<Vec<Self>, ListError> {
        use diesel::prelude::*;
        use gnc_db_schema::commodities::dsl::*;

        commodities
            .select(Self::as_select())
            .load(&mut db.connection)
            .map_err(ListError::QueryError)
            .log_err()
    }

    /// Returns the namespace of the commodity.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the mnemonic of the commodity.
    pub fn mnemonic(&self) -> &str {
        &self.mnemonic
    }

    /// Returns the full name of the commodity.
    pub fn fullname(&self) -> Option<&str> {
        self.fullname.as_deref()
    }

    /// Returns the identifying code of the commodity.
    ///
    /// For currencies, this is the ISO 4217 numeric code.
    pub fn numeric_code(&self) -> Option<&str> {
        self.cusip.as_deref()
    }

    /// Returns the fraction into which the commodity can be divided.
    pub fn fraction(&self) -> i32 {
        self.fraction
    }

    /// Gets the scale to use for the commodity.
    pub fn scale(&self) -> i64 {
        BigDecimal::from(self.fraction)
            .inverse()
            .normalized()
            .fractional_digit_count()
    }

    /// Gets the scale to use for the commodity in quotes.
    ///
    /// Quotes generally use a scale that is more precise than the actual scale
    /// of the currency.
    pub fn scale_quote(&self) -> i64 {
        self.scale().max(4)
    }

    /// Returns whether to get price quotes for this commodity.
    pub fn quote_flag(&self) -> bool {
        self.quote_flag != 0
    }

    /// Returns the source to use to get price quotes.
    pub fn quote_source(&self) -> Option<&str> {
        self.quote_source.as_deref()
    }

    /// Returns the timezone used for price quotes.
    pub fn quote_tz(&self) -> Option<&str> {
        self.quote_tz.as_deref()
    }
}

impl Price {
    // TODO: Decide on when to join or not.

    /// Lists prices.
    pub fn list(&mut self, db: &mut Database) -> Result<Vec<Self>, ListError> {
        use diesel::prelude::*;
        use gnc_db_schema::prices::dsl::*;

        prices
            .select(Self::as_select())
            .load(&mut db.connection)
            .map_err(ListError::QueryError)
            .log_err()
    }

    /// Lists prices by symbol.
    pub fn list_by_symbol(
        symbol: &str,
        db: &mut Database,
    ) -> Result<Vec<(Self, Commodity, Commodity)>, ListError> {
        use diesel::prelude::*;
        use gnc_db_schema::{commodities, prices};

        diesel::alias!(commodities as currencies: Currency);

        prices::table
            .inner_join(
                commodities::table
                    .on(commodities::guid.eq(prices::commodity_guid)),
            )
            .inner_join(
                currencies.on(currencies
                    .fields(commodities::guid)
                    .eq(prices::currency_guid)),
            )
            .filter(commodities::mnemonic.eq(symbol))
            .order_by(prices::date.asc())
            .select((
                Self::as_select(),
                Commodity::as_select(),
                currencies.fields(
                    <Commodity as Selectable<Sqlite>>::construct_selection(),
                ),
            ))
            .load(&mut db.connection)
            .map_err(ListError::QueryError)
            .log_err()
    }

    /// Returns the commodity which is priced.
    // TODO: Better error.
    pub fn commodity(&self, db: &mut Database) -> Result<Commodity, ListError> {
        use diesel::prelude::*;
        use gnc_db_schema::commodities;

        commodities::table
            .find(&self.commodity_guid)
            .select(Commodity::as_select())
            .first(&mut db.connection)
            .map_err(ListError::QueryError)
            .log_err()
    }

    /// Returns the currency used for the value.
    // TODO: Better error.
    pub fn currency(&self, db: &mut Database) -> Result<Commodity, ListError> {
        use diesel::prelude::*;
        use gnc_db_schema::commodities;

        commodities::table
            .find(&self.currency_guid)
            .select(Commodity::as_select())
            .first(&mut db.connection)
            .map_err(ListError::QueryError)
            .log_err()
    }

    /// Returns the date of the price quote.
    ///
    /// # Panics
    ///
    /// This function panics if the date has an incorrect format in the
    /// database.
    #[expect(clippy::panic, reason = "an incorrect format is a bug")]
    pub fn date(&self) -> DateTime<Utc> {
        let Ok(date) =
            NaiveDateTime::parse_from_str(&self.date, "%Y-%m-%d %H:%M:%S")
        else {
            panic!("invalid date `{}`", self.date);
        };

        date.and_utc()
    }

    /// Returns the source of the quote.
    pub fn source(&self) -> Option<PriceSource> {
        self.source
            .as_deref()
            .map(|s| PriceSource::from_str(s).unwrap())
    }

    /// Returns the source of the price in string format.
    pub fn source_str(&self) -> &str {
        self.source.as_deref().unwrap_or_default()
    }

    /// Returns the type of price.
    pub fn r#type(&self) -> Option<PriceType> {
        self.type_
            .as_deref()
            .map(|s| PriceType::from_str(s).expect("invalid price source"))
    }

    /// Returns the type of price in string format.
    pub fn type_str(&self) -> &str {
        self.type_.as_deref().unwrap_or_default()
    }

    /// Returns the price.
    pub fn value(&self) -> BigDecimal {
        let num = BigDecimal::from(self.value_num);
        let denom = BigDecimal::from(self.value_denom);
        num / denom
    }
}
