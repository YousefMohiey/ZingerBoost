use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use zb_application::tweak_engine::TweakEngine;
use zb_infrastructure::windows_api::metrics_collector::MetricsCollector;
use zb_infrastructure::windows_api::system_cleaner::SystemCleaner;
use zb_infrastructure::services::ServiceController;

pub struct AppState {
    pub engine: Mutex<Option<Arc<TweakEngine>>>,
    pub metrics: Arc<MetricsCollector>,
    pub cleaner: Arc<SystemCleaner>,
    pub services: Arc<ServiceController>,
    pub favorites: Mutex<FavoritesManager>,
}

/// Manages persistent favorites stored as JSON in %APPDATA%/ZingerBoost/favorites.json
pub struct FavoritesManager {
    favorites: HashSet<String>,
    file_path: PathBuf,
}

impl FavoritesManager {
    pub fn new() -> Self {
        let app_data = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
        let dir = PathBuf::from(&app_data).join("ZingerBoost");
        if !dir.exists() {
            let _ = fs::create_dir_all(&dir);
        }
        let file_path = dir.join("favorites.json");
        let favorites = Self::load_from_file(&file_path);
        Self { favorites, file_path }
    }

    fn load_from_file(path: &PathBuf) -> HashSet<String> {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(items) = serde_json::from_str::<Vec<String>>(&content) {
                return items.into_iter().collect();
            }
        }
        HashSet::new()
    }

    fn save_to_file(&self) {
        let items: Vec<String> = self.favorites.iter().cloned().collect();
        if let Ok(json) = serde_json::to_string_pretty(&items) {
            let _ = fs::write(&self.file_path, json);
        }
    }

    pub fn get_all(&self) -> Vec<String> {
        self.favorites.iter().cloned().collect()
    }

    pub fn toggle(&mut self, key: String) -> bool {
        let is_now_fav = if self.favorites.contains(&key) {
            self.favorites.remove(&key);
            false
        } else {
            self.favorites.insert(key);
            true
        };
        self.save_to_file();
        is_now_fav
    }

    pub fn is_favorite(&self, key: &str) -> bool {
        self.favorites.contains(key)
    }
}
