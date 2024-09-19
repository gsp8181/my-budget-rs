use rocket::{fairing::AdHoc, serde::json::Json};

use crate::{
    models::settings::SettingDatabaseObject,
    services::settingsstore::{get_collection, get_setting, set_setting},
    Db,
};

#[get("/")]
async fn get(db: Db) -> Json<Vec<SettingDatabaseObject>> {
    let result: Vec<SettingDatabaseObject> = get_collection(&db).await;

    Json(result)
}

#[get("/<id>")]
async fn get_by_id(db: Db, id: String) -> String {
    get_setting(&db, id, String::from("1")).await //TODO: LIST
}

#[post("/", format = "json", data = "<obj>")]
async fn post(db: Db, obj: Json<Vec<SettingDatabaseObject>>) {
    for setting in obj.0 {
        set_setting(&db, setting.name, setting.value).await;
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Settings", |rocket| async {
        rocket.mount("/api/settings".to_string(), routes![get, get_by_id, post])
    })
}
