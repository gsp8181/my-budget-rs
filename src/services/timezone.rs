use chrono_tz::Tz;

use crate::{services::settingsstore::get_setting, DbPool};

pub const TIMEZONE_SETTING: &str = "timezone";
pub const FALLBACK_TIMEZONE: Tz = chrono_tz::Europe::London;

/// The raw timezone name reported by the host OS, if it could be read at all.
pub fn system_timezone_raw() -> Option<String> {
    iana_time_zone::get_timezone().ok()
}

/// Parses a timezone name into a `Tz`, returning `None` if it isn't a known IANA zone.
pub fn parse_timezone(name: &str) -> Option<Tz> {
    name.trim().parse::<Tz>().ok()
}

/// The host timezone if it is a valid IANA zone, otherwise Europe/London.
pub fn default_timezone() -> Tz {
    system_timezone_raw()
        .and_then(|name| parse_timezone(&name))
        .unwrap_or(FALLBACK_TIMEZONE)
}

/// The timezone used for budget calculations: the stored setting, falling back to
/// Europe/London if the stored value is not a valid IANA zone.
pub async fn get_configured_timezone(pool: &DbPool) -> Tz {
    let raw = get_setting(
        pool,
        String::from(TIMEZONE_SETTING),
        default_timezone().name().to_string(),
    )
    .await;
    parse_timezone(&raw).unwrap_or(FALLBACK_TIMEZONE)
}
