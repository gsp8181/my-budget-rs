use diesel::{
    deserialize::FromSql,
    prelude::AsChangeset,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
    AsExpression, FromSqlRow, Identifiable, Insertable, Queryable,
};
use rocket_sync_db_pools::diesel;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[allow(non_snake_case)]
#[derive(
    Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset
)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = settings)]
pub struct SettingDatabaseObject {
    pub id: Option<i32>,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
//TODO: this does not have to be a string, investigate optional
pub struct SettingEntryObject {
    pub dailyRate: String,
    pub pay: String,
    pub payday: String,
    pub weekdaySaving: String,
}

diesel::table! {
    use diesel::sql_types::*;

    settings (id) {
        id -> Nullable<Integer>,
        name -> Text,
        value -> Text,
    }
}

