use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::core::{PCWSTR, PWSTR};
use windows::Win32::System::Services::{
    ChangeServiceConfigW, CloseServiceHandle, EnumServicesStatusExW, OpenSCManagerW, OpenServiceW,
    QueryServiceConfigW, ENUM_SERVICE_STATUS_PROCESSW, ENUM_SERVICE_TYPE, QUERY_SERVICE_CONFIGW,
    SC_ENUM_PROCESS_INFO, SC_MANAGER_ENUMERATE_SERVICE, SERVICE_AUTO_START, SERVICE_BOOT_START,
    SERVICE_CHANGE_CONFIG, SERVICE_DEMAND_START, SERVICE_ERROR, SERVICE_NO_CHANGE,
    SERVICE_QUERY_CONFIG, SERVICE_STATE_ALL, SERVICE_SYSTEM_START, SERVICE_WIN32,
};

use crate::error::{Error, Result};
use crate::models::{ItemStatus, SourceType, StartupItem};

use super::StartupSource;

pub struct ServicesScanner;

impl ServicesScanner {
    pub fn new() -> Self {
        Self
    }

    fn pwstr_to_string(ptr: PWSTR) -> String {
        if ptr.is_null() {
            return String::new();
        }

        unsafe {
            let len = (0..).take_while(|&i| *ptr.0.add(i) != 0).count();
            let slice = std::slice::from_raw_parts(ptr.0, len);
            OsString::from_wide(slice).to_string_lossy().to_string()
        }
    }

    fn pcwstr_to_string(ptr: PCWSTR) -> String {
        if ptr.is_null() {
            return String::new();
        }

        unsafe {
            let len = (0..).take_while(|&i| *ptr.0.add(i) != 0).count();
            let slice = std::slice::from_raw_parts(ptr.0, len);
            OsString::from_wide(slice).to_string_lossy().to_string()
        }
    }

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }
}

impl StartupSource for ServicesScanner {
    fn scan(&self) -> Result<Vec<StartupItem>> {
        let mut items = Vec::new();

        unsafe {
            // Open Service Control Manager
            let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ENUMERATE_SERVICE)
                .map_err(|_| Error::ScmAccessDenied)?;

            // First call to get required buffer size
            let mut bytes_needed = 0u32;
            let mut services_returned = 0u32;
            let mut resume_handle = 0u32;

            let _ = EnumServicesStatusExW(
                scm,
                SC_ENUM_PROCESS_INFO,
                SERVICE_WIN32,
                SERVICE_STATE_ALL,
                None,
                &mut bytes_needed,
                &mut services_returned,
                Some(&mut resume_handle),
                PCWSTR::null(),
            );

            if bytes_needed == 0 {
                let _ = CloseServiceHandle(scm);
                return Ok(items);
            }

            // Allocate buffer
            let mut buffer = vec![0u8; bytes_needed as usize];
            resume_handle = 0;

            let result = EnumServicesStatusExW(
                scm,
                SC_ENUM_PROCESS_INFO,
                SERVICE_WIN32,
                SERVICE_STATE_ALL,
                Some(&mut buffer),
                &mut bytes_needed,
                &mut services_returned,
                Some(&mut resume_handle),
                PCWSTR::null(),
            );

            if result.is_err() {
                let _ = CloseServiceHandle(scm);
                return Ok(items);
            }

            // Parse services
            let services = std::slice::from_raw_parts(
                buffer.as_ptr() as *const ENUM_SERVICE_STATUS_PROCESSW,
                services_returned as usize,
            );

            for service in services {
                let service_name = Self::pwstr_to_string(service.lpServiceName);
                let display_name = Self::pwstr_to_string(service.lpDisplayName);

                // Convert service name to wide string for OpenServiceW
                let service_name_wide = Self::to_wide(&service_name);

                // Open service to get config
                let service_handle = match OpenServiceW(
                    scm,
                    PCWSTR::from_raw(service_name_wide.as_ptr()),
                    SERVICE_QUERY_CONFIG,
                ) {
                    Ok(h) => h,
                    Err(_) => continue,
                };

                // Get service configuration
                let mut config_buffer = vec![0u8; 8192];
                let mut config_bytes_needed = 0u32;

                let config_result = QueryServiceConfigW(
                    service_handle,
                    Some(config_buffer.as_mut_ptr() as *mut QUERY_SERVICE_CONFIGW),
                    config_buffer.len() as u32,
                    &mut config_bytes_needed,
                );

                if config_result.is_err() {
                    let _ = CloseServiceHandle(service_handle);
                    continue;
                }

                let config = &*(config_buffer.as_ptr() as *const QUERY_SERVICE_CONFIGW);

                // Only include auto-start services
                let start_type = config.dwStartType;
                if start_type != SERVICE_AUTO_START
                    && start_type != SERVICE_BOOT_START
                    && start_type != SERVICE_SYSTEM_START
                {
                    let _ = CloseServiceHandle(service_handle);
                    continue;
                }

                let status = ItemStatus::Enabled;
                let binary_path = Self::pwstr_to_string(config.lpBinaryPathName);

                let item = StartupItem::new(
                    display_name.clone(),
                    SourceType::WindowsService,
                    service_name.clone(),
                    binary_path,
                )
                .with_status(status)
                .with_description(Some(format!("Service: {}", service_name)));

                items.push(item);

                let _ = CloseServiceHandle(service_handle);
            }

            let _ = CloseServiceHandle(scm);
        }

        Ok(items)
    }

    fn enable(&self, item: &StartupItem) -> Result<()> {
        unsafe {
            let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ENUMERATE_SERVICE)
                .map_err(|_| Error::ScmAccessDenied)?;

            let service_name_wide = Self::to_wide(&item.source_location);

            let service_handle = OpenServiceW(
                scm,
                PCWSTR::from_raw(service_name_wide.as_ptr()),
                SERVICE_CHANGE_CONFIG,
            )
            .map_err(|e| {
                let _ = CloseServiceHandle(scm);
                Error::PermissionDenied {
                    message: format!("Cannot modify service: {}", e),
                }
            })?;

            let result = ChangeServiceConfigW(
                service_handle,
                ENUM_SERVICE_TYPE(SERVICE_NO_CHANGE),
                SERVICE_AUTO_START,
                SERVICE_ERROR(SERVICE_NO_CHANGE),
                PCWSTR::null(),
                PCWSTR::null(),
                None,
                PCWSTR::null(),
                PCWSTR::null(),
                PCWSTR::null(),
                PCWSTR::null(),
            );

            let _ = CloseServiceHandle(service_handle);
            let _ = CloseServiceHandle(scm);

            result.map_err(|e| Error::PermissionDenied {
                message: format!("Failed to enable service: {}", e),
            })
        }
    }

    fn disable(&self, item: &StartupItem) -> Result<()> {
        unsafe {
            let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ENUMERATE_SERVICE)
                .map_err(|_| Error::ScmAccessDenied)?;

            let service_name_wide = Self::to_wide(&item.source_location);

            let service_handle = OpenServiceW(
                scm,
                PCWSTR::from_raw(service_name_wide.as_ptr()),
                SERVICE_CHANGE_CONFIG,
            )
            .map_err(|e| {
                let _ = CloseServiceHandle(scm);
                Error::PermissionDenied {
                    message: format!("Cannot modify service: {}", e),
                }
            })?;

            let result = ChangeServiceConfigW(
                service_handle,
                ENUM_SERVICE_TYPE(SERVICE_NO_CHANGE),
                SERVICE_DEMAND_START,
                SERVICE_ERROR(SERVICE_NO_CHANGE),
                PCWSTR::null(),
                PCWSTR::null(),
                None,
                PCWSTR::null(),
                PCWSTR::null(),
                PCWSTR::null(),
                PCWSTR::null(),
            );

            let _ = CloseServiceHandle(service_handle);
            let _ = CloseServiceHandle(scm);

            result.map_err(|e| Error::PermissionDenied {
                message: format!("Failed to disable service: {}", e),
            })
        }
    }

    fn source_types(&self) -> Vec<SourceType> {
        vec![SourceType::WindowsService]
    }
}

impl Default for ServicesScanner {
    fn default() -> Self {
        Self::new()
    }
}
