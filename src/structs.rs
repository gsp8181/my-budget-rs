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

#[derive(Debug, Serialize)]
pub(crate) struct PublicItem {
    pub amount: Decimal,
    pub remaining_week: Decimal,
    pub end_of_week: Decimal,
    pub full_weekend: Decimal,
    pub monthly_debits: Decimal,
    pub monthly_credits: Decimal,
    pub net_saved_this_month: Decimal,
    pub card_held_total: Decimal,
    pub net_saved_avg: Decimal,
    pub saved_this_year: Decimal,
    pub today: Vec<JsonObject>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct JsonObject {
    //todo: db credit or debit
    pub id: Option<i32>,
    pub oldId: Option<i32>,
    pub category: Category,
    pub name: String,
    pub day: Option<i32>,
    pub amount: Decimal,
    pub cardid: Option<i32>,
    pub dbName: Db_Name,
}

#[allow(non_snake_case)]
#[derive(
    Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset,
)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = item)]
pub struct DatabaseObject {
    //TODO: fix me
    //todo: db credit or debit
    pub id: Option<i32>,
    pub oldId: Option<i32>,
    pub category: Category,
    pub name: String,
    pub day: Option<i32>,
    pub amount: String,
    pub cardid: Option<i32>,
    pub dbName: Db_Name,
}

diesel::table! {
    use diesel::sql_types::*;
    use super::Category;
    use super::Db_Name;

    item (id) {
        id -> Nullable<Integer>,
        oldId -> Nullable<Integer>,
        category -> Text,
        name -> Text,
        day -> Nullable<Integer>,
        amount -> Text,
        cardid -> Nullable<Integer>,
        dbName -> Text
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct JsonEntryObject {
    pub name: Option<String>,
    pub day: Option<String>,
    pub amount: Option<Decimal>,
    pub cardid: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromSqlRow, AsExpression)]
#[allow(non_camel_case_types)]
#[diesel(sql_type = Text)]
pub enum Category {
    bank,
    cardbalance,
    creditcard,
    cash,
    debt,
    misc,
    recurring,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromSqlRow, AsExpression)]
#[allow(non_camel_case_types)]
#[diesel(sql_type = Text)]
pub enum Db_Name {
    debit,
    credit,
}

impl FromSql<Text, Sqlite> for Category {
    fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(t.as_str().try_into()?)
    }
}

impl ToSql<Text, Sqlite> for Category {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for Db_Name {
    fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(t.as_str().try_into()?)
    }
}

impl ToSql<Text, Sqlite> for Db_Name {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

impl fmt::Display for Db_Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Db_Name::debit => write!(f, "debit"),
            Db_Name::credit => write!(f, "credit"),
        }
    }
}

impl TryFrom<&str> for Db_Name {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "debit" => Ok(Db_Name::debit),
            "credit" => Ok(Db_Name::credit),
            _ => Err(format!("Unknown state: {}", value)),
        }
    }
}

impl From<DatabaseObject> for JsonObject {

    fn from(object: DatabaseObject) -> JsonObject {
        JsonObject {
            id: object.id,
            amount: match Decimal::from_str(object.amount.as_str()) {
                Ok(am) => am,
                Err(_) => dec!(0),
            },
            oldId: object.oldId,
            category: object.category,
            name: object.name,
            day: object.day,
            cardid: object.cardid,
            dbName: object.dbName,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::bank => write!(f, "bank"),
            Category::cardbalance => write!(f, "cardbalance"),
            Category::creditcard => write!(f, "creditcard"),
            Category::cash => write!(f, "cash"),
            Category::debt => write!(f, "debt"),
            Category::misc => write!(f, "misc"),
            Category::recurring => write!(f, "recurring"),
        }
    }
}

impl TryFrom<&str> for Category {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bank" => Ok(Category::bank),
            "cardbalance" => Ok(Category::cardbalance),
            "creditcard" => Ok(Category::creditcard),
            "cash" => Ok(Category::cash),
            "debt" => Ok(Category::debt),
            "misc" => Ok(Category::misc),
            "recurring" => Ok(Category::recurring),
            _ => Err(format!("Unknown state: {}", value)),
        }
    }
}