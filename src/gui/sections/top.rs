use iced::widget::{container, row, column, text, Space};
use iced::alignment::{self, Horizontal, Vertical};
use iced::{Element, Length};

use crate::model::State;
use crate::updates::Message;
use crate::gui::style::{card_style, header_style};
use crate::gui::formatting::{format_frequency, format_time_remaining};

pub fn view(state: &State) -> Element<Message> {
    column! [
        // Top bar
        row![
            container(
                text("Ryzone - Control Mobile Ryzen Power States")
            )
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(header_style())
            .width(Length::Fill)
            .height(Length::Fixed(33.3))
        ],

        row![
            // Processor container
            container(create_processor_view(state))
                .style(card_style())
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fixed(120.0)),
    
            // GPU container
            container(create_gpu_view(state))
                .style(card_style())
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fixed(120.0)),
    
            // Power container
            container(create_power_view(state))
                .style(card_style())
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fixed(120.0))
        ]
        .spacing(20)
    ]
    .spacing(20)
    .width(Length::Fill)
    .into()
    
}

fn create_processor_view(state: &State) -> Element<Message> {
    column![
        row![
            column![
                text("CPU").size(20),
                text("Current").size(10),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format_frequency(state.current_max_freq)).size(28),
            ]
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(2),

        Space::with_height(8),

        row![
            column![
                text("Min (last 5 mins)").size(12),
                text("Max (last 5 mins)").size(12),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format_frequency(state.min_freq_5min)).size(12),
                text(format_frequency(state.max_freq_5min)).size(12),
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
                text("GPU").size(20),
                text("Current").size(10),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format_frequency(state.current_gpu_freq)).size(28),
            ]
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(2),
 
        Space::with_height(8),
 
        row![
            column![
                text("Min (last 5 mins)").size(12),
                text("Max (last 5 mins)").size(12),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format_frequency(state.min_gpu_freq_5min)).size(12),
                text(format_frequency(state.max_gpu_freq_5min)).size(12),
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
                text("Power").size(20),
                text("APU Total").size(10),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format!("{:.1} W", state.curr_stapm_value as f32 / 1000.0)).size(28),
            ]
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(2),
 
        Space::with_height(8),
 
        row![
            column![
                text(format!("Battery {}", match state.batt_status.as_str() {
                    "Discharging" => "usage",
                    "Charging" => "charging",
                    _ => ""
                })).size(12),
                text("Est Running time").size(12),
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            column![
                text(format!("{:.1} W", state.batt_power as f64 / 10.0)).size(12),
                text(format_time_remaining(state.batt_time)).size(12),
            ]
            .spacing(2)
        ]
    ]
    .into()
}