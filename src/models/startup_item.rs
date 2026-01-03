use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

use super::{ItemStatus, SourceType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupItem {
    /// Unique identifier (hash of source + name + command)
    pub id: String,

    /// Display name of the startup item
    pub name: String,

    /// Source type (Registry, Folder, Task, Service)
    pub source: SourceType,

    /// Specific source location (e.g., registry key path or file path)
    pub source_location: String,

    /// Current status
    pub status: ItemStatus,

    /// Command line or executable path
    pub command: String,

    /// Publisher/Company name (if available)
    pub publisher: Option<String>,

    /// Description
    pub description: Option<String>,

    /// Whether this item requires admin to modify
    pub requires_admin: bool,

    /// File path of the executable (resolved)
    pub executable_path: Option<PathBuf>,

    /// Whether the executable file exists
    pub file_exists: bool,
}

impl StartupItem {
    pub fn new(
        name: String,
        source: SourceType,
        source_location: String,
        command: String,
    ) -> Self {
        let id = Self::generate_id(&source, &name, &command);
        let executable_path = Self::extract_executable_path(&command);
        let file_exists = executable_path
            .as_ref()
            .is_some_and(|p| p.exists());

        Self {
            id,
            name,
            source,
            source_location,
            status: ItemStatus::Enabled,
            command,
            publisher: None,
            description: None,
            requires_admin: source.requires_admin(),
            executable_path,
            file_exists,
        }
    }

    fn generate_id(source: &SourceType, name: &str, command: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}:{name}:{command}", source));
        let result = hasher.finalize();
        hex::encode(&result[..8]) // Use first 8 bytes for shorter ID
    }

    fn extract_executable_path(command: &str) -> Option<PathBuf> {
        let command = command.trim();

        // Handle quoted paths
        if command.starts_with('"') {
            if let Some(end) = command[1..].find('"') {
                let path = &command[1..=end];
                return Some(PathBuf::from(path));
            }
        }

        // Handle unquoted paths - take until first space or end
        let path = command.split_whitespace().next()?;

        // Expand environment variables
        let expanded = Self::expand_env_vars(path);
        Some(PathBuf::from(expanded))
    }

    fn expand_env_vars(path: &str) -> String {
        let mut result = path.to_string();

        // Common environment variables
        let env_vars = [
            ("%SystemRoot%", std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string())),
            ("%ProgramFiles%", std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string())),
            ("%ProgramFiles(x86)%", std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| "C:\\Program Files (x86)".to_string())),
            ("%USERPROFILE%", std::env::var("USERPROFILE").unwrap_or_default()),
            ("%APPDATA%", std::env::var("APPDATA").unwrap_or_default()),
            ("%LOCALAPPDATA%", std::env::var("LOCALAPPDATA").unwrap_or_default()),
        ];

        for (var, value) in env_vars {
            result = result.replace(var, &value);
            // Also handle lowercase
            result = result.replace(&var.to_lowercase(), &value);
        }

        result
    }

    pub fn with_status(mut self, status: ItemStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_publisher(mut self, publisher: Option<String>) -> Self {
        self.publisher = publisher;
        self
    }

    pub fn with_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn display_command(&self) -> String {
        if self.command.len() > 60 {
            format!("{}...", &self.command[..57])
        } else {
            self.command.clone()
        }
    }

    pub fn display_path(&self) -> String {
        self.executable_path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| self.command.clone())
    }
}

impl PartialEq for StartupItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for StartupItem {}

impl std::hash::Hash for StartupItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
