use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

const LOG_FILE: &str = "storage/transactions.log";
const MAX_LOG_SIZE: u64 = 1_048_576; // 1 MB
const MAX_ROTATED_FILES: u32 = 5;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub action: String,
    pub controller: String,
    pub item_id: Option<i32>,
    pub details: serde_json::Value,
}

fn rotate_log() {
    // Remove the oldest rotated file if it exists
    let oldest = PathBuf::from(format!("{}.{}", LOG_FILE, MAX_ROTATED_FILES));
    if oldest.exists() {
        let _ = fs::remove_file(&oldest);
    }

    // Shift each rotated file up by one (e.g. .4 → .5, .3 → .4, …, .1 → .2)
    for i in (1..MAX_ROTATED_FILES).rev() {
        let from = PathBuf::from(format!("{}.{}", LOG_FILE, i));
        let to = PathBuf::from(format!("{}.{}", LOG_FILE, i + 1));
        if from.exists() {
            let _ = fs::rename(&from, &to);
        }
    }

    // Rename the current log to .1
    let log_path = PathBuf::from(LOG_FILE);
    if log_path.exists() {
        let _ = fs::rename(&log_path, format!("{}.1", LOG_FILE));
    }
}

pub fn log_transaction<T: Serialize>(
    action: &str,
    controller: &str,
    item_id: Option<i32>,
    details: &T,
) {
    let entry = LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        action: action.to_string(),
        controller: controller.to_string(),
        item_id,
        details: serde_json::to_value(details).unwrap_or(serde_json::Value::Null),
    };

    let log_line = match serde_json::to_string(&entry) {
        Ok(line) => format!("{}\n", line),
        Err(_) => return,
    };

    let log_path = PathBuf::from(LOG_FILE);

    // Rotate if the current file would exceed the size limit
    if let Ok(metadata) = fs::metadata(&log_path) {
        if metadata.len() + log_line.len() as u64 > MAX_LOG_SIZE {
            rotate_log();
        }
    }

    // Ensure the storage directory exists
    if let Some(parent) = log_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Append the new entry
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = file.write_all(log_line.as_bytes());
    }
}

pub fn read_logs() -> Vec<LogEntry> {
    let mut all_lines: Vec<String> = Vec::new();

    // Collect lines in chronological order (oldest files first)
    for i in (1..=MAX_ROTATED_FILES).rev() {
        let rotated_path = PathBuf::from(format!("{}.{}", LOG_FILE, i));
        if let Ok(file) = File::open(&rotated_path) {
            let reader = BufReader::new(file);
            all_lines.extend(reader.lines().filter_map(|l| l.ok()));
        }
    }

    // Current log file contains the newest entries
    let log_path = PathBuf::from(LOG_FILE);
    if let Ok(file) = File::open(&log_path) {
        let reader = BufReader::new(file);
        all_lines.extend(reader.lines().filter_map(|l| l.ok()));
    }

    // Reverse to get newest-first order, then parse
    all_lines
        .into_iter()
        .rev()
        .filter_map(|line| serde_json::from_str::<LogEntry>(&line).ok())
        .collect()
}
