use diesel::{prelude::AsChangeset, Identifiable, Insertable, Queryable};
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = settings)]
//TODO: does not have to be a string
pub struct SettingDatabaseObject {
    pub id: Option<i32>,
    pub name: String,
    pub value: String,
}

diesel::table! {
    use diesel::sql_types::{Integer, Nullable, Text};

    settings (id) {
        id -> Nullable<Integer>,
        name -> Text,
        value -> Text,
    }
}
