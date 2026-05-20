#![windows_subsystem = "windows"]

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;
use zb_application::tweak_engine::TweakEngine;
use zb_infrastructure::logging::init_logging;
use zb_infrastructure::persistence::audit_logger::SqliteAuditLogger;
use zb_infrastructure::persistence::sqlite_repo::{init_database, SqliteRepo};
use zb_infrastructure::services::ServiceController;
use zb_infrastructure::windows_api::metrics_collector::MetricsCollector;
use zb_infrastructure::windows_api::system_cleaner::SystemCleaner;
use zb_shared::constants::CREATE_NO_WINDOW;

mod commands;
mod state;

#[cfg(target_os = "windows")]
fn is_admin() -> bool {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    Command::new("net")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["session"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn relaunch_as_admin() {
    use std::env;
    use windows::core::PCWSTR;
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

    let exe = env::current_exe().unwrap_or_default();
    use std::os::windows::ffi::OsStrExt;
    let exe_wide: Vec<u16> = exe.as_os_str().encode_wide().chain(Some(0)).collect();
    let verb: Vec<u16> = "runas".encode_utf16().chain(Some(0)).collect();

    unsafe {
        ShellExecuteW(
            None,
            PCWSTR(verb.as_ptr()),
            PCWSTR(exe_wide.as_ptr()),
            None,
            None,
            SW_SHOWNORMAL,
        )
    };
}

/// Start background task that pushes metrics to the frontend every second.
/// Uses webview eval() to directly update DOM elements, bypassing all
/// event/callback indirection. This avoids WebView2 timer throttling.
fn start_metrics_emitter(app: tauri::AppHandle, metrics: Arc<MetricsCollector>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut tick_count: u64 = 0;
        loop {
            interval.tick().await;
            tick_count += 1;
            let m = metrics.current_json();
            if tick_count % 5 == 1 {
                tracing::info!("[metrics-emitter] tick#{} json={}", tick_count, &m);
            }

            // Build JS that directly updates DOM elements - no callbacks needed
            let js = format!(
                r#"
(function() {{
    try {{
        var d = {};
        var cpuEl = document.getElementById('home-cpu-value');
        var ramEl = document.getElementById('home-ram-value');
        var diskEl = document.getElementById('home-disk-value');
        var netEl = document.getElementById('home-network-value');
        var cpuBar = document.getElementById('home-cpu-bar');
        var ramBar = document.getElementById('home-ram-bar');
        var diskBar = document.getElementById('home-disk-bar');
        var netBar = document.getElementById('home-network-bar');
        var tsEl = document.getElementById('metrics-last-update');
        var ecEl = document.getElementById('event-counter');
        
        if (cpuEl) {{ cpuEl.textContent = Math.round(d.cpu_percent) + '%'; }}
        if (ramEl) {{ ramEl.textContent = Math.round(d.ram_percent) + '%'; }}
        if (diskEl) {{ diskEl.textContent = Math.round(d.disk_active_percent) + '%'; }}
        var netTotal = d.network_down_mbps + d.network_up_mbps;
        if (netEl) {{ netEl.textContent = netTotal < 1 ? (netTotal*1000).toFixed(0)+' Kbps' : netTotal.toFixed(1)+' Mbps'; }}
        if (cpuBar) {{ cpuBar.style.width = Math.min(d.cpu_percent,100)+'%'; }}
        if (ramBar) {{ ramBar.style.width = Math.min(d.ram_percent,100)+'%'; }}
        if (diskBar) {{ diskBar.style.width = Math.min(d.disk_active_percent,100)+'%'; }}
        if (netBar) {{ netBar.style.width = Math.min(netTotal*5,100)+'%'; }}
        if (tsEl) {{ tsEl.textContent = new Date().toLocaleTimeString(); }}
        if (ecEl) {{ ecEl.textContent = 'eval:' + (parseInt(ecEl.dataset.c||'0')+1); ecEl.dataset.c = (parseInt(ecEl.dataset.c||'0')+1).toString(); }}
    }} catch(e) {{ console.error('[metrics-eval]', e); }}
}})()
"#,
                m
            );

            if let Some(wv) = app.get_webview_window("main") {
                if let Err(e) = wv.eval(&js) {
                    tracing::warn!("[metrics-emitter] eval failed: {:?}", e);
                }
            } else {
                tracing::warn!("[metrics-emitter] no webview window 'main'");
            }
        }
    });
}

#[tokio::main]
async fn main() {
    init_logging();

    #[cfg(target_os = "windows")]
    if !is_admin() {
        tracing::warn!("Not running as administrator. Relaunching with elevation...");
        relaunch_as_admin();
        return;
    }

    let engine = init_database()
        .map(|db_conn| {
            let snapshot_service = SqliteRepo::from_connection(db_conn.clone());
            let audit_service = SqliteAuditLogger::from_connection(db_conn);
            let tweaks = commands::make_all_tweaks();
            Arc::new(TweakEngine::new(tweaks, snapshot_service, audit_service))
        })
        .ok();

    let app_state = state::AppState {
        engine: Mutex::new(engine),
        metrics: MetricsCollector::new(),
        cleaner: Arc::new(SystemCleaner::new()),
        services: Arc::new(ServiceController::new()),
        favorites: Mutex::new(state::FavoritesManager::new()),
    };
    tracing::info!("AppState initialized with MetricsCollector (background sampler started)");

    // Clone metrics for the background emitter before moving app_state into .manage()
    let metrics_for_emitter = app_state.metrics.clone();

    tauri::Builder::default()
        .setup(move |app| {
            // Start background metrics emitter
            start_metrics_emitter(app.handle().clone(), metrics_for_emitter);

            // Enable devtools only in debug builds
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.open_devtools();
                }
            }

            Ok(())
        })
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_app_info,
            commands::check_for_updates,
            commands::uninstall_app,
            commands::debug_log,
            commands::check_admin,
            commands::get_tweaks,
            commands::get_tweak_states,
            commands::apply_tweak,
            commands::revert_tweak,
            commands::apply_all_tweaks,
            commands::revert_all_tweaks,
            commands::get_services,
            commands::start_service,
            commands::stop_service,
            commands::disable_service,
            commands::get_cleaner_items,
            commands::clean_category,
            commands::clean_all,
            commands::get_bloatware,
            commands::check_bloatware_installed,
            commands::remove_bloatware,
            commands::get_software,
            commands::check_software_installed,
            commands::install_software,
            commands::get_metrics,
            commands::get_backups,
            commands::create_backup,
            commands::restore_backup,
            commands::delete_backup,
            commands::clear_backups,
            commands::toggle_game_mode,
            commands::get_audit_log,
            commands::clear_audit_log,
            commands::get_favorites,
            commands::toggle_favorite,
            commands::is_favorite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ZingerBoost");
}
