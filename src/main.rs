use iced::time::{self, Duration};
use iced::{window, Size, Subscription, Theme};
use model::AppTheme;

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
            min_size: Some(Size::new(900.0, 650.0)),
            size: Size::new(1200.0, 800.0),
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

fn theme(state: &State) -> Theme {
    match state.selected_theme {
        AppTheme::Light => Theme::Light,
        AppTheme::Dark => Theme::Dark,
        AppTheme::CatppuccinLatte => Theme::CatppuccinLatte,
        AppTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
        AppTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
        AppTheme::CatppuccinMocha => Theme::CatppuccinMocha,
        AppTheme::TokyoNight => Theme::TokyoNight,
        AppTheme::TokyoNightStorm => Theme::TokyoNightStorm,
        AppTheme::TokyoNightLight => Theme::TokyoNightLight,
        AppTheme::KanagawaWave => Theme::KanagawaWave,
        AppTheme::KanagawaDragon => Theme::KanagawaDragon,
        AppTheme::KanagawaLotus => Theme::KanagawaLotus,
        AppTheme::Moonfly => Theme::Moonfly,
        AppTheme::Nightfly => Theme::Nightfly,
        AppTheme::Oxocarbon => Theme::Oxocarbon,
    }
}
