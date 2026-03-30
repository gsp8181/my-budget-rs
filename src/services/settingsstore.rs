use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::models::settings::{settings, SettingDatabaseObject};
use crate::{AppError, DbPool};

pub async fn get_collection(pool: &DbPool) -> Vec<SettingDatabaseObject> {
    let conn = pool.get().await.expect("pool connection");
    conn.interact(move |conn| settings::table.load(conn))
        .await
        .expect("interact error")
        .expect("diesel error")
}

pub async fn print_all_values(pool: &DbPool) -> Result<Vec<SettingDatabaseObject>, AppError> {
    let conn = pool.get().await?;
    conn.interact(move |conn| settings::table.load(conn))
        .await?
        .map_err(AppError::from)
}

pub async fn get_setting(pool: &DbPool, setting_name: String, default_value: String) -> String {
    let setting_name_clone = setting_name.clone();
    let conn = pool.get().await.expect("pool connection");
    let setting_value: Option<String> = conn
        .interact(move |conn| {
            settings::table
                .filter(settings::name.eq(setting_name_clone))
                .select(settings::value)
                .first(conn)
                .optional()
        })
        .await
        .expect("interact error")
        .expect("diesel error");

    if let Some(value) = setting_value {
        value
    } else {
        set_setting(pool, setting_name, default_value.clone()).await;
        default_value
    }
}

//TODO: &str
pub async fn set_setting(pool: &DbPool, name: String, value: String) {
    //TODO: ensure that it is matching to an existing setting
    let name_clone = name.clone();
    let conn = pool.get().await.expect("pool connection");
    let setting: Option<SettingDatabaseObject> = conn
        .interact(move |conn| {
            settings::table
                .filter(settings::name.eq(name_clone))
                .first(conn)
                .optional()
        })
        .await
        .expect("interact error")
        .expect("diesel error");

    //TODO: test unique constant
    if let Some(object) = setting {
        let new_setting = SettingDatabaseObject {
            id: object.id,
            name: object.name.clone(),
            value,
        };
        let conn = pool.get().await.expect("pool connection");
        let _affected: SettingDatabaseObject = conn
            .interact(move |conn| {
                diesel::update(settings::table)
                    .filter(settings::name.eq(object.name))
                    .set(new_setting)
                    .returning(settings::all_columns)
                    .get_result(conn)
            })
            .await
            .expect("interact error")
            .expect("diesel error");
    } else {
        let new_obj = SettingDatabaseObject {
            id: None,
            name,
            value,
        };
        let conn = pool.get().await.expect("pool connection");
        let _result: SettingDatabaseObject = conn
            .interact(move |conn| {
                diesel::insert_into(settings::table)
                    .values(new_obj)
                    .returning(settings::all_columns)
                    .get_result(conn)
                    .expect("Error saving new setting")
            })
            .await
            .expect("interact error");
    }
}
