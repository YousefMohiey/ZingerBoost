use iced::widget::{button, column, container, row, text};
use iced::{Background, Border, Color, Element, Length, Task, Theme};

fn main() -> iced::Result {
    iced::application("Test", App::update, App::view)
        .theme(|state| state.theme())
        .run_with(App::new)
}

struct App {
    tab: usize,
    dark: bool,
}
#[derive(Debug, Clone)]
enum Message {
    Next,
    Toggle,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (Self { tab: 0, dark: true }, Task::none())
    }
    fn theme(&self) -> Theme {
        if self.dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
    fn update(&mut self, m: Message) -> Task<Message> {
        match m {
            Message::Next => {
                self.tab = (self.tab + 1) % 8;
                Task::none()
            }
            Message::Toggle => {
                self.dark = !self.dark;
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let sidebar = container(
            column(
                (0..8)
                    .map(|i| {
                        let mut btn = button(text(format!("Tab {i}")).size(13)).width(Length::Fill);
                        if i == self.tab {
                            btn = btn.style(|_, _| button::Style {
                                background: Some(Background::Color(Color::from_rgb(
                                    0.055, 0.647, 0.914,
                                ))),
                                text_color: Color::WHITE,
                                border: Border::default(),
                                ..Default::default()
                            });
                        }
                        btn.on_press(Message::Next).into()
                    })
                    .collect::<Vec<Element<Message>>>(),
            )
            .spacing(4)
            .padding(8),
        )
        .width(Length::Fixed(180.0));

        let content: Element<Message> = text(format!("Tab {0}", self.tab)).into();
        container(row![sidebar, container(content).padding(16)]).into()
    }
}
