mod registry;
mod scheduled_tasks;
mod services;
mod startup_folder;

pub use registry::RegistryScanner;
pub use scheduled_tasks::TaskSchedulerScanner;
pub use services::ServicesScanner;
pub use startup_folder::StartupFolderScanner;

use crate::error::Result;
use crate::models::{ItemStatus, SourceType, StartupItem};

/// Trait for startup item sources
pub trait StartupSource: Send + Sync {
    /// Scan and return all startup items from this source
    fn scan(&self) -> Result<Vec<StartupItem>>;

    /// Enable a startup item
    fn enable(&self, item: &StartupItem) -> Result<()>;

    /// Disable a startup item
    fn disable(&self, item: &StartupItem) -> Result<()>;

    /// Get the source types this scanner handles
    fn source_types(&self) -> Vec<SourceType>;
}

/// Scan all sources and return combined results
pub fn scan_all_sources() -> Vec<StartupItem> {
    let mut items = Vec::new();

    // Registry
    let registry = RegistryScanner::new();
    if let Ok(registry_items) = registry.scan() {
        items.extend(registry_items);
    }

    // Startup folders
    let folders = StartupFolderScanner::new();
    if let Ok(folder_items) = folders.scan() {
        items.extend(folder_items);
    }

    // Scheduled tasks
    let tasks = TaskSchedulerScanner::new();
    if let Ok(task_items) = tasks.scan() {
        items.extend(task_items);
    }

    // Services
    let services = ServicesScanner::new();
    if let Ok(service_items) = services.scan() {
        items.extend(service_items);
    }

    items
}

/// Modify a startup item's status
pub fn modify_item(item: &StartupItem, new_status: ItemStatus) -> Result<()> {
    match item.source {
        SourceType::RegistryCurrentUserRun
        | SourceType::RegistryCurrentUserRunOnce
        | SourceType::RegistryLocalMachineRun
        | SourceType::RegistryLocalMachineRunOnce
        | SourceType::RegistryLocalMachineWow6432 => {
            let scanner = RegistryScanner::new();
            match new_status {
                ItemStatus::Enabled => scanner.enable(item),
                ItemStatus::Disabled => scanner.disable(item),
                ItemStatus::Unknown => Ok(()),
            }
        }
        SourceType::StartupFolderUser | SourceType::StartupFolderAllUsers => {
            let scanner = StartupFolderScanner::new();
            match new_status {
                ItemStatus::Enabled => scanner.enable(item),
                ItemStatus::Disabled => scanner.disable(item),
                ItemStatus::Unknown => Ok(()),
            }
        }
        SourceType::ScheduledTask => {
            let scanner = TaskSchedulerScanner::new();
            match new_status {
                ItemStatus::Enabled => scanner.enable(item),
                ItemStatus::Disabled => scanner.disable(item),
                ItemStatus::Unknown => Ok(()),
            }
        }
        SourceType::WindowsService => {
            let scanner = ServicesScanner::new();
            match new_status {
                ItemStatus::Enabled => scanner.enable(item),
                ItemStatus::Disabled => scanner.disable(item),
                ItemStatus::Unknown => Ok(()),
            }
        }
    }
}
