use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task, Theme};
use std::sync::Arc;
use std::time::Duration;
use zb_infrastructure::logging::init_logging;
use zb_infrastructure::services::ServiceController;
use zb_shared::types::SystemMetrics;

pub fn run() -> iced::Result {
    init_logging();
    iced::application("ZingerBoost", App::update, App::view)
        .theme(|state| state.theme())
        .subscription(App::subscription)
        .run_with(App::new)
}

type T = Task<Message>;

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
    Tick,
    MetricsUpdated(SystemMetrics),
    TweakApply(usize, String),
    TweakResult(usize, String),
    SvcStop(usize, String),
    SvcDisable(usize, String),
    SvcResult(usize, String),
    Clean(String),
    CleanResult(String),
    DebloatRemove(String),
    DebloatResult(String),
    SoftwareInstall(String),
    SoftwareResult(String),
}

pub struct App {
    current_tab: Tab,
    metrics: SystemMetrics,
    tweaks: Vec<(String, String, String)>,
    services: Vec<(String, String, String)>,
    cleaner_items: Vec<(String, String, String, f64)>,
    bloatware: Vec<(String, String)>,
    software: Vec<(String, String, String)>,
    status: Option<String>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let tweaks = make_all_tweaks()
            .iter()
            .map(|t| {
                let m = t.metadata();
                (
                    m.name.clone(),
                    m.description.clone(),
                    format!("{:?}", m.risk),
                )
            })
            .collect();

        let services = ServiceController::new()
            .query_services()
            .into_iter()
            .map(|s| (s.display_name, s.name, s.status))
            .collect();

        let cleaner = zb_infrastructure::windows_api::system_cleaner::SystemCleaner::new()
            .scan_categories()
            .iter()
            .map(|c| {
                (
                    c.name.clone(),
                    c.description.clone(),
                    c.risk.clone(),
                    c.size_bytes as f64 / 1048576.0,
                )
            })
            .collect();

        let bloatware = zb_shared::software::get_bloatware_catalog()
            .into_iter()
            .map(|b| (b.name, b.description))
            .collect();

        let software = zb_shared::software::get_software_catalog()
            .into_iter()
            .map(|s| (s.name, format!("{:?}", s.category), s.winget_id))
            .collect();

        (
            Self {
                current_tab: Tab::Dashboard,
                metrics: Default::default(),
                tweaks,
                services,
                cleaner_items: cleaner,
                bloatware,
                software,
                status: None,
            },
            Task::perform(
                async {
                    let c =
                        zb_infrastructure::windows_api::metrics_collector::MetricsCollector::new();
                    c.current().await
                },
                Message::MetricsUpdated,
            ),
        )
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_secs(2)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(t) => {
                self.current_tab = t;
                T::none()
            }
            Message::Tick => T::perform(
                async {
                    let c =
                        zb_infrastructure::windows_api::metrics_collector::MetricsCollector::new();
                    c.current().await
                },
                Message::MetricsUpdated,
            ),
            Message::MetricsUpdated(m) => {
                self.metrics = m;
                T::none()
            }
            Message::TweakApply(idx, ref id) => {
                let id2 = id.clone();
                T::perform(
                    async {
                        let tweaks = make_all_tweaks();
                        if let Some(t) = tweaks.iter().find(|tw| tw.metadata().id == id2) {
                            match t.apply().await {
                                Ok(r) => r.message,
                                Err(e) => e.to_string(),
                            }
                        } else {
                            "Not found".into()
                        }
                    },
                    move |msg| Message::TweakResult(idx, msg),
                )
            }
            Message::TweakResult(idx, msg) => {
                self.status = Some(format!("Tweak {0}: {1}", idx, msg));
                T::none()
            }
            Message::SvcStop(_, ref name) => {
                self.status = Some(match ServiceController::new().stop_service(name) {
                    Ok(m) => m,
                    Err(e) => e,
                });
                T::none()
            }
            Message::SvcDisable(_, ref name) => {
                self.status = Some(match ServiceController::new().disable_service(name) {
                    Ok(m) => m,
                    Err(e) => e,
                });
                T::none()
            }
            Message::SvcResult(_, msg) => {
                self.status = Some(msg);
                T::none()
            }
            Message::Clean(ref id) => {
                self.status = Some(format!("Cleaning {0}...", id));
                T::none()
            }
            Message::CleanResult(msg) => {
                self.status = Some(msg);
                T::none()
            }
            Message::DebloatRemove(ref name) => {
                self.status = Some(format!("Removing {0}...", name));
                T::none()
            }
            Message::DebloatResult(msg) => {
                self.status = Some(msg);
                T::none()
            }
            Message::SoftwareInstall(ref name) => {
                self.status = Some(format!("Installing {0} via Winget...", name));
                T::none()
            }
            Message::SoftwareResult(msg) => {
                self.status = Some(msg);
                T::none()
            }
        }
    }

    fn view(&self) -> Element<'static, Message> {
        let sidebar = container(
            column(
                Tab::ALL
                    .iter()
                    .map(|tab| {
                        let active = *tab == self.current_tab;
                        let mut btn = button(text(tab.label()).size(13)).width(Length::Fill);
                        if active {
                            btn = btn.style(|_, _| button::Style {
                                background: Some(Background::Color(Color::from_rgb(
                                    0.055, 0.647, 0.914,
                                ))),
                                text_color: Color::WHITE,
                                border: Border::default(),
                                ..Default::default()
                            });
                        }
                        btn.on_press(Message::TabSelected(*tab)).into()
                    })
                    .collect::<Vec<Element<Message>>>(),
            )
            .spacing(4)
            .padding(8),
        )
        .width(Length::Fixed(180.0));

        let content: Element<Message> = match self.current_tab {
            Tab::Dashboard => self.dashboard_view(),
            Tab::Tweaks => list_view("Tweaks", &self.tweaks, |i, id| Message::TweakApply(i, id)),
            Tab::Services => svc_view(&self.services),
            Tab::Cleaner => cleaner_view(&self.cleaner_items),
            Tab::Snapshots => text("Snapshots — created when you apply tweaks").into(),
            Tab::Debloat => list2_view(
                "Debloat",
                &self.bloatware,
                "Remove",
                |name| Message::DebloatRemove(name),
                "These can be reinstalled from Microsoft Store",
            ),
            Tab::Software => list2_view(
                "Software",
                &self.software,
                "Install",
                |id| Message::SoftwareInstall(id),
                "",
            ),
            Tab::Settings => settings_view(),
        };

        let body = column![row![sidebar, container(scrollable(content)).padding(16)].spacing(0)];
        if let Some(ref s) = self.status {
            container(text(s).size(12).color(Color::from_rgb(0.8, 0.8, 0.8)))
                .padding(8)
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.05, 0.05, 0.05))),
                    ..Default::default()
                })
                .into()
        } else {
            container(body).into()
        }
    }
}

// ===== VIEWS =====

fn card_style() -> impl Fn(&Theme) -> container::Style {
    |_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.13, 0.13, 0.13))),
        border: Border::default(),
        ..Default::default()
    }
}

fn small_btn(label: &str) -> iced::widget::Button<Message> {
    button(text(label).size(11)).padding(iced::Padding {
        top: 4.0,
        right: 10.0,
        bottom: 4.0,
        left: 10.0,
    })
}

impl App {
    fn dashboard_view(&self) -> Element<'static, Message> {
        let m = &self.metrics;
        let card = |l: String, v: String| -> Element<'static, Message> {
            container(
                column![
                    text(l).size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
                    text(v).size(24)
                ]
                .spacing(4),
            )
            .padding(16)
            .width(Length::Fill)
            .style(card_style())
            .into()
        };
        column![
            row![
                card("CPU Usage".into(), format!("{:.1}%", m.cpu_percent)),
                card("RAM Usage".into(), format!("{:.1}%", m.ram_percent))
            ]
            .spacing(12),
            text("29 tweaks · 19 services · 9 cleaner · 34 debloat").size(14),
        ]
        .spacing(16)
        .into()
    }
}

fn list_view(
    title: &str,
    items: &[(String, String, String)],
    on_action: fn(usize, String) -> Message,
) -> Element<'static, Message> {
    let mut col = column![text(title).size(20)].spacing(8);
    for (i, (name, desc, _risk)) in items.iter().enumerate() {
        let card = row![
            column![
                text(name.clone()).size(14).width(Length::Fill),
                text(desc.clone())
                    .size(12)
                    .color(Color::from_rgb(0.5, 0.5, 0.5))
            ]
            .spacing(2)
            .width(Length::Fill),
            small_btn("Apply").on_press(on_action(i, name.clone())),
        ]
        .spacing(8)
        .align_y(Alignment::Center);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col.into()
}

fn svc_view(items: &[(String, String, String)]) -> Element<'static, Message> {
    let mut col = column![text("Services").size(20)].spacing(8);
    for (i, (name, _svc_name, status)) in items.iter().enumerate() {
        let running = status == "Running";
        let sc = if running {
            Color::from_rgb(0.07, 0.73, 0.51)
        } else {
            Color::from_rgb(0.4, 0.4, 0.4)
        };
        let card = row![
            column![
                text(name.clone()).size(14).width(Length::Fill),
                text(if running { "Running" } else { "Stopped" })
                    .size(11)
                    .color(sc)
            ]
            .spacing(2)
            .width(Length::Fill),
            small_btn("Stop").on_press(Message::SvcStop(i, name.clone())),
            small_btn("Disable").on_press(Message::SvcDisable(i, name.clone())),
        ]
        .spacing(8)
        .align_y(Alignment::Center);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col.into()
}

fn cleaner_view(items: &[(String, String, String, f64)]) -> Element<'static, Message> {
    let mut col = column![text("Cleaner").size(20)].spacing(8);
    for (name, _desc, risk, mb) in items {
        let rc = if risk == "safe" {
            Color::from_rgb(0.07, 0.73, 0.51)
        } else {
            Color::from_rgb(0.96, 0.37, 0.04)
        };
        let card = row![
            column![
                text(name.clone()).size(14).width(Length::Fill),
                text(format!("{:.1} MB", mb))
                    .size(12)
                    .color(Color::from_rgb(0.5, 0.5, 0.5))
            ]
            .spacing(2)
            .width(Length::Fill),
            container(text(risk.clone()).size(10))
                .padding(4)
                .style(move |_| container::Style {
                    background: Some(Background::Color(rc)),
                    text_color: Some(Color::WHITE),
                    border: Border::default(),
                    ..Default::default()
                }),
            small_btn("Clean").on_press(Message::Clean(name.clone())),
        ]
        .spacing(8)
        .align_y(Alignment::Center);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col.into()
}

fn list2_view(
    title: &str,
    items: &[(String, String)],
    btn_label: &str,
    on_btn: fn(String) -> Message,
    info: &str,
) -> Element<'static, Message> {
    let mut col = column![text(title).size(20)].spacing(8);
    if !info.is_empty() {
        col = col.push(text(info).size(11).color(Color::from_rgb(0.96, 0.37, 0.04)));
    }
    for (name, desc_or_id) in items {
        let card = row![
            column![
                text(name.clone()).size(14).width(Length::Fill),
                text(desc_or_id.clone())
                    .size(12)
                    .color(Color::from_rgb(0.5, 0.5, 0.5))
            ]
            .spacing(2)
            .width(Length::Fill),
            small_btn(btn_label).on_press(on_btn(desc_or_id.clone())),
        ]
        .spacing(8)
        .align_y(Alignment::Center);
        col = col.push(container(card).padding(12).style(card_style()));
    }
    col.into()
}

fn settings_view() -> Element<'static, Message> {
    column![
        text("Settings").size(20),
        container(
            column![
                text("Version").size(14),
                text("v0.0.5 — Iced Edition")
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
                text("29 real tweaks · 19 real services · 9 cleaner · 34 debloat")
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
    .into()
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
