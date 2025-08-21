use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, text, pick_list, row};
use iced::{Element, Length};

use crate::gui::style::{card_style, tab_content_style};
use crate::model::{State, AppTheme, PowerUnit, TemperatureUnit};
use crate::updates::Message;

pub fn view(state: &State) -> Element<Message> {
    // Create compact setting rows
    let display_settings = container(
        column![
            text("Display Settings").size(18).color([0.3, 0.8, 1.0]),
            row![
                // Theme setting - compact
                column![
                    text("Theme").size(14),
                    pick_list(
                        vec![
                            AppTheme::Light,
                            AppTheme::Dark,
                            AppTheme::CatppuccinLatte,
                            AppTheme::CatppuccinFrappe,
                            AppTheme::CatppuccinMacchiato,
                            AppTheme::CatppuccinMocha,
                            AppTheme::TokyoNight,
                            AppTheme::TokyoNightStorm,
                            AppTheme::TokyoNightLight,
                            AppTheme::KanagawaWave,
                            AppTheme::KanagawaDragon,
                            AppTheme::KanagawaLotus,
                            AppTheme::Moonfly,
                            AppTheme::Nightfly,
                            AppTheme::Oxocarbon,
                        ],
                        Some(state.selected_theme),
                        Message::SetTheme
                    )
                    .width(Length::Fill),
                ]
                .spacing(6)
                .width(Length::Fill),
                
                // Power unit setting - compact  
                column![
                    text("Power Unit").size(14),
                    pick_list(
                        vec![PowerUnit::MilliWatt, PowerUnit::Watt],
                        Some(state.power_display_unit),
                        Message::SetPowerUnit
                    )
                    .width(Length::Fill),
                ]
                .spacing(6)
                .width(Length::Fill),
                
                // Temperature unit setting - compact
                column![
                    text("Temperature Unit").size(14),
                    pick_list(
                        vec![TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit],
                        Some(state.temperature_display_unit),
                        Message::SetTemperatureUnit
                    )
                    .width(Length::Fill),
                ]
                .spacing(6)
                .width(Length::Fill),
            ]
            .spacing(20),
        ]
        .spacing(12)
    )
    .style(card_style())
    .padding(16);

    let usage_guide = container(
        column![
            text("Quick Guide").size(18).color([0.8, 0.6, 1.0]),
            row![
                column![
                    text("• Use Profiles section for power profiles").size(12),
                    text("• Use Custom Overrides for manual control").size(12),
                ]
                .spacing(4)
                .width(Length::Fill),
                column![
                    text("• All power data stored internally in mW").size(12),
                    text("• All temperature data stored internally in °C").size(12),
                ]
                .spacing(4)
                .width(Length::Fill),
            ]
            .spacing(20),
        ]
        .spacing(8)
    )
    .style(card_style())
    .padding(16);

    let content = column![
        container(text("Settings").size(20))
            .padding([0, 10])
            .align_x(Horizontal::Left),
        display_settings,
        usage_guide,
    ]
    .spacing(16)
    .padding(10)
    .width(Length::Fill);

    container(content)
        .style(tab_content_style())
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .into()
}
