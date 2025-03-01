use iced::time::{self, Duration};
use iced::{window, Size, Subscription, Theme};

mod gui;
mod model;
mod updates;
mod utils;

use gui::view;
use model::State;
use updates::{update, Message};

fn update_state_values(_: &State) -> Subscription<Message> {
    time::every(Duration::from_secs(1)).map(|_| Message::UpdateStateValues)
}

fn main() -> iced::Result {
    iced::application("Ryzone", update, view)
        .window(window::Settings {
            min_size: Some(Size::new(600.0, 450.0)),
            size: Size::new(800.0, 600.0),
            icon: Some(
                window::icon::from_file_data(
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.ico")),
                    None,
                )
                .expect("icon file should be reachable and in ICO file format"),
            ),
            ..Default::default()
        })
        .theme(theme)
        .subscription(update_state_values)
        .run()
}

fn theme(_state: &State) -> Theme {
    Theme::TokyoNightStorm
}
