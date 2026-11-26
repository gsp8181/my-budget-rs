use axum::{extract::State, Json};
use serde::Serialize;

use crate::{
    services::{
        settingsstore::get_setting,
        timezone::{
            default_timezone, parse_timezone, system_timezone_raw, FALLBACK_TIMEZONE,
            TIMEZONE_SETTING,
        },
    },
    DbPool,
};

#[derive(Serialize)]
struct TimezoneInfo {
    /// Timezone reported by the host OS (None if it could not be read).
    system: Option<String>,
    /// Whether the host timezone is a valid IANA zone.
    system_valid: bool,
    /// Default used when no setting is stored: the host zone, or Europe/London if invalid.
    default: String,
    /// Value stored in the `timezone` setting.
    configured: String,
    /// Timezone actually used for calculations.
    effective: String,
    /// Current time in the effective timezone (RFC 3339).
    now: String,
    /// All selectable IANA timezone names.
    available: Vec<&'static str>,
}

async fn get(State(pool): State<DbPool>) -> Json<TimezoneInfo> {
    let system = system_timezone_raw();
    let system_valid = system.as_deref().and_then(parse_timezone).is_some();
    let default = default_timezone();

    let configured = get_setting(
        &pool,
        String::from(TIMEZONE_SETTING),
        default.name().to_string(),
    )
    .await;
    let effective = parse_timezone(&configured).unwrap_or(FALLBACK_TIMEZONE);

    Json(TimezoneInfo {
        system,
        system_valid,
        default: default.name().to_string(),
        configured,
        effective: effective.name().to_string(),
        now: chrono::Utc::now().with_timezone(&effective).to_rfc3339(),
        available: chrono_tz::TZ_VARIANTS.iter().map(|tz| tz.name()).collect(),
    })
}

pub fn router() -> axum::Router<DbPool> {
    axum::Router::new().route("/", axum::routing::get(get))
}
