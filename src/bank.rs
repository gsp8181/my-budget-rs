use std::io::Result;

use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use serde_json::Value;

use crate::helper::{get_attributes, re_json};
use crate::store::{
    delete_record_by_id, get_record_by_id, insert_record, modify_record_by_id, print_all_values,
};
use crate::structs::{Category, DBObj, DBObjIn, Db_Name};

const CATEGORY: Category = Category::bank;
const DB_NAME: Db_Name = Db_Name::credit;
const ATTRIBUTES: &str = "name,amount";

#[get("/")]
async fn get() -> Json<Vec<DBObj>> {
    let result: Vec<DBObj> = print_all_values(DB_NAME, CATEGORY, false);

    Json(result)
}

#[get("/<id>")]
async fn get_by_id(id: u32) -> Option<Json<DBObj>> {
    let result = get_record_by_id(DB_NAME, CATEGORY, id);

    match result {
        Some(v) => Some(Json(v)),
        None => None,
    }
}

#[post("/", format = "json", data = "<obj>")]
async fn post(obj: Json<DBObjIn>) -> Json<DBObj> {
    let result = insert_record(DB_NAME, CATEGORY, obj.0, get_attributes(ATTRIBUTES));

    Json(result)
}

#[put("/<id>", format = "json", data = "<obj>")]
async fn put(id: u32, obj: Json<DBObjIn>) -> Result<Json<DBObj>> {
    let result = modify_record_by_id(DB_NAME, CATEGORY, get_attributes(ATTRIBUTES), id, obj.0);

    re_json(result)
}

#[delete("/<id>")]
async fn delete(id: u32) -> Result<Value> {
    delete_record_by_id(DB_NAME, CATEGORY, id)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API Stage", |rocket| async {
        rocket.mount("/api/bank", routes![get, get_by_id, post, put, delete])
    })
}
