use std::process::Command;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

use crate::error::Result;

/// Check if the current process is running with elevated (administrator) privileges
pub fn is_elevated() -> bool {
    unsafe {
        let mut token_handle = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut size,
        );

        if result.is_err() {
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}

/// Request elevation by restarting the process with admin privileges
pub fn request_elevation() -> Result<()> {
    let exe = std::env::current_exe()?;
    let args: Vec<String> = std::env::args().skip(1).collect();

    Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs",
                exe.display(),
                args.join(" ")
            ),
        ])
        .spawn()?;

    std::process::exit(0);
}

/// Display a warning if not running as admin
pub fn admin_warning() -> Option<String> {
    if !is_elevated() {
        Some(
            "Not running as Administrator. Some items cannot be modified.".to_string()
        )
    } else {
        None
    }
}
