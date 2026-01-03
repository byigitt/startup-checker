use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Registry access error: {0}")]
    Registry(String),

    #[error("Windows API error: {0}")]
    WindowsApi(#[from] windows::core::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    #[error("Item not found: {id}")]
    ItemNotFound { id: String },

    #[error("Backup failed: {reason}")]
    BackupFailed { reason: String },

    #[error("Restore failed: {reason}")]
    RestoreFailed { reason: String },

    #[error("Change failed and was rolled back")]
    ChangeFailed,

    #[error("COM initialization failed: {0}")]
    ComInitFailed(String),

    #[error("Service control manager access denied")]
    ScmAccessDenied,

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Task scheduler error: {0}")]
    TaskScheduler(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, Error>;
