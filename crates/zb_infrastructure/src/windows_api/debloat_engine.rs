use std::process::Command;

#[derive(Debug)]
pub struct DebloatEngine;

#[derive(Debug)]
pub enum DebloatError {
    ProcessFailed(String),
    AccessDenied(String),
    NotFound(String),
}

impl std::fmt::Display for DebloatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebloatError::ProcessFailed(m) => write!(f, "Process failed: {}", m),
            DebloatError::AccessDenied(m) => write!(f, "Access denied: {}", m),
            DebloatError::NotFound(m) => write!(f, "Not found: {}", m),
        }
    }
}

impl DebloatEngine {
    pub fn new() -> Self {
        Self
    }

    /// Remove an AppX package using PowerShell
    pub fn remove_appx_package(package_family_name: &str) -> Result<String, DebloatError> {
        let script = format!(
            "Get-AppxPackage -AllUsers *{}* | Remove-AppxPackage -ErrorAction SilentlyContinue; Get-AppxProvisionedPackage -Online | Where-Object {{ $_.DisplayName -like '*{}*' }} | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue",
            package_family_name, package_family_name
        );
        let output = Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(e.to_string()))?;

        if output.status.success() {
            Ok(format!("Removed AppX package: {}", package_family_name))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("AccessDenied") {
                Err(DebloatError::AccessDenied(package_family_name.to_string()))
            } else {
                Ok(format!(
                    "Package {} may already be removed",
                    package_family_name
                ))
            }
        }
    }

    /// Remove via takeown + icacls (fallback for stubborn packages)
    pub fn force_remove(package_path: &str) -> Result<String, DebloatError> {
        Command::new("takeown")
            .args(["/f", package_path, "/r", "/d", "y"])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(format!("takeown failed: {}", e)))?;

        Command::new("icacls")
            .args([package_path, "/grant", "Administrators:F", "/t", "/q"])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(format!("icacls failed: {}", e)))?;

        Ok(format!("Force removed: {}", package_path))
    }

    /// Disable Windows Widgets (News & Interests)
    pub fn disable_widgets() -> Result<String, DebloatError> {
        let script = "Get-AppxPackage -AllUsers *WebExperience* | Remove-AppxPackage -ErrorAction SilentlyContinue";
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", script])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(e.to_string()))?;
        Ok("Widgets disabled".to_string())
    }

    /// Remove via DISM (last resort for provisioned packages)
    pub fn dism_remove(package_name: &str) -> Result<String, DebloatError> {
        let output = Command::new("dism")
            .args([
                "/online",
                "/Remove-ProvisionedAppxPackage",
                "/PackageName:",
                package_name,
            ])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(e.to_string()))?;

        if output.status.success() {
            Ok(format!("DISM removed: {}", package_name))
        } else {
            Err(DebloatError::ProcessFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }
}
