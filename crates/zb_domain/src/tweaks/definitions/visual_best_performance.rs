use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::constants::CREATE_NO_WINDOW;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

pub struct VisualBestPerformanceTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for VisualBestPerformanceTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualBestPerformanceTweak {
    pub fn new() -> Self {
        Self { provider: None }
    }
    pub fn with_provider(provider: Arc<dyn crate::registry::RegistryProvider>) -> Self {
        Self {
            provider: Some(provider),
        }
    }
}

#[cfg(windows)]
fn apply_visual_fx() -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    let script = r#"
$desk = "HKCU:\Control Panel\Desktop"
$adv = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"
$vis = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects"
Set-ItemProperty $vis -Name VisualFXSetting -Value 3 -Type DWord -Force
Set-ItemProperty $desk -Name UserPreferencesMask -Type Binary -Value ([byte[]](144,18,3,128,16,0,0,0)) -Force
Set-ItemProperty $desk -Name DragFullWindows -Value "1" -Force
Set-ItemProperty $desk -Name FontSmoothing -Value "2" -Force
Set-ItemProperty $adv -Name IconsOnly -Value 0 -Type DWord -Force
Set-ItemProperty $adv -Name ListviewShadow -Value 0 -Type DWord -Force
Set-ItemProperty $adv -Name DisablePreviewDesktop -Value 1 -Type DWord -Force
Set-ItemProperty $adv -Name DisableThumbnailCache -Value 1 -Type DWord -Force
Set-ItemProperty $adv -Name TaskbarAnimations -Value 0 -Type DWord -Force
Set-ItemProperty $adv -Name ListviewAlphaSelect -Value 0 -Type DWord -Force
Set-ItemProperty $desk -Name SmoothScroll -Value 0 -Type DWord -Force
Write-Host "REGISTRY_OK"
"#;

    let output = Command::new("powershell")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["-NoProfile", "-Command", script])
        .output()
        .map_err(|e| format!("PowerShell error: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.contains("REGISTRY_OK") && !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    thread::sleep(Duration::from_millis(300));

    let _kill_output = Command::new("taskkill")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["/F", "/IM", "explorer.exe"])
        .output()
        .map_err(|e| format!("taskkill error: {}", e))?;

    thread::sleep(Duration::from_millis(1500));

    let start_output = Command::new("powershell")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["-NoProfile", "-Command", "Start-Process explorer"])
        .output()
        .map_err(|e| format!("explorer start error: {}", e))?;

    if !start_output.status.success() {
        return Err(format!(
            "Failed to restart explorer: {}",
            String::from_utf8_lossy(&start_output.stderr)
        ));
    }

    Ok(())
}

#[cfg(windows)]
fn revert_visual_fx(previous_value: u32) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    let script = format!(
        r#"
$vis = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects"
Set-ItemProperty $vis -Name VisualFXSetting -Value {} -Type DWord -Force
Write-Host "REGISTRY_OK"
"#,
        previous_value
    );

    let output = Command::new("powershell")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["-NoProfile", "-Command", &script])
        .output()
        .map_err(|e| format!("PowerShell error: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.contains("REGISTRY_OK") && !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    thread::sleep(Duration::from_millis(300));

    let _kill_output = Command::new("taskkill")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["/F", "/IM", "explorer.exe"])
        .output()
        .map_err(|e| format!("taskkill error: {}", e))?;

    thread::sleep(Duration::from_millis(1500));

    let start_output = Command::new("powershell")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["-NoProfile", "-Command", "Start-Process explorer"])
        .output()
        .map_err(|e| format!("explorer start error: {}", e))?;

    if !start_output.status.success() {
        return Err(format!(
            "Failed to restart explorer: {}",
            String::from_utf8_lossy(&start_output.stderr)
        ));
    }

    Ok(())
}

#[cfg(not(windows))]
fn apply_visual_fx() -> Result<(), String> {
    Err("Not on Windows".into())
}

#[async_trait]
impl Tweak for VisualBestPerformanceTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_best_performance".into(),
            name: "Best Performance Visual Effects".into(),
            description: "Disables all visual animations for max responsiveness. Keeps thumbnails, window dragging, and font smoothing ON.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects"),
                RegPath::hkcu(r"Control Panel\Desktop"),
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
            ],
            source_url: Some("https://winutil.christitus.com/dev/tweaks/z--advanced-tweaks---caution/display".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let vis =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects");
            let desk = RegPath::hkcu(r"Control Panel\Desktop");
            let _adv =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");

            let fx = provider.read(&vis, "VisualFXSetting").await.ok();
            let drag = provider.read(&desk, "DragFullWindows").await.ok();
            let font = provider.read(&desk, "FontSmoothing").await.ok();

            let fx_ok = matches!(fx, Some(RegValue::Dword(3)));
            let drag_ok = matches!(drag, Some(RegValue::Sz(ref v)) if v == "1");
            let font_ok = matches!(font, Some(RegValue::Sz(ref v)) if v == "2");

            Ok(fx_ok && drag_ok && font_ok)
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let vis =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects");
            let fx = provider
                .read(&vis, "VisualFXSetting")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path: vis,
                name: "VisualFXSetting".into(),
                previous: fx,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(
                    r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects",
                ),
                name: "VisualFXSetting".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        apply_visual_fx().map_err(|e| TweakError::Unknown(e))?;

        Ok(TweakResult {
            reboot_required: false,
            message: "Visual effects applied. Explorer restarted to apply changes immediately."
                .into(),
        })
    }

    async fn revert(&self, snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        let previous_value = if let SnapshotData::Registry {
            name: _, previous, ..
        } = snapshot
        {
            match previous {
                RegValue::Dword(v) => *v,
                _ => 1,
            }
        } else {
            1
        };

        revert_visual_fx(previous_value).map_err(|e| TweakError::Unknown(e))?;

        Ok(TweakResult {
            reboot_required: false,
            message: "Visual effects restored. Explorer restarted to apply changes.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Sets Windows Visual Effects to Best Performance with 3 items kept ON: thumbnails, window dragging, font smoothing. Restarts explorer to apply immediately.".into(),
            why_it_helps: "Maximum UI responsiveness without sacrificing thumbnails, window dragging, or font clarity.".into(),
            potential_risks: Some("Windows will look plain — no animations, shadows, or transparency. Explorer restarts briefly.".into()),
            how_to_revert: "Restores the previous VisualFXSetting. Explorer restarts to apply.".into(),
        }
    }
}
