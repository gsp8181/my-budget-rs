use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::{Build, Rocket};

use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::models::settings::{settings, SettingDatabaseObject};
use crate::Db;

use lazy_static::lazy_static;

use std::sync::Mutex;

pub type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

lazy_static! {
    static ref my_mutex: Mutex<i32> = Mutex::new(0i32);
}

pub async fn get_collection(db: &Db) -> Vec<SettingDatabaseObject> {
    let ids: Vec<SettingDatabaseObject> = db
        .run(move |conn| settings::table.load(conn))
        .await
        .unwrap();

    ids
}

pub async fn print_all_values(db: &Db) -> Result<Vec<SettingDatabaseObject>> {
    let ids: Vec<SettingDatabaseObject> = db.run(move |conn| settings::table.load(conn)).await?;

    Ok(ids)
}

pub async fn get_setting(db: &Db, setting: String, default: String) -> String {
    let settingName = setting.clone().to_string();

    let ids: Vec<String> = db
        .run(move |conn| {
            settings::table
                .filter(settings::name.eq(settingName))
                .select(settings::value)
                .load(conn)
        })
        .await
        .unwrap();

    if ids.len() == 1 {
        ids.first().unwrap().clone()
    } else if ids.len() == 0 {
        set_setting(&db, setting, default.clone());
        default.to_string()
    } else {
        panic!("Found two settings entries for a single setting")
    }
}

pub async fn set_setting(db: &Db, name: String, value: String) {
    //TODO: this is because theres something weird going on with the compiler, borrow check errors despite cloning
    let name2 = name.clone();

    let ids: Vec<SettingDatabaseObject> = db
        .run(move |conn| settings::table.filter(settings::name.eq(&name2)).load(conn))
        .await
        .unwrap();

        //TODO:test unique constant
    if ids.len() == 1 {
        let setting = ids.first().unwrap().clone();
        let newSetting = SettingDatabaseObject {
            id: setting.id,
            name: ids.first().unwrap().name.clone(),
            value: value.to_string(),
        };
        let affected: SettingDatabaseObject = db
            .run(move |conn| {
                diesel::update(settings::table)
                    .filter(settings::name.eq(ids.first().unwrap().name.clone()))
                    .set(newSetting)
                    .returning(settings::all_columns)
                    .get_result(conn)
            })
            .await
            .unwrap();
    } else if ids.len() == 0 {
        let new_obj = SettingDatabaseObject {
            id: None,
            name: name.clone(),
            value: value.to_string(),
        };

        let result: SettingDatabaseObject = db
            .run(move |conn| {
                diesel::insert_into(settings::table)
                    .values(new_obj)
                    .returning(settings::all_columns)
                    .get_result(conn)
                    .expect("Error saving new setting")
            })
            .await;
    } else {
        panic!("Found two settings entries for a single setting")
    }
}
