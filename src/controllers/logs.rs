use rocket::{fairing::AdHoc, serde::json::Json};

use crate::services::logstore::{read_logs, LogEntry};

#[get("/?<limit>")]
fn get_logs(limit: Option<usize>) -> Json<Vec<LogEntry>> {
    let entries = read_logs();
    let entries = match limit {
        Some(n) => entries.into_iter().take(n).collect(),
        None => entries,
    };
    Json(entries)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Logs", |rocket| async {
        rocket.mount("/api/logs", routes![get_logs])
    })
}
