use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::models::StartupItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    pub timestamp: DateTime<Utc>,
    pub items: Vec<StartupItem>,
    pub version: String,
    pub description: Option<String>,
}

impl Backup {
    pub fn new(items: Vec<StartupItem>, description: Option<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            items,
            version: env!("CARGO_PKG_VERSION").to_string(),
            description,
        }
    }

    pub fn filename(&self) -> String {
        format!("backup_{}.json", self.timestamp.format("%Y%m%d_%H%M%S"))
    }
}

/// Get the backup directory path
pub fn get_backup_dir() -> Result<PathBuf> {
    let backup_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("startup-checker")
        .join("backups");

    fs::create_dir_all(&backup_dir).map_err(Error::Io)?;

    Ok(backup_dir)
}

/// Create a backup of the current startup items
pub fn create_backup(items: &[StartupItem], description: Option<String>) -> Result<PathBuf> {
    let backup_dir = get_backup_dir()?;

    let backup = Backup::new(items.to_vec(), description);
    let filename = backup.filename();
    let path = backup_dir.join(&filename);

    let json = serde_json::to_string_pretty(&backup).map_err(Error::Serialization)?;
    fs::write(&path, json).map_err(Error::Io)?;

    Ok(path)
}

/// List all available backups
pub fn list_backups() -> Result<Vec<(PathBuf, Backup)>> {
    let backup_dir = get_backup_dir()?;
    let mut backups = Vec::new();

    if !backup_dir.exists() {
        return Ok(backups);
    }

    let entries = fs::read_dir(&backup_dir).map_err(Error::Io)?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.extension().is_some_and(|e| e == "json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(backup) = serde_json::from_str::<Backup>(&content) {
                    backups.push((path, backup));
                }
            }
        }
    }

    // Sort by timestamp, newest first
    backups.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));

    Ok(backups)
}

/// Restore from a backup file
pub fn restore_backup(backup_path: &PathBuf) -> Result<Backup> {
    let content = fs::read_to_string(backup_path).map_err(Error::Io)?;
    let backup: Backup = serde_json::from_str(&content).map_err(Error::Serialization)?;
    Ok(backup)
}

/// Get the most recent backup
pub fn get_latest_backup() -> Result<Option<(PathBuf, Backup)>> {
    let backups = list_backups()?;
    Ok(backups.into_iter().next())
}
