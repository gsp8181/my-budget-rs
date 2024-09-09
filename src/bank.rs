use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use serde_json::Value;

use crate::store::Result;

use crate::Db;

use crate::helper::{get_attributes, re_json};
use crate::store::{
    delete_record_by_id, get_record_by_id, insert_record, modify_record_by_id, print_all_values,
};
use crate::structs::{Category, DBObj, DBObjDBIntermediate, DBObjIn, Db_Name};

const CATEGORY: Category = Category::bank;
const DB_NAME: Db_Name = Db_Name::credit;
const ATTRIBUTES: &str = "name,amount";

#[get("/")]
async fn get(db: Db) -> Json<Vec<DBObjDBIntermediate>> {
    let result: Vec<DBObjDBIntermediate> = print_all_values(db, DB_NAME, CATEGORY, false).await.unwrap();

    Json(result)
}

#[get("/<id>")]
async fn get_by_id(db: Db, id: i32) -> Option<Json<DBObjDBIntermediate>> {
    let result = get_record_by_id(db, DB_NAME, CATEGORY, id).await;

    match result {Ok(v)=>Some(Json(v)),
    Err(e) => None, }
}

#[post("/", format = "json", data = "<obj>")]
async fn post(db: Db, obj: Json<DBObjIn>) -> Json<DBObjDBIntermediate> {
    let result = insert_record(db, DB_NAME, CATEGORY, obj.0, get_attributes(ATTRIBUTES));

    Json(result.await.unwrap())
    //re_json(result.await).unwrap()
}

#[put("/<id>", format = "json", data = "<obj>")]
async fn put(db: Db, id: i32, obj: Json<DBObjIn>) -> Result<Json<DBObjDBIntermediate>> {
    let result = modify_record_by_id(db, DB_NAME, CATEGORY, get_attributes(ATTRIBUTES), id, obj.0);

    match result.await {Ok(v)=>Ok(Json(v)),
        Err(e) => Err(e), }
}

#[delete("/<id>")]
async fn delete(db: Db, id: i32) -> Result<Option<()>> {
    delete_record_by_id(db, DB_NAME, CATEGORY, id).await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("API Stage", |rocket| async {
        rocket.mount("/api/bank", routes![get, get_by_id, post, put, delete])
    })
}
