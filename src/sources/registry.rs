use windows::core::PCWSTR;
use windows::Win32::Foundation::{ERROR_NO_MORE_ITEMS, ERROR_SUCCESS, WIN32_ERROR};
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegEnumValueW, RegOpenKeyExW, RegQueryValueExW,
    RegSetValueExW, HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS, KEY_READ,
    REG_CREATE_KEY_DISPOSITION, REG_OPTION_NON_VOLATILE, REG_SZ, REG_EXPAND_SZ,
};

use crate::error::{Error, Result};
use crate::models::{ItemStatus, SourceType, StartupItem};

use super::StartupSource;

const DISABLED_SUBKEY: &str = "AutorunsDisabled";

pub struct RegistryScanner;

impl RegistryScanner {
    pub fn new() -> Self {
        Self
    }

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn check_win32_error(error: WIN32_ERROR) -> std::result::Result<(), WIN32_ERROR> {
        if error == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(error)
        }
    }

    fn scan_registry_key(&self, root: HKEY, path: &str, source: SourceType) -> Vec<StartupItem> {
        let mut items = Vec::new();

        // Scan enabled items
        if let Ok(values) = self.enumerate_values(root, path) {
            for (name, command) in values {
                if !name.is_empty() && !command.is_empty() {
                    let item = StartupItem::new(name, source, path.to_string(), command)
                        .with_status(ItemStatus::Enabled);
                    items.push(item);
                }
            }
        }

        // Scan disabled items
        let disabled_path = format!("{path}\\{DISABLED_SUBKEY}");
        if let Ok(values) = self.enumerate_values(root, &disabled_path) {
            for (name, command) in values {
                if !name.is_empty() && !command.is_empty() {
                    let item = StartupItem::new(name, source, disabled_path.clone(), command)
                        .with_status(ItemStatus::Disabled);
                    items.push(item);
                }
            }
        }

        items
    }

    fn enumerate_values(&self, root: HKEY, path: &str) -> Result<Vec<(String, String)>> {
        let mut results = Vec::new();

        unsafe {
            let path_wide = Self::to_wide(path);
            let mut hkey = HKEY::default();

            let status = RegOpenKeyExW(
                root,
                PCWSTR::from_raw(path_wide.as_ptr()),
                0,
                KEY_READ,
                &mut hkey,
            );
            if status != ERROR_SUCCESS {
                return Ok(results);
            }

            let mut index = 0u32;
            loop {
                let mut name_buf = vec![0u16; 256];
                let mut name_len = name_buf.len() as u32;
                let mut data_buf = vec![0u8; 4096];
                let mut data_len = data_buf.len() as u32;
                let mut value_type = 0u32;

                let status = RegEnumValueW(
                    hkey,
                    index,
                    windows::core::PWSTR::from_raw(name_buf.as_mut_ptr()),
                    &mut name_len,
                    None,
                    Some(&mut value_type),
                    Some(data_buf.as_mut_ptr()),
                    Some(&mut data_len),
                );

                if status == ERROR_NO_MORE_ITEMS {
                    break;
                }

                if status != ERROR_SUCCESS {
                    index += 1;
                    continue;
                }

                // Only process string values (REG_SZ=1, REG_EXPAND_SZ=2)
                if value_type == REG_SZ.0 || value_type == REG_EXPAND_SZ.0 {
                    let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);

                    // Convert data to string (it's wide chars)
                    let data_u16: Vec<u16> = data_buf[..data_len as usize]
                        .chunks_exact(2)
                        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                        .collect();

                    let command = String::from_utf16_lossy(&data_u16)
                        .trim_end_matches('\0')
                        .to_string();

                    if !name.is_empty() {
                        results.push((name, command));
                    }
                }

                index += 1;
            }

            let _ = RegCloseKey(hkey);
        }

        Ok(results)
    }

    fn get_root_key(&self, source: SourceType) -> HKEY {
        match source {
            SourceType::RegistryCurrentUserRun | SourceType::RegistryCurrentUserRunOnce => {
                HKEY_CURRENT_USER
            }
            _ => HKEY_LOCAL_MACHINE,
        }
    }

    fn get_key_path(&self, source: SourceType) -> &'static str {
        match source {
            SourceType::RegistryCurrentUserRun => {
                r"Software\Microsoft\Windows\CurrentVersion\Run"
            }
            SourceType::RegistryCurrentUserRunOnce => {
                r"Software\Microsoft\Windows\CurrentVersion\RunOnce"
            }
            SourceType::RegistryLocalMachineRun => {
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"
            }
            SourceType::RegistryLocalMachineRunOnce => {
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce"
            }
            SourceType::RegistryLocalMachineWow6432 => {
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run"
            }
            _ => "",
        }
    }

    fn open_key(&self, root: HKEY, path: &str, write: bool) -> Result<HKEY> {
        unsafe {
            let path_wide = Self::to_wide(path);
            let mut hkey = HKEY::default();
            let access = if write { KEY_ALL_ACCESS } else { KEY_READ };

            let status = RegOpenKeyExW(
                root,
                PCWSTR::from_raw(path_wide.as_ptr()),
                0,
                access,
                &mut hkey,
            );

            Self::check_win32_error(status).map_err(|e| Error::PermissionDenied {
                message: format!("Cannot open registry key: {:?}", e),
            })?;

            Ok(hkey)
        }
    }

    fn create_key(&self, root: HKEY, path: &str) -> Result<HKEY> {
        unsafe {
            let path_wide = Self::to_wide(path);
            let mut hkey = HKEY::default();
            let mut disposition = REG_CREATE_KEY_DISPOSITION::default();

            let status = RegCreateKeyExW(
                root,
                PCWSTR::from_raw(path_wide.as_ptr()),
                0,
                PCWSTR::null(),
                REG_OPTION_NON_VOLATILE,
                KEY_ALL_ACCESS,
                None,
                &mut hkey,
                Some(&mut disposition),
            );

            Self::check_win32_error(status).map_err(|e| Error::PermissionDenied {
                message: format!("Cannot create registry key: {:?}", e),
            })?;

            Ok(hkey)
        }
    }

    fn get_value(&self, hkey: HKEY, name: &str) -> Result<String> {
        unsafe {
            let name_wide = Self::to_wide(name);
            let mut data_buf = vec![0u8; 4096];
            let mut data_len = data_buf.len() as u32;

            let status = RegQueryValueExW(
                hkey,
                PCWSTR::from_raw(name_wide.as_ptr()),
                None,
                None,
                Some(data_buf.as_mut_ptr()),
                Some(&mut data_len),
            );

            Self::check_win32_error(status).map_err(|e| Error::ItemNotFound {
                id: format!("Registry value not found: {:?}", e),
            })?;

            let data_u16: Vec<u16> = data_buf[..data_len as usize]
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();

            Ok(String::from_utf16_lossy(&data_u16)
                .trim_end_matches('\0')
                .to_string())
        }
    }

    fn set_value(&self, hkey: HKEY, name: &str, value: &str) -> Result<()> {
        unsafe {
            let name_wide = Self::to_wide(name);
            let value_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
            let data: Vec<u8> = value_wide
                .iter()
                .flat_map(|&w| w.to_le_bytes())
                .collect();

            let status = RegSetValueExW(
                hkey,
                PCWSTR::from_raw(name_wide.as_ptr()),
                0,
                REG_SZ,
                Some(&data),
            );

            Self::check_win32_error(status).map_err(|e| Error::PermissionDenied {
                message: format!("Cannot set registry value: {:?}", e),
            })?;

            Ok(())
        }
    }

    fn delete_value(&self, hkey: HKEY, name: &str) -> Result<()> {
        unsafe {
            let name_wide = Self::to_wide(name);

            let status = RegDeleteValueW(hkey, PCWSTR::from_raw(name_wide.as_ptr()));

            Self::check_win32_error(status).map_err(|e| Error::PermissionDenied {
                message: format!("Cannot delete registry value: {:?}", e),
            })?;

            Ok(())
        }
    }
}

impl StartupSource for RegistryScanner {
    fn scan(&self) -> Result<Vec<StartupItem>> {
        let mut all_items = Vec::new();

        all_items.extend(self.scan_registry_key(
            HKEY_CURRENT_USER,
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            SourceType::RegistryCurrentUserRun,
        ));

        all_items.extend(self.scan_registry_key(
            HKEY_CURRENT_USER,
            r"Software\Microsoft\Windows\CurrentVersion\RunOnce",
            SourceType::RegistryCurrentUserRunOnce,
        ));

        all_items.extend(self.scan_registry_key(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            SourceType::RegistryLocalMachineRun,
        ));

        all_items.extend(self.scan_registry_key(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
            SourceType::RegistryLocalMachineRunOnce,
        ));

        all_items.extend(self.scan_registry_key(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run",
            SourceType::RegistryLocalMachineWow6432,
        ));

        Ok(all_items)
    }

    fn enable(&self, item: &StartupItem) -> Result<()> {
        let root = self.get_root_key(item.source);
        let base_path = self.get_key_path(item.source);
        let disabled_path = format!("{base_path}\\{DISABLED_SUBKEY}");

        // Open disabled key and get value
        let disabled_key = self.open_key(root, &disabled_path, true)?;
        let value = self.get_value(disabled_key, &item.name)?;

        // Create/open enabled key and set value
        let enabled_key = self.create_key(root, base_path)?;
        self.set_value(enabled_key, &item.name, &value)?;

        // Delete from disabled
        self.delete_value(disabled_key, &item.name)?;

        unsafe {
            let _ = RegCloseKey(disabled_key);
            let _ = RegCloseKey(enabled_key);
        }

        Ok(())
    }

    fn disable(&self, item: &StartupItem) -> Result<()> {
        let root = self.get_root_key(item.source);
        let base_path = self.get_key_path(item.source);
        let disabled_path = format!("{base_path}\\{DISABLED_SUBKEY}");

        // Open enabled key and get value
        let enabled_key = self.open_key(root, base_path, true)?;
        let value = self.get_value(enabled_key, &item.name)?;

        // Create/open disabled key and set value
        let disabled_key = self.create_key(root, &disabled_path)?;
        self.set_value(disabled_key, &item.name, &value)?;

        // Delete from enabled
        self.delete_value(enabled_key, &item.name)?;

        unsafe {
            let _ = RegCloseKey(disabled_key);
            let _ = RegCloseKey(enabled_key);
        }

        Ok(())
    }

    fn source_types(&self) -> Vec<SourceType> {
        vec![
            SourceType::RegistryCurrentUserRun,
            SourceType::RegistryCurrentUserRunOnce,
            SourceType::RegistryLocalMachineRun,
            SourceType::RegistryLocalMachineRunOnce,
            SourceType::RegistryLocalMachineWow6432,
        ]
    }
}

impl Default for RegistryScanner {
    fn default() -> Self {
        Self::new()
    }
}
