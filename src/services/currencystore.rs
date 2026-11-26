use diesel::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::str::FromStr;

use crate::models::currency::{currency, Currency};
use crate::services::txlog::{self, Action};
use crate::{AppError, DbPool};

pub async fn get_all_currencies(pool: &DbPool) -> Result<Vec<Currency>, AppError> {
    let conn = pool.get().await?;
    conn.interact(|conn| currency::table.order(currency::id.asc()).load(conn))
        .await?
        .map_err(AppError::from)
}

/// Build a map of currency_id -> rate for fast lookup during calculations.
pub async fn build_currency_rate_map(pool: &DbPool) -> HashMap<i32, Decimal> {
    let currencies = get_all_currencies(pool).await.unwrap_or_default();
    let mut map = HashMap::new();
    for c in currencies {
        if let (Some(id), Ok(rate)) = (c.id, Decimal::from_str(&c.rate)) {
            map.insert(id, rate);
        }
    }
    map
}

/// Returns the id of the oldest (lowest id) currency, defaulting to 1.
pub async fn get_default_currency_id(pool: &DbPool) -> i32 {
    let conn = pool.get().await.expect("pool connection");
    let result: Option<Option<i32>> = conn
        .interact(|conn| {
            currency::table
                .order(currency::id.asc())
                .select(currency::id)
                .first::<Option<i32>>(conn)
                .optional()
        })
        .await
        .expect("interact error")
        .expect("diesel error");
    result.flatten().unwrap_or(1)
}

pub async fn insert_currency(
    pool: &DbPool,
    rate: String,
    symbol: String,
    name: String,
) -> Result<Currency, AppError> {
    let new_currency = Currency {
        id: None,
        rate,
        symbol,
        name,
    };
    let conn = pool.get().await?;
    let created: Currency = conn
        .interact(move |conn| {
            diesel::insert_into(currency::table)
                .values(&new_currency)
                .returning(currency::all_columns)
                .get_result(conn)
        })
        .await?
        .map_err(AppError::from)?;
    txlog::record(Action::Create, "currency", &created);
    Ok(created)
}

pub async fn update_currency(
    pool: &DbPool,
    id: i32,
    rate: String,
    symbol: String,
    name: String,
) -> Result<Currency, AppError> {
    let conn = pool.get().await?;
    let updated: Currency = conn
        .interact(move |conn| {
            diesel::update(currency::table.filter(currency::id.eq(id)))
                .set((
                    currency::rate.eq(rate),
                    currency::symbol.eq(symbol),
                    currency::name.eq(name),
                ))
                .returning(currency::all_columns)
                .get_result(conn)
        })
        .await?
        .map_err(AppError::from)?;
    txlog::record(Action::Update, "currency", &updated);
    Ok(updated)
}

pub async fn delete_currency(pool: &DbPool, id: i32) -> Result<(), AppError> {
    let conn = pool.get().await?;
    let result = conn
        .interact(move |conn| {
            diesel::delete(currency::table.filter(currency::id.eq(id)))
                .returning(currency::all_columns)
                .get_result::<Currency>(conn)
                .optional()
        })
        .await?;

    match result {
        Ok(deleted) => {
            if let Some(c) = &deleted {
                txlog::record(Action::Delete, "currency", c);
            }
            Ok(())
        }
        Err(diesel::result::Error::DatabaseError(_, ref info))
            if info.message().contains("Cannot delete currency") =>
        {
            Err(AppError::Conflict(info.message().to_string()))
        }
        Err(e) => Err(AppError::from(e)),
    }
}
