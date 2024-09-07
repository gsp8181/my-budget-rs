use std::io::Result;
use rocket::serde::json::Json;

use crate::structs::DBObj;

pub fn re_json(result: Result<DBObj>) -> Result<Json<DBObj>>
{
    match result {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err(e),
    }
}

pub fn get_attributes(attributes:&str) -> Vec<&str>
{
    let attributes_vec: Vec<_> = attributes.split(',').collect();

    attributes_vec
}