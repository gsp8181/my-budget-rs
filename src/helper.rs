use crate::{models::item::DatabaseObject, services::itemstore::Result};

use rocket::serde::json::Json;

use crate::models::item::JsonObject;

pub fn re_json(result: Result<DatabaseObject>) -> Result<Json<DatabaseObject>> {
    match result {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err(e),
    }
}

pub fn get_attributes(attributes: &str) -> Vec<&str> {
    attributes.split(',').collect::<Vec<_>>()
}
