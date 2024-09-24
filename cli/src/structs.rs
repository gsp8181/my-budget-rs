use std::cmp::Ordering;

use cursive_table_view::TableViewItem;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicItem {
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
    pub today: Vec<JsonObject>, //Vec<JsonObject>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct JsonEntryObject {
    pub name: Option<String>,
    pub day: Option<String>,
    pub amount: Option<Decimal>,
    pub cardid: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum Category {
    bank,
    cardbalance,
    creditcard,
    cash,
    debt,
    misc,
    recurring,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum Db_Name {
    debit,
    credit,
}

//columns

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Name,
    Amount,
    Day,
    CardId,
}

impl BasicColumn {
    pub fn as_str(&self) -> &str {
        match *self {
            BasicColumn::Name => "Name",
            BasicColumn::Amount => "Amount",
            BasicColumn::Day => "Day Taken",
            BasicColumn::CardId => "Card Used",
        }
    }
}

impl TableViewItem<BasicColumn> for JsonObject {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Amount => format!("£{:.2}", self.amount),
            BasicColumn::Day => format!("{:?}", self.day), //TODO: fix option printinga
            BasicColumn::CardId => format!("{:?}", self.cardid), //TODO: fix option printing
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Amount => self.amount.cmp(&other.amount),
            BasicColumn::Day => self.day.cmp(&other.day),
            BasicColumn::CardId => self.cardid.cmp(&other.cardid),
        }
    }
}
