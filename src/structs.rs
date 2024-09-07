use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
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
    pub today: Vec<DBObj>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct DBObj {
    //todo: db credit or debit
    pub id: u32,
    pub oldId: Option<u32>,
    pub category: Category,
    pub name: String,
    pub day: Option<u32>,
    pub amount: Decimal,
    pub cardid: Option<u32>,
    pub dbName: Db_Name,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct DBObjIn {
    pub name: Option<String>,
    pub day: Option<u32>,
    pub amount: Option<Decimal>,
    pub cardid: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum Category {
    bank,
    cardbalance,
    creditcard,
    cash,
    debt,
    misc,
    recurring,
}

#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum Db_Name {
    debit,
    credit,
}

pub const PAYDAY: u32 = 30;
pub const WEEKDAY_SAVING: Decimal = dec!(25);
pub const DAILY_RATE: Decimal = dec!(40);
pub const TOTAL_PAY: Decimal = dec!(1000.00);
