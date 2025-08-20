use iced::alignment::{self, Horizontal, Vertical};
use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length};

use crate::gui::formatting::{format_frequency, format_time_remaining};
use crate::gui::style::{card_style, header_style, metrics_card_style};
use crate::model::State;
use crate::updates::Message;

pub fn view(state: &State) -> Element<Message> {
    column![
        // Modern header
        container(
            row![
                column![
                    text("Ryzone")
                        .size(28)
                        .color([0.2, 0.7, 1.0]),
                    text("AMD Ryzen Power Control Dashboard")
                        .size(14)
                        .color([0.6, 0.6, 0.6]),
                ]
                .spacing(4),
                Space::with_width(Length::Fill),
                column![
                    text("Status: Active")
                        .size(12)
                        .color([0.3, 0.8, 0.3]),
                    text("TokyoNight Theme")
                        .size(10)
                        .color([0.5, 0.5, 0.5]),
                ]
                .spacing(2)
                .align_x(Horizontal::Right),
            ]
            .align_y(Vertical::Center)
            .padding(20)
        )
        .style(header_style())
        .width(Length::Fill),
        
        // Enhanced metrics cards
        row![
            container(create_processor_view(state))
                .style(metrics_card_style())
                .padding(24)
                .width(Length::Fill)
                .height(Length::Fixed(140.0)),
            container(create_gpu_view(state))
                .style(metrics_card_style())
                .padding(24)
                .width(Length::Fill)
                .height(Length::Fixed(140.0)),
            container(create_power_view(state))
                .style(metrics_card_style())
                .padding(24)
                .width(Length::Fill)
                .height(Length::Fixed(140.0))
        ]
        .spacing(24)
    ]
    .spacing(24)
    .width(Length::Fill)
    .into()
}

fn create_processor_view(state: &State) -> Element<Message> {
    column![
        row![
            column![
                text("CPU").size(20).color([0.3, 0.8, 1.0]),
                text("Current").size(10).color([0.6, 0.6, 0.6]),
            ].spacing(2),
            Space::with_width(Length::Fill),
            column![text(format_frequency(state.current_max_freq)).size(28).color([1.0, 1.0, 1.0]),]
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(2),
        Space::with_height(8),
        row![
            column![
                text("Min (last 5 mins)").size(12).color([0.6, 0.6, 0.6]),
                text("Max (last 5 mins)").size(12).color([0.6, 0.6, 0.6]),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format_frequency(state.min_freq_5min)).size(12).color([0.8, 0.8, 0.8]),
                text(format_frequency(state.max_freq_5min)).size(12).color([0.8, 0.8, 0.8]),
            ]
            .spacing(2)
        ]
    ]
    .into()
}

fn create_gpu_view(state: &State) -> Element<Message> {
    column![
        row![
            column![
                text("GPU").size(20).color([0.8, 0.3, 1.0]),
                text("Current").size(10).color([0.6, 0.6, 0.6]),
            ].spacing(2),
            Space::with_width(Length::Fill),
            column![text(format_frequency(state.current_gpu_freq)).size(28).color([1.0, 1.0, 1.0]),]
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(2),
        Space::with_height(8),
        row![
            column![
                text("Min (last 5 mins)").size(12).color([0.6, 0.6, 0.6]),
                text("Max (last 5 mins)").size(12).color([0.6, 0.6, 0.6]),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format_frequency(state.min_gpu_freq_5min)).size(12).color([0.8, 0.8, 0.8]),
                text(format_frequency(state.max_gpu_freq_5min)).size(12).color([0.8, 0.8, 0.8]),
            ]
            .spacing(2)
        ]
    ]
    .into()
}

fn create_power_view(state: &State) -> Element<Message> {
    column![
        row![
            column![
                text("Power").size(20).color([1.0, 0.8, 0.3]),
                text("APU Total").size(10).color([0.6, 0.6, 0.6]),
            ].spacing(2),
            Space::with_width(Length::Fill),
            column![text(format!("{:.1} W", state.curr_stapm_value as f32 / 1000.0)).size(28).color([1.0, 1.0, 1.0]),]
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(2),
        Space::with_height(8),
        row![
            column![
                text(format!(
                    "Battery {}",
                    match state.batt_status.as_str() {
                        "Discharging" => "usage",
                        "Charging" => "charging",
                        _ => "",
                    }
                ))
                .size(12)
                .color([0.6, 0.6, 0.6]),
                text("Est Running time").size(12).color([0.6, 0.6, 0.6]),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format!("{:.1} W", state.batt_power as f64 / 10.0)).size(12).color([0.8, 0.8, 0.8]),
                text(format_time_remaining(state.batt_time)).size(12).color([0.8, 0.8, 0.8]),
            ]
            .spacing(2)
        ]
    ]
    .into()
}
