use std::fs;
use std::path::PathBuf;

use crate::error::{Error, Result};
use crate::models::{ItemStatus, SourceType, StartupItem};

use super::StartupSource;

const DISABLED_EXTENSION: &str = ".disabled";

pub struct StartupFolderScanner;

impl StartupFolderScanner {
    pub fn new() -> Self {
        Self
    }

    fn get_user_startup_folder() -> Option<PathBuf> {
        dirs::data_dir().map(|d| {
            d.join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Startup")
        })
    }

    fn get_all_users_startup_folder() -> PathBuf {
        PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup")
    }

    fn scan_folder(&self, folder: &PathBuf, source: SourceType) -> Vec<StartupItem> {
        let mut items = Vec::new();

        if !folder.exists() {
            return items;
        }

        let entries = match fs::read_dir(folder) {
            Ok(e) => e,
            Err(_) => return items,
        };

        for entry in entries.flatten() {
            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                continue;
            }

            let file_name = match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };

            // Skip desktop.ini and other system files
            if file_name.starts_with('.') || file_name.eq_ignore_ascii_case("desktop.ini") {
                continue;
            }

            // Check if disabled
            let (display_name, status) = if file_name.ends_with(DISABLED_EXTENSION) {
                let name = file_name.trim_end_matches(DISABLED_EXTENSION).to_string();
                (name, ItemStatus::Disabled)
            } else {
                // Remove extension for display
                let name = path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or(file_name.clone());
                (name, ItemStatus::Enabled)
            };

            // Get the target for shortcuts
            let command = if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("lnk"))
                || file_name.ends_with(".lnk.disabled")
            {
                // For .lnk files, we'd need to read the shortcut target
                // For now, just use the shortcut path
                path.display().to_string()
            } else {
                path.display().to_string()
            };

            let item = StartupItem::new(
                display_name,
                source,
                folder.display().to_string(),
                command,
            )
            .with_status(status);

            items.push(item);
        }

        items
    }
}

impl StartupSource for StartupFolderScanner {
    fn scan(&self) -> Result<Vec<StartupItem>> {
        let mut all_items = Vec::new();

        // User startup folder
        if let Some(user_folder) = Self::get_user_startup_folder() {
            all_items.extend(self.scan_folder(&user_folder, SourceType::StartupFolderUser));
        }

        // All users startup folder
        let all_users_folder = Self::get_all_users_startup_folder();
        all_items.extend(self.scan_folder(&all_users_folder, SourceType::StartupFolderAllUsers));

        Ok(all_items)
    }

    fn enable(&self, item: &StartupItem) -> Result<()> {
        let source_location = PathBuf::from(&item.source_location);

        // Find the disabled file
        let entries = fs::read_dir(&source_location).map_err(Error::Io)?;

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();

            // Check if this is the disabled version of our item
            if file_name.ends_with(DISABLED_EXTENSION) {
                let base_name = file_name.trim_end_matches(DISABLED_EXTENSION);
                if base_name.starts_with(&item.name) {
                    // Rename to remove .disabled extension
                    let new_path = source_location.join(base_name);
                    fs::rename(&path, &new_path).map_err(Error::Io)?;
                    return Ok(());
                }
            }
        }

        Err(Error::ItemNotFound {
            id: item.id.clone(),
        })
    }

    fn disable(&self, item: &StartupItem) -> Result<()> {
        let source_location = PathBuf::from(&item.source_location);

        // Find the enabled file
        let entries = fs::read_dir(&source_location).map_err(Error::Io)?;

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();

            // Skip already disabled files
            if file_name.ends_with(DISABLED_EXTENSION) {
                continue;
            }

            // Check if this is our item (match by stem)
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            if stem == item.name {
                // Rename to add .disabled extension
                let new_name = format!("{file_name}{DISABLED_EXTENSION}");
                let new_path = source_location.join(new_name);
                fs::rename(&path, &new_path).map_err(Error::Io)?;
                return Ok(());
            }
        }

        Err(Error::ItemNotFound {
            id: item.id.clone(),
        })
    }

    fn source_types(&self) -> Vec<SourceType> {
        vec![SourceType::StartupFolderUser, SourceType::StartupFolderAllUsers]
    }
}

impl Default for StartupFolderScanner {
    fn default() -> Self {
        Self::new()
    }
}
