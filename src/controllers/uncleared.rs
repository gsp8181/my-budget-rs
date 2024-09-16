use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use crate::store::Result;

use crate::Db;

use crate::helper::get_attributes;
use crate::store::{
    delete_record_by_id, get_record_by_id, insert_record, modify_record_by_id, print_all_values,
};
use crate::structs::{Category, DatabaseObject, JsonEntryObject, Db_Name};

#[get("/")]
async fn get(db: Db) -> Json<Vec<DatabaseObject>> {
    let result: Vec<DatabaseObject> = print_all_values(db, DB_NAME, CATEGORY, false)
        .await
        .unwrap();

    Json(result)
}

#[get("/<id>")]
async fn get_by_id(db: Db, id: i32) -> Option<Json<DatabaseObject>> {
    let result = get_record_by_id(db, DB_NAME, CATEGORY, id).await;

    match result {
        Ok(v) => Some(Json(v)),
        Err(e) => None,
    }
}

#[post("/", format = "json", data = "<obj>")]
async fn post(db: Db, obj: Json<JsonEntryObject>) -> Json<DatabaseObject> {
    let result = insert_record(db, DB_NAME, CATEGORY, obj.0, get_attributes(ATTRIBUTES));

    Json(result.await.unwrap())
}

#[put("/<id>", format = "json", data = "<obj>")]
async fn put(db: Db, id: i32, obj: Json<JsonEntryObject>) -> Result<Json<DatabaseObject>> {
    let result = modify_record_by_id(db, DB_NAME, CATEGORY, get_attributes(ATTRIBUTES), id, obj.0);

    match result.await {
        Ok(v) => Ok(Json(v)),
        Err(e) => Err(e),
    }
}

#[delete("/<id>")]
async fn delete(db: Db, id: i32) -> Result<Option<()>> {
    delete_record_by_id(db, DB_NAME, CATEGORY, id).await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite(CONTROLLERNAME, |rocket| async {
        rocket.mount(
            format!("/api/{}", CONTROLLERNAME),
            routes![get, get_by_id, post, put, delete],
        )
    })
}

const CATEGORY: Category = Category::creditcard;
const DB_NAME: Db_Name = Db_Name::debit;
const ATTRIBUTES: &str = "name,amount,cardid"; //TODO:enum?
const CONTROLLERNAME: &str = "uncleared";
