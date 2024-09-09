use crate::{store::Result, structs::DBObjDBIntermediate};

use rocket::serde::json::Json;

use crate::structs::DBObj;

pub fn re_json(result: Result<DBObjDBIntermediate>) -> Result<Json<DBObjDBIntermediate>> {
    match result {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err(e),
    }
}

pub fn get_attributes(attributes: &str) -> Vec<&str> {
    let attributes_vec: Vec<_> = attributes.split(',').collect();

    attributes_vec
}
