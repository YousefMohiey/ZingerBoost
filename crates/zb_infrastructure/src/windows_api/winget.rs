use std::os::windows::process::CommandExt;
use std::process::Command;

use zb_shared::constants::CREATE_NO_WINDOW;

#[derive(Debug, Clone)]
pub struct WingetInstaller;

impl Default for WingetInstaller {
    fn default() -> Self {
        Self::new()
    }
}

impl WingetInstaller {
    pub fn new() -> Self {
        Self
    }

    pub fn is_available(&self) -> bool {
        Command::new("winget")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn install(&self, package_id: &str) -> Result<String, String> {
        let mut child = Command::new("winget")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "install",
                "--id",
                package_id,
                "--accept-source-agreements",
                "--accept-package-agreements",
                "--silent",
            ])
            .spawn()
            .map_err(|e| format!("Failed to run winget: {}", e))?;

        let timeout = std::time::Duration::from_secs(300); // 5 minutes
        let start = std::time::Instant::now();

        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        return Ok(format!("{} installed successfully", package_id));
                    } else {
                        return Err(format!("Winget exited with code: {:?}", status.code()));
                    }
                }
                Ok(None) => {
                    if start.elapsed() > timeout {
                        let _ = child.kill();
                        return Err(format!(
                            "Winget install timed out after {} seconds",
                            timeout.as_secs()
                        ));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                Err(e) => {
                    return Err(format!("Failed to wait for winget: {}", e));
                }
            }
        }
    }

    pub fn remove_appx(&self, package_family_name: &str) -> Result<String, String> {
        let output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "-NoProfile",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &format!(
                    "Get-AppxPackage *{}* | Remove-AppxPackage",
                    package_family_name
                ),
            ])
            .output()
            .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn remove_provisioned_appx(&self, package_name: &str) -> Result<String, String> {
        let output = Command::new("powershell").creation_flags(CREATE_NO_WINDOW)
            .args([
                "-NoProfile", "-WindowStyle", "Hidden", "-Command",
                &format!("Get-AppxProvisionedPackage -Online | Where-Object {{ $_.DisplayName -like '*{}*' }} | Remove-AppxProvisionedPackage -Online", package_name),
            ])
            .output()
            .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
