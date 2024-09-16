use rocket::{fairing::AdHoc, serde::json::Json};

use crate::{
    models::settings::{SettingDatabaseObject, SettingEntryObject},
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
async fn post(db: Db, obj: Json<SettingEntryObject>) {
    //TODO: no other practical way to work with the braindead API in the UI
    set_setting(&db, String::from("dailyRate"), obj.dailyRate.clone()).await;
    set_setting(&db, String::from("pay"), obj.pay.clone()).await;
    set_setting(&db, String::from("payday"), obj.payday.clone()).await;
    set_setting(
        &db,
        String::from("weekdaySaving"),
        obj.weekdaySaving.clone(),
    )
    .await;
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Settings", |rocket| async {
        rocket.mount(format!("/api/settings"), routes![get, get_by_id, post])
    })
}

/*
pub const PAYDAY: u32 = 30;
pub const WEEKDAY_SAVING: Decimal = dec!(25);
pub const DAILY_RATE: Decimal = dec!(40);
pub const TOTAL_PAY: Decimal = dec!(1000.00);
 */
