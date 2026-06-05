use zb_domain::tweaks::{Tweak, TweakResult, TweakError};
use zb_infrastructure::registry::WinRegistryProvider;
use async_trait::async_trait;

pub struct ShowFileExtensionsTweak {
    registry: WinRegistryProvider,
}

impl ShowFileExtensionsTweak {
    pub fn new() -> Self {
        Self {
            registry: WinRegistryProvider::new(),
        }
    }
}

#[async_trait]
impl Tweak for ShowFileExtensionsTweak {
    fn id(&self) -> &str {
        "show_file_extensions"
    }

    fn name(&self) -> &str {
        "Show File Extensions"
    }

    fn description(&self) -> &str {
        "Display file extensions in File Explorer for better visibility"
    }

    fn category(&self) -> &str {
        "visual"
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        self.registry
            .set_dword(
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
                "HideFileExt",
                0,
            )
            .await
            .map_err(TweakError::Registry)?;
        Ok(TweakResult::Applied)
    }

    async fn revert(&self) -> Result<TweakResult, TweakError> {
        self.registry
            .set_dword(
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
                "HideFileExt",
                1,
            )
            .await
            .map_err(TweakError::Registry)?;
        Ok(TweakResult::Reverted)
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        let value = self.registry
            .get_dword(
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
                "HideFileExt",
            )
            .await;
        Ok(value == Some(0))
    }
}
