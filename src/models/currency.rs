use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = currency)]
pub struct Currency {
    pub id: Option<i32>,
    pub rate: String,
    pub symbol: String,
    pub name: String,
}

diesel::table! {
    use diesel::sql_types::{Integer, Nullable, Text};

    currency (id) {
        id -> Nullable<Integer>,
        rate -> Text,
        symbol -> Text,
        name -> Text,
    }
}
