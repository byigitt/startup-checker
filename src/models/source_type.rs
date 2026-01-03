use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum SourceType {
    RegistryCurrentUserRun,
    RegistryCurrentUserRunOnce,
    RegistryLocalMachineRun,
    RegistryLocalMachineRunOnce,
    RegistryLocalMachineWow6432,
    StartupFolderUser,
    StartupFolderAllUsers,
    ScheduledTask,
    WindowsService,
}

impl SourceType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::RegistryCurrentUserRun => "Registry (HKCU\\Run)",
            Self::RegistryCurrentUserRunOnce => "Registry (HKCU\\RunOnce)",
            Self::RegistryLocalMachineRun => "Registry (HKLM\\Run)",
            Self::RegistryLocalMachineRunOnce => "Registry (HKLM\\RunOnce)",
            Self::RegistryLocalMachineWow6432 => "Registry (HKLM\\Wow6432)",
            Self::StartupFolderUser => "Startup Folder (User)",
            Self::StartupFolderAllUsers => "Startup Folder (All Users)",
            Self::ScheduledTask => "Scheduled Tasks",
            Self::WindowsService => "Windows Services",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::RegistryCurrentUserRun => "HKCU\\Run",
            Self::RegistryCurrentUserRunOnce => "HKCU\\RunOnce",
            Self::RegistryLocalMachineRun => "HKLM\\Run",
            Self::RegistryLocalMachineRunOnce => "HKLM\\RunOnce",
            Self::RegistryLocalMachineWow6432 => "HKLM\\Wow6432",
            Self::StartupFolderUser => "User Startup",
            Self::StartupFolderAllUsers => "All Users Startup",
            Self::ScheduledTask => "Tasks",
            Self::WindowsService => "Services",
        }
    }

    pub fn requires_admin(&self) -> bool {
        matches!(
            self,
            Self::RegistryLocalMachineRun
                | Self::RegistryLocalMachineRunOnce
                | Self::RegistryLocalMachineWow6432
                | Self::StartupFolderAllUsers
                | Self::WindowsService
        )
    }

    pub fn is_registry(&self) -> bool {
        matches!(
            self,
            Self::RegistryCurrentUserRun
                | Self::RegistryCurrentUserRunOnce
                | Self::RegistryLocalMachineRun
                | Self::RegistryLocalMachineRunOnce
                | Self::RegistryLocalMachineWow6432
        )
    }

    pub fn is_startup_folder(&self) -> bool {
        matches!(self, Self::StartupFolderUser | Self::StartupFolderAllUsers)
    }

    pub fn registry_path(&self) -> Option<&'static str> {
        match self {
            Self::RegistryCurrentUserRun => {
                Some(r"Software\Microsoft\Windows\CurrentVersion\Run")
            }
            Self::RegistryCurrentUserRunOnce => {
                Some(r"Software\Microsoft\Windows\CurrentVersion\RunOnce")
            }
            Self::RegistryLocalMachineRun => {
                Some(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run")
            }
            Self::RegistryLocalMachineRunOnce => {
                Some(r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce")
            }
            Self::RegistryLocalMachineWow6432 => {
                Some(r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run")
            }
            _ => None,
        }
    }

    pub fn all() -> &'static [SourceType] {
        &[
            Self::RegistryCurrentUserRun,
            Self::RegistryCurrentUserRunOnce,
            Self::RegistryLocalMachineRun,
            Self::RegistryLocalMachineRunOnce,
            Self::RegistryLocalMachineWow6432,
            Self::StartupFolderUser,
            Self::StartupFolderAllUsers,
            Self::ScheduledTask,
            Self::WindowsService,
        ]
    }
}
