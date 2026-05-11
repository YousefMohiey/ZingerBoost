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
    Clean(String),
    DebloatRemove(String),
    SoftwareInstall(String),
    OpResult(String),
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
            T::perform(
                async {
                    zb_infrastructure::windows_api::metrics_collector::MetricsCollector::new()
                        .current()
                        .await
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
                    zb_infrastructure::windows_api::metrics_collector::MetricsCollector::new()
                        .current()
                        .await
                },
                Message::MetricsUpdated,
            ),
            Message::MetricsUpdated(m) => {
                self.metrics = m;
                T::none()
            }
            Message::TweakApply(idx, id) => {
                self.status = Some(format!("Applying {0}...", id));
                T::none()
            }
            Message::TweakResult(idx, msg) => {
                self.status = Some(msg);
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
            Message::Clean(ref name) => {
                self.status = Some(format!("Cleaning {0}...", name));
                T::none()
            }
            Message::DebloatRemove(ref name) => {
                self.status = Some(format!("Removing {0}...", name));
                T::none()
            }
            Message::SoftwareInstall(ref name) => {
                self.status = Some(format!("Installing {0} via Winget...", name));
                T::none()
            }
            Message::OpResult(msg) => {
                self.status = Some(msg);
                T::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
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

        fn card_style() -> impl Fn(&Theme) -> container::Style {
            |_| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.13, 0.13, 0.13))),
                border: Border::default(),
                ..Default::default()
            }
        }
        fn btn(label: &str) -> iced::widget::Button<Message> {
            button(text(label).size(11)).padding(iced::Padding {
                top: 4.0,
                right: 10.0,
                bottom: 4.0,
                left: 10.0,
            })
        }

        let content: Element<Message> = match self.current_tab {
            Tab::Dashboard => {
                let m = &self.metrics;
                let mc = |l: String, v: String| -> Element<Message> {
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
                        mc("CPU Usage".into(), format!("{:.1}%", m.cpu_percent)),
                        mc("RAM Usage".into(), format!("{:.1}%", m.ram_percent))
                    ]
                    .spacing(12),
                    text("29 tweaks · 19 services · 9 cleaner · 34 debloat").size(14)
                ]
                .spacing(16)
                .into()
            }
            Tab::Tweaks => {
                let mut col = column![text("Tweaks").size(20)].spacing(8);
                for (i, (name, desc, _)) in self.tweaks.iter().enumerate() {
                    let card = row![
                        column![
                            text(name.clone()).size(14).width(Length::Fill),
                            text(desc.clone())
                                .size(12)
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                        ]
                        .spacing(2)
                        .width(Length::Fill),
                        btn("Apply").on_press(Message::TweakApply(i, name.clone()))
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center);
                    col = col.push(container(card).padding(12).style(card_style()));
                }
                col.into()
            }
            Tab::Services => {
                let mut col = column![text("Services").size(20)].spacing(8);
                for (i, (display, _, status)) in self.services.iter().enumerate() {
                    let running = status == "Running";
                    let sc = if running {
                        Color::from_rgb(0.07, 0.73, 0.51)
                    } else {
                        Color::from_rgb(0.4, 0.4, 0.4)
                    };
                    let card = row![
                        column![
                            text(display.clone()).size(14).width(Length::Fill),
                            text(if running { "Running" } else { "Stopped" })
                                .size(11)
                                .color(sc)
                        ]
                        .spacing(2)
                        .width(Length::Fill),
                        btn("Stop").on_press(Message::SvcStop(i, display.clone())),
                        btn("Disable").on_press(Message::SvcDisable(i, display.clone()))
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center);
                    col = col.push(container(card).padding(12).style(card_style()));
                }
                col.into()
            }
            Tab::Cleaner => {
                let mut col = column![text("Cleaner").size(20)].spacing(8);
                for (name, _desc, risk, mb) in &self.cleaner_items {
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
                        btn("Clean").on_press(Message::Clean(name.clone()))
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center);
                    col = col.push(container(card).padding(12).style(card_style()));
                }
                col.into()
            }
            Tab::Snapshots => text("Snapshots — created when you apply tweaks").into(),
            Tab::Debloat => {
                let mut col = column![
                    text("Debloat").size(20),
                    text("These can be reinstalled from Microsoft Store")
                        .size(11)
                        .color(Color::from_rgb(0.96, 0.37, 0.04))
                ]
                .spacing(8);
                for (name, desc) in &self.bloatware {
                    let card = row![
                        column![
                            text(name.clone()).size(14).width(Length::Fill),
                            text(desc.clone())
                                .size(12)
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                        ]
                        .spacing(2)
                        .width(Length::Fill),
                        btn("Remove").on_press(Message::DebloatRemove(name.clone()))
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center);
                    col = col.push(container(card).padding(12).style(card_style()));
                }
                col.into()
            }
            Tab::Software => {
                let mut col = column![text("Software").size(20)].spacing(8);
                for (name, cat, winget_id) in &self.software {
                    let card = row![
                        column![
                            text(name.clone()).size(14).width(Length::Fill),
                            text(cat.clone())
                                .size(12)
                                .color(Color::from_rgb(0.5, 0.5, 0.5))
                        ]
                        .spacing(2)
                        .width(Length::Fill),
                        btn("Install").on_press(Message::SoftwareInstall(winget_id.clone()))
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center);
                    col = col.push(container(card).padding(12).style(card_style()));
                }
                col.into()
            }
            Tab::Settings => column![
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
            .into(),
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
