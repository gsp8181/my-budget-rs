//! Append-only plain-text transaction log of every create/update/delete,
//! written to `transactions.log` alongside the SQLite database file.
//!
//! Line format (timestamps are always GMT/UTC):
//! `2026-11-26T14:03:12Z<TAB>CREATE<TAB>item<TAB>{...json...}`

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use serde::Serialize;

const LOG_FILE_NAME: &str = "transactions.log";
const DEFAULT_DB_DIR: &str = "storage";

static LOG_PATH: OnceLock<PathBuf> = OnceLock::new();
static WRITE_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Create,
    Update,
    Delete,
}

impl Action {
    fn as_str(self) -> &'static str {
        match self {
            Action::Create => "CREATE",
            Action::Update => "UPDATE",
            Action::Delete => "DELETE",
        }
    }
}

/// Point the log at the folder containing the given SQLite database file.
/// Call once at startup; later calls are ignored.
pub fn init(db_url: &str) {
    let _ = LOG_PATH.set(log_path_for_db_url(Some(db_url)));
}

fn log_path_for_db_url(db_url: Option<&str>) -> PathBuf {
    let dir = match db_url {
        Some(url) => {
            let file = url.trim_start_matches("sqlite://").trim_start_matches("file:");
            match Path::new(file).parent() {
                Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
                _ => PathBuf::from("."),
            }
        }
        None => PathBuf::from(DEFAULT_DB_DIR),
    };
    dir.join(LOG_FILE_NAME)
}

fn log_path() -> &'static Path {
    LOG_PATH.get_or_init(|| log_path_for_db_url(None))
}

/// Record a CUD action. Failures to write the log are reported to stderr but
/// never fail the request itself.
pub fn record<T: Serialize>(action: Action, table: &str, payload: &T) {
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let body = serde_json::to_string(payload).unwrap_or_else(|e| format!("<unserializable: {e}>"));
    let line = format!("{timestamp}\t{}\t{table}\t{body}\n", action.as_str());

    let path = log_path();
    let _guard = WRITE_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let result = (|| {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
            .write_all(line.as_bytes())
    })();

    if let Err(e) = result {
        eprintln!("transaction log: failed to write to {}: {e}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_lives_next_to_database() {
        assert_eq!(
            log_path_for_db_url(Some("storage/database.sqlite")),
            PathBuf::from("storage/transactions.log")
        );
        assert_eq!(
            log_path_for_db_url(Some("/data/db/budget.sqlite")),
            PathBuf::from("/data/db/transactions.log")
        );
        assert_eq!(
            log_path_for_db_url(Some("database.sqlite")),
            PathBuf::from("./transactions.log")
        );
        assert_eq!(log_path_for_db_url(None), PathBuf::from("storage/transactions.log"));
    }
}
