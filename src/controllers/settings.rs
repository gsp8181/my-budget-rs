use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::settings::SettingDatabaseObject,
    services::settingsstore::{get_collection, get_setting, set_setting},
    services::timezone::{parse_timezone, TIMEZONE_SETTING},
    DbPool,
};

async fn get(State(pool): State<DbPool>) -> Json<Vec<SettingDatabaseObject>> {
    let result: Vec<SettingDatabaseObject> = get_collection(&pool).await;
    Json(result)
}

async fn get_by_id(State(pool): State<DbPool>, Path(id): Path<String>) -> String {
    get_setting(&pool, id, String::from("1")).await //TODO: LIST
}

async fn post(
    State(pool): State<DbPool>,
    Json(obj): Json<Vec<SettingDatabaseObject>>,
) -> Result<(), (StatusCode, String)> {
    // Validate everything before writing anything, so a bad value doesn't half-save.
    for setting in &obj {
        if setting.name == TIMEZONE_SETTING && parse_timezone(&setting.value).is_none() {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Unknown timezone '{}'", setting.value),
            ));
        }
    }
    for setting in obj {
        let value = if setting.name == TIMEZONE_SETTING {
            setting.value.trim().to_string()
        } else {
            setting.value
        };
        set_setting(&pool, setting.name, value).await;
    }
    Ok(())
}

pub fn router() -> axum::Router<DbPool> {
    axum::Router::new()
        .route("/", axum::routing::get(get).post(post))
        .route("/{id}", axum::routing::get(get_by_id))
}
