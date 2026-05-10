pub mod views;

use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, vertical_rule, Column,
    Container, Row,
};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task, Theme};
use std::sync::Arc;
use std::time::Duration;
use zb_infrastructure::logging::init_logging;
use zb_infrastructure::services::{ServiceController, WindowsService};
use zb_infrastructure::windows_api::system_cleaner::SystemCleaner;
use zb_shared::types::SystemMetrics;

pub fn run() -> iced::Result {
    init_logging();
    iced::application("ZingerBoost", App::update, App::view)
        .theme(|state| state.theme())
        .subscription(App::subscription)
        .run_with(App::new)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Tweaks,
    Services,
    Cleaner,
    Snapshots,
    Debloat,
    Software,
    Settings,
}

impl Tab {
    const ALL: [Tab; 8] = [
        Tab::Dashboard,
        Tab::Tweaks,
        Tab::Services,
        Tab::Cleaner,
        Tab::Snapshots,
        Tab::Debloat,
        Tab::Software,
        Tab::Settings,
    ];
    fn label(&self) -> &str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Tweaks => "Tweaks",
            Tab::Services => "Services",
            Tab::Cleaner => "Cleaner",
            Tab::Snapshots => "Snapshots",
            Tab::Debloat => "Debloat",
            Tab::Software => "Software",
            Tab::Settings => "Settings",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    RefreshMetrics,
    MetricsUpdated(SystemMetrics),
    TweaksLoaded(Vec<zb_shared::types::TweakMetadata>),
    TweakApply(usize, String),
    TweakRevert(usize, String),
    TweakResult(usize, String, bool),
    ServiceStop(usize, String),
    ServiceDisable(usize, String),
    ServiceActionResult(String),
    CleanerRun(String),
    CleanerResult(String, String),
    LoadSnapshots,
    SnapshotsLoaded(Vec<zb_domain::snapshots::SystemSnapshot>),
    SnapshotRestore(String),
    SnapshotRestored(String),
    DebloatRemove(String),
    DebloatResult(String, String),
    SoftwareInstall(String),
    SoftwareResult(String, String),
    ThemeToggled,
    AdminCheck(bool),
    StatusMessage(String),
    ClearStatus,
}

// ===== DATA =====

struct TweakRow {
    meta: zb_shared::types::TweakMetadata,
    applied: bool,
}
struct SvcRow {
    svc: WindowsService,
    idx: usize,
}

fn make_all_tweaks() -> Vec<Arc<dyn zb_domain::tweaks::Tweak>> {
    let rp = zb_infrastructure::registry::WinRegistryProvider::new();
    vec![
        Arc::new(zb_domain::tweaks::definitions::DisableGameDvrTweak::with_provider(rp.clone())),
        Arc::new(
            zb_domain::tweaks::definitions::DisableTransparencyTweak::with_provider(rp.clone()),
        ),
        Arc::new(zb_domain::tweaks::definitions::DisableAnimationsTweak::with_provider(rp.clone())),
        Arc::new(
            zb_domain::tweaks::definitions::ShowFileExtensionsTweak::with_provider(rp.clone()),
        ),
        Arc::new(zb_domain::tweaks::definitions::DisableStickyKeysTweak::with_provider(rp.clone())),
        Arc::new(
            zb_domain::tweaks::definitions::DisableStartupDelayTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableBackgroundAppsTweak::with_provider(rp.clone()),
        ),
        Arc::new(zb_domain::tweaks::definitions::DisableTelemetryTweak::with_provider(rp.clone())),
        Arc::new(zb_domain::tweaks::definitions::DisableMenuDelayTweak::with_provider(rp.clone())),
        Arc::new(
            zb_domain::tweaks::definitions::DisableCursorShadowTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableFontSmoothingTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableTaskbarAnimationsTweak::with_provider(
                rp.clone(),
            ),
        ),
        Arc::new(zb_domain::tweaks::definitions::DisableAeroShakeTweak::with_provider(rp.clone())),
        Arc::new(zb_domain::tweaks::definitions::DisableAeroSnapTweak::with_provider(rp.clone())),
        Arc::new(zb_domain::tweaks::definitions::DisablePeekTweak::with_provider(rp.clone())),
        Arc::new(
            zb_domain::tweaks::definitions::DisableSmoothScrollTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableComboAnimationTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableTaskbarBadgesTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableAllVisualEffectsTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableDropShadowsTweak::with_provider(rp.clone()),
        ),
        Arc::new(zb_domain::tweaks::definitions::DisableThumbnailsTweak::with_provider(rp.clone())),
        Arc::new(zb_domain::tweaks::definitions::DisableMinMaxAnimTweak::with_provider(rp.clone())),
        Arc::new(
            zb_domain::tweaks::definitions::DisableLockScreenAdsTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableStartSuggestionsTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableExplorerAdsTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableAdvertisingIdTweak::with_provider(rp.clone()),
        ),
        Arc::new(zb_domain::tweaks::definitions::DisableMeetNowTweak::with_provider(rp.clone())),
        Arc::new(zb_domain::tweaks::definitions::DisableHibernationTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::SetHighPerformanceTweak::new()),
    ]
}

async fn do_refresh_metrics() -> SystemMetrics {
    let c = zb_infrastructure::windows_api::metrics_collector::MetricsCollector::new();
    c.current().await
}

async fn do_list_tweaks() -> Vec<zb_shared::types::TweakMetadata> {
    make_all_tweaks().iter().map(|t| t.metadata()).collect()
}

async fn do_apply_tweak(id: String) -> (String, String, bool) {
    let tweaks = make_all_tweaks();
    if let Some(t) = tweaks.iter().find(|tw| tw.metadata().id == id) {
        let prev = match t.capture_state().await {
            Ok(s) => s,
            Err(e) => return (id, e.to_string(), false),
        };
        match t.apply().await {
            Ok(r) => {
                // Save snapshot to database so user can view/restore later
                if let Ok(db_conn) = zb_infrastructure::persistence::init_database() {
                    let repo = zb_infrastructure::persistence::SqliteRepo::from_connection(db_conn);
                    let _ = repo.save_applied(&id, prev).await;
                }
                (id.clone(), r.message, true)
            }
            Err(e) => (id, e.to_string(), false),
        }
    } else {
        (id, "Tweak not found".into(), false)
    }
}

async fn do_revert_tweak(id: String) -> (String, String, bool) {
    let tweaks = make_all_tweaks();
    if let Some(t) = tweaks.iter().find(|tw| tw.metadata().id == id) {
        let snap = match t.capture_state().await {
            Ok(s) => s,
            Err(e) => return (id, e.to_string(), false),
        };
        match t.revert(&snap).await {
            Ok(r) => (id, r.message, true),
            Err(e) => (id, e.to_string(), false),
        }
    } else {
        (id, "Tweak not found".into(), false)
    }
}

async fn do_clean_category(id: String) -> (String, String) {
    let c = SystemCleaner::new();
    let result = c.clean_category(&id);
    (
        id,
        format!("{} — {} freed", result.category, result.bytes_freed),
    )
}

async fn do_remove_bloat(name: String) -> (String, String) {
    match zb_infrastructure::windows_api::debloat_engine::DebloatEngine::remove_appx_package(&name)
    {
        Ok(msg) => (name, msg),
        Err(e) => (name, e.to_string()),
    }
}

async fn do_install_winget(name: String) -> (String, String) {
    let w = zb_infrastructure::windows_api::winget::WingetInstaller::new();
    match w.install(&name) {
        Ok(msg) => (name, msg),
        Err(e) => (name, e),
    }
}

async fn do_load_snapshots() -> Vec<zb_domain::snapshots::SystemSnapshot> {
    match zb_infrastructure::persistence::init_database() {
        Ok(db_conn) => {
            let repo = zb_infrastructure::persistence::SqliteRepo::from_connection(db_conn);
            repo.list_snapshots().await.unwrap_or_default()
        }
        Err(_) => vec![],
    }
}

async fn do_restore_snapshot(snap_id: String) -> String {
    let db_conn = match zb_infrastructure::persistence::init_database() {
        Ok(c) => c,
        Err(e) => return format!("DB error: {}", e),
    };
    let repo = zb_infrastructure::persistence::SqliteRepo::from_connection(db_conn);
    // Load snapshot records and revert each tweak
    let snaps = match repo.list_snapshots().await {
        Ok(s) => s,
        Err(e) => return format!("Load error: {}", e),
    };
    let target = match snaps.iter().find(|s| s.id.to_string() == snap_id) {
        Some(s) => s.clone(),
        None => return "Snapshot not found".into(),
    };
    let tweaks = make_all_tweaks();
    let mut restored = 0;
    for record in &target.tweak_records {
        if let Some(t) = tweaks.iter().find(|tw| tw.metadata().id == record.tweak_id) {
            if t.revert(&record.snapshot_data).await.is_ok() {
                restored += 1;
            }
        }
    }
    format!("Restored {} tweaks from snapshot", restored)
}

async fn do_check_admin() -> bool {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::Security::IsUserAnAdmin;
        IsUserAnAdmin().as_bool()
    }
    #[cfg(not(target_os = "windows"))]
    true
}

fn do_create_restore_point() -> String {
    #[cfg(target_os = "windows")]
    {
        let desc: Vec<u16> = "ZingerBoost Pre-Tweak Snapshot\0".encode_utf16().collect();
        use windows::Win32::System::Restore::{
            SRSetRestorePointW, RESTOREPOINTINFO, RESTOREPOINTINFOW,
        };
        let mut info = RESTOREPOINTINFOW {
            dwEventType: 100, // APPLICATION_INSTALL
            dwRestorePtType: 0,
            llSequenceNumber: 0,
            szDescription: [0; 256],
        };
        for (i, &ch) in desc.iter().take(255).enumerate() {
            info.szDescription[i] = ch;
        }
        let mut stat = std::mem::zeroed();
        unsafe {
            if SRSetRestorePointW(&info, &mut stat) != 0 {
                return "System Restore Point created".into();
            }
        }
    }
    "Restore point unavailable on this platform".into()
}

pub struct App {
    current_tab: Tab,
    dark_mode: bool,
    metrics: SystemMetrics,
    tweaks: Vec<TweakRow>,
    services: Vec<SvcRow>,
    snapshots: Vec<zb_domain::snapshots::SystemSnapshot>,
    snapshots_loaded: bool,
    software_catalog: Vec<zb_shared::software::SoftwarePackage>,
    bloatware: Vec<zb_shared::software::SoftwarePackage>,
    protected_apps: Vec<String>,
    status_message: Option<String>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let services = ServiceController::new()
            .query_services()
            .into_iter()
            .enumerate()
            .map(|(i, s)| SvcRow { svc: s, idx: i })
            .collect();
        (
            Self {
                current_tab: Tab::Dashboard,
                dark_mode: true,
                metrics: SystemMetrics {
                    cpu_percent: 0.0,
                    ram_percent: 0.0,
                    ram_used_mb: 0,
                    ram_total_mb: 0,
                    disk_active_percent: 0.0,
                    network_down_mbps: 0.0,
                    network_up_mbps: 0.0,
                },
                tweaks: vec![],
                services,
                snapshots: vec![],
                snapshots_loaded: false,
                software_catalog: zb_shared::software::get_software_catalog(),
                bloatware: zb_shared::software::get_bloatware_catalog(),
                protected_apps: zb_shared::software::get_protected_apps()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                status_message: None,
            },
            Task::batch(vec![
                Task::perform(do_refresh_metrics(), Message::MetricsUpdated),
                Task::perform(do_list_tweaks(), Message::TweaksLoaded),
                Task::perform(do_check_admin(), Message::AdminCheck),
            ]),
        )
    }

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_secs(2)).map(|_| Message::RefreshMetrics)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => {
                let task = if tab == Tab::Snapshots && !self.snapshots_loaded {
                    self.snapshots_loaded = true;
                    Task::perform(do_load_snapshots(), Message::SnapshotsLoaded)
                } else {
                    Task::none()
                };
                self.current_tab = tab;
                task
            }
            Message::RefreshMetrics => Task::perform(do_refresh_metrics(), Message::MetricsUpdated),
            Message::MetricsUpdated(m) => {
                self.metrics = m;
                Task::none()
            }
            Message::TweaksLoaded(meta) => {
                self.tweaks = meta
                    .into_iter()
                    .map(|m| TweakRow {
                        meta: m,
                        applied: false,
                    })
                    .collect();
                Task::none()
            }
            Message::TweakApply(idx, ref id) => {
                let id2 = id.clone();
                Task::perform(do_apply_tweak(id2), move |(_, msg, ok)| {
                    Message::TweakResult(idx, msg, ok)
                })
                .chain(Task::done(Message::StatusMessage("Applying...".into())))
            }
            Message::TweakRevert(idx, ref id) => {
                let id2 = id.clone();
                Task::perform(do_revert_tweak(id2), move |(_, msg, ok)| {
                    Message::TweakResult(idx, msg, ok)
                })
            }
            Message::TweakResult(idx, msg, ok) => {
                if let Some(t) = self.tweaks.get_mut(idx) {
                    t.applied = ok;
                }
                Task::done(Message::StatusMessage(msg))
            }
            Message::ServiceStop(_, name) => {
                match ServiceController::new().stop_service(&name) {
                    Ok(msg) => self.status_message = Some(msg),
                    Err(e) => self.status_message = Some(e),
                }
                Task::none()
            }
            Message::ServiceDisable(_, name) => {
                match ServiceController::new().disable_service(&name) {
                    Ok(msg) => self.status_message = Some(msg),
                    Err(e) => self.status_message = Some(e),
                }
                Task::none()
            }
            Message::ServiceActionResult(msg) => {
                self.status_message = Some(msg);
                Task::none()
            }
            Message::CleanerRun(ref id) => {
                let id2 = id.clone();
                Task::perform(do_clean_category(id2), |(_, msg)| {
                    Message::CleanerResult("clean".into(), msg)
                })
            }
            Message::CleanerResult(_, msg) => {
                self.status_message = Some(msg);
                Task::none()
            }
            Message::LoadSnapshots => Task::perform(do_load_snapshots(), Message::SnapshotsLoaded),
            Message::SnapshotsLoaded(snaps) => {
                self.snapshots = snaps;
                Task::none()
            }
            Message::SnapshotRestore(ref id) => {
                let id2 = id.clone();
                Task::perform(do_restore_snapshot(id2), |msg| {
                    Message::SnapshotRestored(msg)
                })
            }
            Message::SnapshotRestored(msg) => {
                self.status_message = Some(msg);
                Task::none()
            }
            Message::AdminCheck(is_admin) => {
                if !is_admin {
                    self.status_message =
                        Some("Not running as Administrator — some features limited".into());
                }
                Task::none()
            }
            Message::DebloatRemove(ref name) => {
                let n = name.clone();
                Task::perform(do_remove_bloat(n), |(_, msg)| {
                    Message::DebloatResult("debloat".into(), msg)
                })
            }
            Message::DebloatResult(_, msg) => {
                self.status_message = Some(msg);
                Task::none()
            }
            Message::SoftwareInstall(ref winget_id) => {
                let id = winget_id.clone();
                Task::perform(do_install_winget(id), |(_, msg)| {
                    Message::SoftwareResult("install".into(), msg)
                })
            }
            Message::SoftwareResult(_, msg) => {
                self.status_message = Some(msg);
                Task::none()
            }
            Message::ThemeToggled => {
                self.dark_mode = !self.dark_mode;
                Task::none()
            }
            Message::StatusMessage(msg) => {
                self.status_message = Some(msg);
                Task::none()
            }
            Message::ClearStatus => {
                self.status_message = None;
                Task::none()
            }
        }
    }
}

// ===== STYLES =====

fn brand_color() -> Color {
    Color::from_rgb(0.055, 0.647, 0.914)
}
fn safe_color() -> Color {
    Color::from_rgb(0.067, 0.725, 0.506)
}
fn warn_color() -> Color {
    Color::from_rgb(0.961, 0.369, 0.043)
}
fn danger_color() -> Color {
    Color::from_rgb(0.937, 0.267, 0.267)
}
fn card_bg2() -> Color {
    Color::from_rgb(0.13, 0.13, 0.13)
}

fn card_style() -> impl Fn(&Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(card_bg2())),
        border: Border::rounded(10),
        ..Default::default()
    }
}

fn small_btn(label: &str) -> Button<Message> {
    button(text(label).size(12)).padding(Padding::from([4, 12]))
}
fn primary_btn(label: &str) -> Button<Message> {
    small_btn(label).style(|theme, status| button::primary(theme, status))
}
fn danger_btn(label: &str) -> Button<Message> {
    small_btn(label).style(|theme, status| button::danger(theme, status))
}

// ===== SIDEBAR =====

fn sidebar_view(current: Tab) -> Column<Message> {
    let mut col = column![].spacing(4).padding(8);
    for tab in &Tab::ALL {
        let is_active = *tab == current;
        let mut btn = button(
            text(tab.label())
                .size(13)
                .align_x(Alignment::Center)
                .width(160),
        )
        .padding(Padding::from([8, 12]));
        if is_active {
            btn = btn.style(|_, _| button::Style {
                background: Some(Background::Color(brand_color())),
                text_color: Color::WHITE,
                border: Border::rounded(8),
                ..Default::default()
            });
        } else {
            btn = btn.style(|t, s| button::text(t, s));
        }
        col = col.push(btn.on_press(Message::TabSelected(*tab)));
    }
    col
}

// ===== DASHBOARD =====

fn dashboard_view(metrics: &SystemMetrics) -> Column<Message> {
    let mc = |label: &str, value: String, sub: String| -> Container<Message> {
        container(
            column![
                text(label).size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
                text(value).size(24),
                container(text(sub).size(11).color(Color::from_rgb(0.4, 0.4, 0.4)))
            ]
            .spacing(4),
        )
        .padding(16)
        .width(Length::Fill)
        .style(card_style())
    };
    column![
        row![
            mc(
                "CPU Usage",
                format!("{:.1}%", metrics.cpu_percent),
                String::new()
            )
            .width(Length::Fill),
            mc(
                "RAM Usage",
                format!("{:.1}%", metrics.ram_percent),
                format!("{} / {} MB", metrics.ram_used_mb, metrics.ram_total_mb)
            )
            .width(Length::Fill)
        ]
        .spacing(12),
        container(vertical_rule(0)).height(12),
        row![
            mc(
                "Disk Active",
                format!("{:.1}%", metrics.disk_active_percent),
                String::new()
            )
            .width(Length::Fill),
            mc(
                "Network",
                format!("↓{:.1} Mbps", metrics.network_down_mbps),
                format!("↑{:.1}", metrics.network_up_mbps)
            )
            .width(Length::Fill)
        ]
        .spacing(12),
        container(vertical_rule(0)).height(16),
        container(
            column![
                text("Recommended Actions").size(14),
                row![
                    text("Disable Transparency Effects")
                        .size(12)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    horizontal_space(),
                    small_btn("Apply")
                        .on_press(Message::TweakApply(0, "visual_disable_transparency".into()))
                ],
                row![
                    text("Disable Game DVR")
                        .size(12)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    horizontal_space(),
                    small_btn("Apply")
                        .on_press(Message::TweakApply(0, "gaming_disable_dvr".into()))
                ],
                row![
                    text("Show File Extensions")
                        .size(12)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    horizontal_space(),
                    small_btn("Apply")
                        .on_press(Message::TweakApply(0, "visual_show_extensions".into()))
                ]
            ]
            .spacing(6)
        )
        .padding(16)
        .width(Length::Fill)
        .style(card_style()),
    ]
    .spacing(8)
}

// ===== TWEAKS =====

fn tweaks_view(tweaks: &[TweakRow]) -> Column<Message> {
    let mut col = column![text("Tweaks").size(20)].spacing(8);
    for (i, t) in tweaks.iter().enumerate() {
        let rc = match t.meta.risk {
            zb_shared::types::RiskLevel::Safe => safe_color(),
            zb_shared::types::RiskLevel::Moderate => warn_color(),
            zb_shared::types::RiskLevel::Advanced => danger_color(),
        };
        let card = column![
            row![
                text(&t.meta.name).size(14),
                horizontal_space(),
                container(text(format!("{:?}", t.meta.risk)).size(11))
                    .padding(4)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(rc)),
                        text_color: Some(Color::WHITE),
                        border: Border::rounded(20),
                        ..Default::default()
                    })
            ],
            text(&t.meta.description)
                .size(12)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            if t.applied {
                primary_btn("Revert").on_press(Message::TweakRevert(i, t.meta.id.clone()))
            } else {
                primary_btn("Apply").on_press(Message::TweakApply(i, t.meta.id.clone()))
            },
        ]
        .spacing(6);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col
}

// ===== SERVICES =====

fn services_view(svcs: &[SvcRow]) -> Column<Message> {
    let mut col = column![text("Services").size(20)].spacing(8);
    for r in svcs {
        let card = column![
            row![
                text(&r.svc.display_name).size(14),
                horizontal_space(),
                container(text(&r.svc.status).size(11))
                    .padding(4)
                    .style(move |_| container::Style {
                        background: Some(Background::Color(if r.svc.status == "Running" {
                            safe_color()
                        } else {
                            Color::from_rgb(0.4, 0.4, 0.4)
                        })),
                        text_color: Some(Color::WHITE),
                        border: Border::rounded(20),
                        ..Default::default()
                    })
            ],
            text(&r.svc.name)
                .size(12)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            text(&r.svc.description)
                .size(11)
                .color(Color::from_rgb(0.4, 0.4, 0.4)),
            row![
                danger_btn("Stop").on_press(Message::ServiceStop(r.idx, r.svc.name.clone())),
                danger_btn("Disable").on_press(Message::ServiceDisable(r.idx, r.svc.name.clone()))
            ]
            .spacing(8),
        ]
        .spacing(6);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col
}

// ===== CLEANER =====

fn cleaner_view() -> Column<Message> {
    let cat = SystemCleaner::new();
    let cats = cat.scan_categories();
    let mut col = column![text("Cleaner").size(20)].spacing(12);
    for c in &cats {
        let rc = if c.risk == "safe" {
            safe_color()
        } else {
            warn_color()
        };
        let card = row![
            text(&c.name).size(14).width(Length::Fill),
            container(text(&c.risk).size(11))
                .padding(4)
                .style(move |_| container::Style {
                    background: Some(Background::Color(rc)),
                    text_color: Some(Color::WHITE),
                    border: Border::rounded(20),
                    ..Default::default()
                }),
            text(format!("{:.1} MB", c.size_bytes as f64 / 1_048_576.0)).size(12),
            if c.size_bytes > 0 {
                primary_btn("Clean").on_press(Message::CleanerRun(c.id.clone()))
            } else {
                button(text("Clean").size(12)).style(button::secondary)
            }
        ]
        .spacing(8)
        .align_y(Alignment::Center);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col
}

// ===== SNAPSHOTS =====

fn snapshots_view(snapshots: &[zb_domain::snapshots::SystemSnapshot]) -> Column<Message> {
    let mut col = column![text("Snapshots").size(20).width(Length::Fill)].spacing(8);
    if snapshots.is_empty() {
        col = col.push(
            container(
                column![
                    text("No snapshots yet").size(16),
                    text("Snapshots are created automatically when you apply tweaks.")
                        .size(12)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                ]
                .spacing(8)
                .align_x(Alignment::Center),
            )
            .padding(40)
            .width(Length::Fill)
            .style(card_style()),
        );
    } else {
        for snap in snapshots {
            let date = snap.created_at.format("%b %d · %H:%M").to_string();
            let count = snap.tweak_records.len();
            let desc = &snap.description;
            let card = row![
                column![
                    text(desc.clone()).size(14),
                    text(date).size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
                ]
                .spacing(4)
                .width(Length::Fill),
                text(format!(
                    "{} tweak{}",
                    count,
                    if count != 1 { "s" } else { "" }
                ))
                .size(12),
                small_btn("Restore").on_press(Message::SnapshotRestore(snap.id.to_string())),
            ]
            .spacing(12)
            .align_y(Alignment::Center);
            col = col.push(container(card).padding(12).style(card_style()));
        }
    }
    col
}

// ===== DEBLOAT =====

fn debloat_view(
    bloatware: &[zb_shared::software::SoftwarePackage],
    protected: &[String],
) -> Column<Message> {
    let mut col = column![
        text("Debloat").size(20),
        container(
            text("These can be reinstalled from Microsoft Store")
                .size(12)
                .color(warn_color())
        )
        .padding(8)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.96, 0.37, 0.04, 0.1))),
            border: Border::rounded(8),
            ..Default::default()
        }),
    ]
    .spacing(8);
    for b in bloatware {
        let card = row![
            text(&b.name).size(14).width(Length::Fill),
            text(&b.description)
                .size(11)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            danger_btn("Remove").on_press(Message::DebloatRemove(b.name.clone())),
        ]
        .spacing(12)
        .align_y(Alignment::Center);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col = col.push(container(vertical_rule(0)).height(16));
    col = col.push(
        container(
            column![
                text("Protected Apps").size(14).color(safe_color()),
                text(protected.join(", "))
                    .size(11)
                    .color(Color::from_rgb(0.4, 0.4, 0.4))
            ]
            .spacing(4),
        )
        .padding(12)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.07, 0.73, 0.51, 0.1))),
            border: Border::rounded(8),
            ..Default::default()
        }),
    );
    col
}

// ===== SOFTWARE =====

fn software_view(catalog: &[zb_shared::software::SoftwarePackage]) -> Column<Message> {
    let mut col = column![text("Software").size(20)].spacing(8);
    for pkg in catalog {
        let card = row![
            text(&pkg.name).size(14).width(Length::Fill),
            text(format!("{:?}", pkg.category))
                .size(11)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            primary_btn("Install").on_press(Message::SoftwareInstall(pkg.winget_id.clone())),
        ]
        .spacing(12)
        .align_y(Alignment::Center);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col
}

// ===== SETTINGS =====

fn settings_view(dark_mode: bool) -> Column<Message> {
    column![
        text("Settings").size(20),
        container(
            column![
                text("Theme").size(14),
                text(if dark_mode { "Dark Mode" } else { "Light Mode" })
                    .size(12)
                    .color(Color::from_rgb(0.5, 0.5, 0.5)),
                primary_btn("Toggle Dark/Light").on_press(Message::ThemeToggled)
            ]
            .spacing(8)
        )
        .padding(16)
        .style(card_style()),
        container(
            column![
                text("Version").size(14),
                text("v0.4.0 — Iced Edition")
                    .size(12)
                    .color(Color::from_rgb(0.5, 0.5, 0.5))
            ]
            .spacing(4)
        )
        .padding(16)
        .style(card_style()),
        container(
            column![
                text("About").size(14),
                text(
                    "29 real tweaks · 19 real services · 9 cleaner categories · 34 debloat targets"
                )
                .size(12)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
                text("Author: YousefMohiey | MIT License")
                    .size(11)
                    .color(Color::from_rgb(0.4, 0.4, 0.4))
            ]
            .spacing(4)
        )
        .padding(16)
        .style(card_style()),
    ]
    .spacing(12)
}

// ===== MAIN VIEW =====

impl App {
    fn view(&self) -> Element<Message> {
        let sidebar = container(sidebar_view(self.current_tab))
            .width(Length::Fixed(180))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.07, 0.07, 0.07))),
                ..Default::default()
            });

        let content: Column<Message> = match self.current_tab {
            Tab::Dashboard => dashboard_view(&self.metrics),
            Tab::Tweaks => tweaks_view(&self.tweaks),
            Tab::Services => services_view(&self.services),
            Tab::Cleaner => cleaner_view(),
            Tab::Snapshots => snapshots_view(&self.snapshots),
            Tab::Debloat => debloat_view(&self.bloatware, &self.protected_apps),
            Tab::Software => software_view(&self.software_catalog),
            Tab::Settings => settings_view(self.dark_mode),
        };

        let body = column![
            row![sidebar, container(scrollable(content)).padding(16)].spacing(0),
            if let Some(ref msg) = self.status_message {
                container(text(msg).size(12).color(Color::from_rgb(0.8, 0.8, 0.8)))
                    .padding(8)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgb(0.05, 0.05, 0.05))),
                        ..Default::default()
                    })
            } else {
                container(text("")).height(0)
            }
        ];

        container(body).into()
    }
}
