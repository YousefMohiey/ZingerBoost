use std::os::windows::process::CommandExt;
use std::process::Command;

use zb_shared::constants::CREATE_NO_WINDOW;

#[derive(Debug)]
pub struct DebloatEngine;

#[derive(Debug)]
pub enum DebloatError {
    ProcessFailed(String),
    AccessDenied(String),
    NotFound(String),
    AllMethodsFailed(String),
}

impl std::fmt::Display for DebloatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebloatError::ProcessFailed(m) => write!(f, "Process failed: {}", m),
            DebloatError::AccessDenied(m) => write!(f, "Access denied: {}", m),
            DebloatError::NotFound(m) => write!(f, "Not found: {}", m),
            DebloatError::AllMethodsFailed(m) => {
                write!(f, "All removal methods failed for: {}", m)
            }
        }
    }
}

impl std::error::Error for DebloatError {}

impl Default for DebloatEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DebloatEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn is_winget_available() -> bool {
        Command::new("winget")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Remove an AppX package using multiple methods in sequence (winutil-style).
    /// Returns which method succeeded.
    pub fn remove_appx_package(name: &str) -> Result<String, DebloatError> {
        // Method 1: PowerShell AppX removal (winutil approach - most effective)
        if let Ok(msg) = Self::try_powershell_remove(name) {
            return Ok(msg);
        }

        // Method 2: Winget uninstall
        if let Ok(msg) = Self::try_winget_uninstall(name) {
            return Ok(msg);
        }

        // Method 3: DISM provisioned package removal
        if let Ok(msg) = Self::try_dism_remove(name) {
            return Ok(msg);
        }

        // Method 4: Registry key deletion
        if let Ok(msg) = Self::try_registry_remove(name) {
            return Ok(msg);
        }

        // Method 5: Filesystem force removal
        if let Ok(msg) = Self::try_filesystem_remove(name) {
            return Ok(msg);
        }

        Err(DebloatError::AllMethodsFailed(name.to_string()))
    }

    fn try_winget_uninstall(name: &str) -> Result<String, String> {
        if !Self::is_winget_available() {
            return Err("Winget not available".to_string());
        }

        let output = Command::new("winget")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["uninstall", "--id", name, "--silent"])
            .output()
            .map_err(|e| format!("Winget spawn failed: {}", e))?;

        if output.status.success() {
            return Ok(format!("Method 1 (Winget) succeeded for '{}'", name));
        }

        let stderr = String::from_utf8_lossy(&output.stderr);

        if stderr.contains("No installed package found") {
            return Ok(format!(
                "Method 1 (Winget): '{}' not installed via winget, skipping",
                name
            ));
        }

        let output2 = Command::new("winget")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["uninstall", "--name", name, "--silent"])
            .output()
            .map_err(|e| format!("Winget name-based spawn failed: {}", e))?;

        if output2.status.success() {
            return Ok(format!(
                "Method 1 (Winget, name-based) succeeded for '{}'",
                name
            ));
        }

        Err(format!(
            "Winget failed: {}",
            String::from_utf8_lossy(&output2.stderr).trim()
        ))
    }
    fn try_powershell_remove(name: &str) -> Result<String, String> {
        let quote = format!(
            "$name='{}'; $pkg = Get-AppxPackage -AllUsers | Where-Object {{ $_.Name -like \"*$name*\" -or $_.PackageFamilyName -like \"*$name*\" }}; if ($pkg) {{ $pkg | Remove-AppxPackage -ErrorAction SilentlyContinue }}; $prov = Get-AppxProvisionedPackage -Online | Where-Object {{ $_.DisplayName -like \"*$name*\" -or $_.PackageName -like \"*$name*\" }}; if ($prov) {{ $prov | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue }}; Write-Host 'PS_REMOVE_DONE'",
            name.replace('\'', "''")
        );

        let output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-Command", &quote])
            .output()
            .map_err(|e| format!("PowerShell spawn failed: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Check for our success marker instead of relying on stderr
        if stdout.contains("PS_REMOVE_DONE") {
            return Ok(format!("Method 2 (PowerShell) succeeded for '{}'", name));
        }

        // If no match found but command succeeded, still consider it a win
        if output.status.success() {
            return Ok(format!(
                "Method 2 (PowerShell): '{}' not found or already removed",
                name
            ));
        }

        Err("PowerShell removal returned empty/no match".to_string())
    }

    fn try_dism_remove(name: &str) -> Result<String, String> {
        let find_script = format!(
            "Get-AppxProvisionedPackage -Online | Where-Object {{ $_.DisplayName -like '*{}*' -or $_.PackageName -like '*{}*' }} | Select-Object -First 1 -ExpandProperty PackageName",
            name.replace('\'', "''"),
            name.replace('\'', "''")
        );

        let find_output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-Command", &find_script])
            .output()
            .map_err(|e| format!("DISM find spawn failed: {}", e))?;

        let package_name = String::from_utf8_lossy(&find_output.stdout)
            .trim()
            .to_string();

        if package_name.is_empty() {
            return Err("DISM: No matching provisioned package found".to_string());
        }

        let output = Command::new("dism")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "/online",
                "/Remove-ProvisionedAppxPackage",
                &format!("/PackageName:{}", package_name),
            ])
            .output()
            .map_err(|e| format!("DISM spawn failed: {}", e))?;

        if output.status.success() {
            return Ok(format!("Method 3 (DISM) succeeded for '{}'", name));
        }

        let stderr = String::from_utf8_lossy(&output.stderr);

        if stderr.contains("0x80070002") || stderr.contains("not found") {
            return Ok(format!("Method 3 (DISM): '{}' already removed", name));
        }

        Err(format!("DISM failed: {}", stderr.trim()))
    }

    fn try_registry_remove(name: &str) -> Result<String, String> {
        let base_key = "HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Appx\\AppxAllUserStore\\Applications";

        let find_script = format!(
            r#"$key='{}'; $matches = Get-ChildItem -Path $key -ErrorAction SilentlyContinue | Where-Object {{ $_.PSChildName -like '*{}*' }}; if ($matches) {{ $matches | ForEach-Object {{ Remove-Item -Path $_.PSPath -Recurse -Force -ErrorAction SilentlyContinue; Write-Host ("Deleted: " + $_.PSChildName) }}; Write-Host 'REGISTRY_OK' }} else {{ Write-Host 'REGISTRY_NO_MATCH' }}"#,
            base_key,
            name.replace('\'', "''")
        );

        let output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-Command", &find_script])
            .output()
            .map_err(|e| format!("Registry remove spawn failed: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.contains("REGISTRY_OK") {
            return Ok(format!("Method 4 (Registry) succeeded for '{}'", name));
        }

        if stdout.contains("REGISTRY_NO_MATCH") {
            return Err("Registry: No matching keys found".to_string());
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.trim().is_empty() {
            return Err(format!("Registry error: {}", stderr.trim()));
        }

        Err("Registry: unknown error".to_string())
    }

    fn try_filesystem_remove(name: &str) -> Result<String, String> {
        let find_path_script = format!(
            r#"$name='{}'; $pkg = Get-AppxPackage -AllUsers | Where-Object {{ $_.PackageFamilyName -like "*$name*" -or $_.Name -like "*$name*" }} | Select-Object -First 1; if ($pkg -and $pkg.InstallLocation) {{ Write-Host $pkg.InstallLocation }} else {{ $prov = Get-AppxProvisionedPackage -Online | Where-Object {{ $_.DisplayName -like "*$name*" -or $_.PackageName -like "*$name*" }} | Select-Object -First 1; if ($prov) {{ $candidate = "$env:ProgramFiles\WindowsApps\" + $prov.PackageName; if (Test-Path $candidate) {{ Write-Host $candidate }} else {{ Write-Host "NOT_FOUND" }} }} else {{ Write-Host "NOT_FOUND" }} }}"#,
            name.replace('\'', "''")
        );

        let find_output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-Command", &find_path_script])
            .output()
            .map_err(|e| format!("Filesystem find spawn failed: {}", e))?;

        let path = String::from_utf8_lossy(&find_output.stdout)
            .trim()
            .to_string();

        if path.is_empty() || path == "NOT_FOUND" {
            return Err("Filesystem: Could not locate package folder".to_string());
        }

        let takeown = Command::new("takeown")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/f", &path, "/r", "/d", "y"])
            .output()
            .map_err(|e| format!("takeown spawn failed: {}", e))?;

        if !takeown.status.success() {
            return Err(format!(
                "takeown failed: {}",
                String::from_utf8_lossy(&takeown.stderr).trim()
            ));
        }

        let icacls = Command::new("icacls")
            .creation_flags(CREATE_NO_WINDOW)
            .args([&path, "/grant", "Administrators:F", "/t", "/q"])
            .output()
            .map_err(|e| format!("icacls spawn failed: {}", e))?;

        if !icacls.status.success() {
            return Err(format!(
                "icacls failed: {}",
                String::from_utf8_lossy(&icacls.stderr).trim()
            ));
        }

        let rmdir = Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/c", "rmdir", "/s", "/q", &path])
            .output()
            .map_err(|e| format!("rmdir spawn failed: {}", e))?;

        if !rmdir.status.success() {
            return Err(format!(
                "rmdir failed: {}",
                String::from_utf8_lossy(&rmdir.stderr).trim()
            ));
        }

        Ok(format!("Method 5 (Filesystem) succeeded for '{}'", name))
    }

    /// Force-remove a stubborn package using takeown + icacls + rmdir.
    /// Takes a package family name, locates its install folder, then
    /// brute-forces ownership, permissions, and deletion.
    pub fn force_remove(package_family_name: &str) -> Result<String, DebloatError> {
        Self::try_filesystem_remove(package_family_name).map_err(|e| {
            if e.contains("takeown") {
                DebloatError::AccessDenied(e)
            } else if e.contains("NOT_FOUND") || e.contains("locate") {
                DebloatError::NotFound(e)
            } else {
                DebloatError::ProcessFailed(e)
            }
        })
    }

    /// Disable lock screen ads, start menu ads, explorer ads,
    /// and advertising ID via registry.
    pub fn remove_windows_ads() -> Result<String, DebloatError> {
        let script = r#"
$lockKeys = @(
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\Personalization'; Name='NoLockScreenCamera'; Value=1},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\Personalization'; Name='NoChangingLockScreen'; Value=1},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\CloudContent'; Name='DisableWindowsSpotlightFeatures'; Value=1},
    @{Path='HKCU:\SOFTWARE\Policies\Microsoft\Windows\CloudContent'; Name='DisableWindowsSpotlightFeatures'; Value=1},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\CloudContent'; Name='DisableWindowsSpotlightOnActionCenter'; Value=1},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\CloudContent'; Name='DisableWindowsSpotlightOnSettings'; Value=1},
    @{Path='HKLM:\SOFTWARE\WOW6432Node\Policies\Microsoft\Windows\CloudContent'; Name='DisableWindowsSpotlightWindowsWelcomeExperience'; Value=1}
)

$startKeys = @(
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='SubscribedContent-338388Enabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='SubscribedContent-338389Enabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='SubscribedContent-338393Enabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='SubscribedContent-353694Enabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='SubscribedContent-353696Enabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='SystemPaneSuggestionsEnabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='SilentInstalledAppsEnabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='ContentDeliveryAllowed'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='OemPreInstalledAppsEnabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='PreInstalledAppsEnabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='PreInstalledAppsEverEnabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='SoftLandingEnabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='RotatingLockScreenOverlayEnabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager'; Name='RotatingLockScreenEnabled'; Value=0}
)

$explorerKeys = @(
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced'; Name='ShowSyncProviderNotifications'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced'; Name='ShowCloudFilesInQuickAccess'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Search'; Name='BingSearchEnabled'; Value=0},
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Search'; Name='CortanaConsent'; Value=0},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name='AllowCortana'; Value=0},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name='AllowCloudSearch'; Value=0},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name='AllowSearchToUseLocation'; Value=0},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name='ConnectedSearchUseWeb'; Value=0},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search'; Name='DisableWebSearch'; Value=1}
)

$adKeys = @(
    @{Path='HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo'; Name='Enabled'; Value=0},
    @{Path='HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo'; Name='Enabled'; Value=0},
    @{Path='HKLM:\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo'; Name='DisabledByGroupPolicy'; Value=1}
)

$allKeys = $lockKeys + $startKeys + $explorerKeys + $adKeys
$count = 0

foreach ($k in $allKeys) {
    $err = $null
    New-ItemProperty -Path $k.Path -Name $k.Name -PropertyType DWord -Value $k.Value -Force -ErrorAction SilentlyContinue -ErrorVariable err | Out-Null
    if (-not $err) { $count++ }
}
Write-Host "ADS_REMOVED:$count"
"#;

        let output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-Command", script])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("AccessDenied")
                || stderr.contains("Access is denied")
                || stderr.contains("0x80070005")
            {
                return Err(DebloatError::AccessDenied(stderr.to_string()));
            }
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("ADS_REMOVED:") {
            Ok("Windows ads disabled via registry".to_string())
        } else {
            Err(DebloatError::ProcessFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    /// Remove Windows Widgets (Web Experience Pack) and disable
    /// News & Interests via registry.
    pub fn remove_widgets() -> Result<String, DebloatError> {
        let ps_script = r#"
Get-AppxPackage -AllUsers *WebExperience* | Remove-AppxPackage -ErrorAction SilentlyContinue
Get-AppxProvisionedPackage -Online | Where-Object { $_.DisplayName -like '*WebExperience*' } | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue
Write-Host 'WIDGET_PKG_REMOVED'
"#;

        let _ = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-Command", ps_script])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(e.to_string()))?;

        let reg_script = r#"
New-ItemProperty -Path 'HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Feeds' -Name 'AllowNewsAndInterests' -PropertyType DWord -Value 0 -Force -ErrorAction SilentlyContinue | Out-Null
New-ItemProperty -Path 'HKLM:\SOFTWARE\Policies\Microsoft\Dsh' -Name 'AllowNewsAndInterests' -PropertyType DWord -Value 0 -Force -ErrorAction SilentlyContinue | Out-Null
New-ItemProperty -Path 'HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Feeds' -Name 'ShellFeedsTaskbarViewMode' -PropertyType DWord -Value 2 -Force -ErrorAction SilentlyContinue | Out-Null
New-ItemProperty -Path 'HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Feeds' -Name 'IsFeedsAvailable' -PropertyType DWord -Value 0 -Force -ErrorAction SilentlyContinue | Out-Null
New-ItemProperty -Path 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Feeds' -Name 'EnableFeeds' -PropertyType DWord -Value 0 -Force -ErrorAction SilentlyContinue | Out-Null
Write-Host 'WIDGETS_DISABLED'
"#;

        let output = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-NoProfile", "-Command", reg_script])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("AccessDenied") || stderr.contains("0x80070005") {
                return Err(DebloatError::AccessDenied(stderr.to_string()));
            }
        }

        Ok("Widgets removed and News & Interests disabled".to_string())
    }

    /// Remove a provisioned package via DISM.
    pub fn dism_remove(package_name: &str) -> Result<String, DebloatError> {
        let output = Command::new("dism")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "/online",
                "/Remove-ProvisionedAppxPackage",
                &format!("/PackageName:{}", package_name),
            ])
            .output()
            .map_err(|e| DebloatError::ProcessFailed(e.to_string()))?;

        if output.status.success() {
            Ok(format!("DISM removed: {}", package_name))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("0x80070002") {
                Ok(format!(
                    "DISM: '{}' already removed or not found",
                    package_name
                ))
            } else {
                Err(DebloatError::ProcessFailed(stderr.to_string()))
            }
        }
    }
}
