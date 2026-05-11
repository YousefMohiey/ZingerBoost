use std::os::windows::process::CommandExt;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone)]
pub struct WingetInstaller;

impl WingetInstaller {
    pub fn new() -> Self {
        Self
    }

    pub fn is_available(&self) -> bool {
        Command::new("winget").creation_flags(CREATE_NO_WINDOW)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn install(&self, package_id: &str) -> Result<String, String> {
        let output = Command::new("winget").creation_flags(CREATE_NO_WINDOW)
            .args([
                "install",
                "--id",
                package_id,
                "--accept-source-agreements",
                "--accept-package-agreements",
                "--silent",
            ])
            .output()
            .map_err(|e| format!("Failed to run winget: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(format!("{} installed successfully", package_id))
        } else if stderr.contains("No installed package found")
            || stdout.contains("No installed package found")
        {
            Ok(format!("{} is already installed", package_id))
        } else {
            Err(format!("Winget error: {} {}", stdout, stderr))
        }
    }

    pub fn remove_appx(&self, package_family_name: &str) -> Result<String, String> {
        let output = Command::new("powershell").creation_flags(CREATE_NO_WINDOW)
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
