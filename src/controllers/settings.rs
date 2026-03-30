use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    models::settings::SettingDatabaseObject,
    services::settingsstore::{get_collection, get_setting, set_setting},
    DbPool,
};

async fn get(State(pool): State<DbPool>) -> Json<Vec<SettingDatabaseObject>> {
    let result: Vec<SettingDatabaseObject> = get_collection(&pool).await;
    Json(result)
}

async fn get_by_id(State(pool): State<DbPool>, Path(id): Path<String>) -> String {
    get_setting(&pool, id, String::from("1")).await //TODO: LIST
}

async fn post(State(pool): State<DbPool>, Json(obj): Json<Vec<SettingDatabaseObject>>) {
    for setting in obj {
        set_setting(&pool, setting.name, setting.value).await;
    }
}

pub fn router() -> axum::Router<DbPool> {
    axum::Router::new()
        .route("/", axum::routing::get(get).post(post))
        .route("/{id}", axum::routing::get(get_by_id))
}
