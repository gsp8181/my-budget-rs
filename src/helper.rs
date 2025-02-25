use crate::{models::item::DatabaseObject, services::itemstore::Result};

use rocket::serde::json::Json;
use std::fs::OpenOptions;
use std::io::Write;

pub fn re_json(result: Result<DatabaseObject>) -> Result<Json<DatabaseObject>> {
    match result {
        Ok(d) => Ok(Json(d)),
        Err(e) => Err(e),
    }
}

pub fn get_attributes(attributes: &str) -> Vec<&str> {
    attributes.split(',').collect::<Vec<_>>()
}

pub fn log_query(operation: &str, category: &str, query_details: &str, data: Option<&serde_json::Value>) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("query_log.txt")
    {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let data_str = data.map_or(String::new(), |d| format!("\nData: {}", serde_json::to_string_pretty(d).unwrap_or_default()));
        let log_entry = format!("[{}] {} - {} - {}{}\n", timestamp, operation, category, query_details, data_str);
        let _ = file.write_all(log_entry.as_bytes());
    }
}
