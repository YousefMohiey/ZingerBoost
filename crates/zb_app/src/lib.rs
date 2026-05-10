// Minimal Iced app — guaranteed to compile
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task, Theme};
use std::time::Duration;

pub fn run() -> iced::Result {
    zb_infrastructure::logging::init_logging();
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
    MetricsUpdated(zb_shared::types::SystemMetrics),
    ThemeToggled,
}

pub struct App {
    current_tab: Tab,
    dark_mode: bool,
    metrics: zb_shared::types::SystemMetrics,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_tab: Tab::Dashboard,
                dark_mode: true,
                metrics: Default::default(),
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
                self.current_tab = tab;
                Task::none()
            }
            Message::RefreshMetrics => Task::perform(
                async {
                    let c =
                        zb_infrastructure::windows_api::metrics_collector::MetricsCollector::new();
                    c.current().await
                },
                Message::MetricsUpdated,
            ),
            Message::MetricsUpdated(m) => {
                self.metrics = m;
                Task::none()
            }
            Message::ThemeToggled => {
                self.dark_mode = !self.dark_mode;
                Task::none()
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
                        let mut btn = button(text(tab.label()).size(13)).width(160);
                        if active {
                            btn = btn.style(|_, _| button::Style {
                                background: Some(Background::Color(Color::from_rgb(
                                    0.055, 0.647, 0.914,
                                ))),
                                text_color: Color::WHITE,
                                border: Border {
                                    radius: 8.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        }
                        btn.on_press(Message::TabSelected(*tab)).into()
                    })
                    .collect(),
            )
            .spacing(4)
            .padding(8),
        )
        .width(Length::Fixed(180.0));

        let content: Element<'static, Message> = match self.current_tab {
            Tab::Dashboard => {
                let m = &self.metrics;
                column![
                    row![
                        _mc("CPU", &format!("{:.1}%", m.cpu_percent), ""),
                        _mc(
                            "RAM",
                            &format!("{:.1}%", m.ram_percent),
                            &format!("{} / {} MB", m.ram_used_mb, m.ram_total_mb)
                        )
                    ]
                    .spacing(12),
                    text("29 tweaks · 19 services · 9 cleaner · 34 debloat").size(14),
                ]
                .spacing(16)
                .into()
            }
            _ => text(format!("{} — ready", self.current_tab.label())).into(),
        };

        let main = row![sidebar, container(scrollable(content)).padding(16)];
        container(main).into()
    }
}

fn _mc(label: &str, value: &str, sub: &str) -> Element<'static, Message> {
    container(
        column![
            text(label).size(12),
            text(value).size(24),
            text(sub).size(11)
        ]
        .spacing(4),
    )
    .padding(16)
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.13, 0.13, 0.13))),
        border: Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}
