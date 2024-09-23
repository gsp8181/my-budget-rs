use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
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
    pub today: Vec<Value>, //Vec<JsonObject>,
}
