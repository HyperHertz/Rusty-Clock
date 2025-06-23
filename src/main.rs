use chrono::Local;
use iced::{executor, widget::Text, Application, Element, Settings, Subscription, Theme};
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

struct Clock {
    time: String,
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let now = Local::now().format("%H:%M:%S").to_string();
        (Self { time: now }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Rust Clock")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        if let Message::Tick = message {
            self.time = Local::now().format("%H:%M:%S").to_string();
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Text::new(&self.time).size(48).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}

fn main() -> iced::Result {
    Clock::run(Settings::default())
}
