use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::models::settings::{settings, SettingDatabaseObject};
use crate::Db;

pub type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

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

pub async fn get_setting(db: &Db, setting_name: String, default_value: String) -> String {
    let setting_name_clone = setting_name.clone();

    let setting_value: Option<String> = db
        .run(move |conn| {
            settings::table
                .filter(settings::name.eq(setting_name_clone))
                .select(settings::value)
                .first(conn)
                .optional()
        })
        .await
        .unwrap();

    if let Some(value) = setting_value { value } else {
        set_setting(db, setting_name, default_value.clone()).await;
        default_value
    }
}

//TODO: &str
pub async fn set_setting(db: &Db, name: String, value: String) {
    //TODO: ensure that it is matching to an existing setting
    let name_clone = name.clone();

    let setting: Option<SettingDatabaseObject> = db
        .run(move |conn| {
            settings::table
                .filter(settings::name.eq(name_clone))
                .first(conn)
                .optional()
        })
        .await
        .unwrap();

    //TODO:test unique constant
    if let Some(object) = setting {
        let new_setting = SettingDatabaseObject {
            id: object.id,
            name: object.name.clone(),
            value,
        };
        let affected: SettingDatabaseObject = db
            .run(move |conn| {
                diesel::update(settings::table)
                    .filter(settings::name.eq(object.name))
                    .set(new_setting)
                    .returning(settings::all_columns)
                    .get_result(conn)
            })
            .await
            .unwrap();
    } else {
        let new_obj = SettingDatabaseObject {
            id: None,
            name,
            value,
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
    }
}
