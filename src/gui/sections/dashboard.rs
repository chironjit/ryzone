use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length};

use crate::gui::formatting::{format_frequency, format_time_remaining};
use crate::gui::style::{metrics_card_style, stat_tip_style};
use crate::model::State;
use crate::updates::Message;

pub fn view(state: &State) -> Element<Message> {
    column![
        // Row 1: Performance & Status - Real-time metrics
        create_performance_status_row(state),
        
        // Row 2: Configuration & Limits - Power settings
        create_configuration_row(state),
    ]
    .spacing(16)
    .width(Length::Fill)
    .into()
}

fn create_performance_status_row(state: &State) -> Element<Message> {
    let power_source_label = if state.batt_status == "Discharging" { "Battery" } else { "AC Power" };
    let power_source_color = if state.batt_status == "Discharging" { [1.0, 0.8, 0.3] } else { [0.3, 1.0, 0.5] };
    
    row![
        // CPU Performance - Left-right layout
        container(
            row![
                column![
                    text("CPU").size(16).color([0.3, 0.8, 1.0]),
                    text("Current").size(11).color([0.6, 0.6, 0.6]),
                ]
                .spacing(2),
                Space::with_width(Length::Fill),
                column![
                    text(format_frequency(state.current_max_freq)).size(24).color([1.0, 1.0, 1.0]),
                    row![
                        text("Min: ").size(11).color([0.6, 0.6, 0.6]),
                        text(format_frequency(state.min_freq_5min)).size(11).color([0.8, 0.8, 0.8]),
                        text(" Max: ").size(11).color([0.6, 0.6, 0.6]),
                        text(format_frequency(state.max_freq_5min)).size(11).color([0.8, 0.8, 0.8]),
                    ]
                ]
                .spacing(4)
                .align_x(Horizontal::Right),
            ]
            .align_y(Vertical::Center)
        )
        .style(metrics_card_style())
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fixed(90.0)),
        
        // GPU Performance - Left-right layout
        container(
            row![
                column![
                    text("GPU").size(16).color([0.8, 0.3, 1.0]),
                    text("Current").size(11).color([0.6, 0.6, 0.6]),
                ]
                .spacing(2),
                Space::with_width(Length::Fill),
                column![
                    text(format_frequency(state.current_gpu_freq)).size(24).color([1.0, 1.0, 1.0]),
                    row![
                        text("Min: ").size(11).color([0.6, 0.6, 0.6]),
                        text(format_frequency(state.min_gpu_freq_5min)).size(11).color([0.8, 0.8, 0.8]),
                        text(" Max: ").size(11).color([0.6, 0.6, 0.6]),
                        text(format_frequency(state.max_gpu_freq_5min)).size(11).color([0.8, 0.8, 0.8]),
                    ]
                ]
                .spacing(4)
                .align_x(Horizontal::Right),
            ]
            .align_y(Vertical::Center)
        )
        .style(metrics_card_style())
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fixed(90.0)),
        
        // Power Consumption - Left-right layout
        container(
            row![
                column![
                    text("Power").size(16).color([1.0, 0.8, 0.3]),
                    text("Total APU").size(11).color([0.6, 0.6, 0.6]),
                ]
                .spacing(2),
                Space::with_width(Length::Fill),
                column![
                    text(format!("{:.1} W", state.curr_stapm_value as f32 / 1000.0)).size(24).color([1.0, 1.0, 1.0]),
                    text(power_source_label).size(12).color(power_source_color),
                ]
                .spacing(4)
                .align_x(Horizontal::Right),
            ]
            .align_y(Vertical::Center)
        )
        .style(metrics_card_style())
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fixed(90.0)),
        
        // Battery Info - Left-right layout
        container(
            row![
                column![
                    text("Battery").size(16).color([0.6, 1.0, 0.8]),
                    text("Runtime").size(11).color([0.6, 0.6, 0.6]),
                ]
                .spacing(2),
                Space::with_width(Length::Fill),
                column![
                    text(format_time_remaining(state.batt_time)).size(20).color([1.0, 1.0, 1.0]),
                    text(format!("{:.1} W draw", state.batt_power as f64 / 10.0)).size(12).color([0.8, 0.8, 0.8]),
                ]
                .spacing(4)
                .align_x(Horizontal::Right),
            ]
            .align_y(Vertical::Center)
        )
        .style(metrics_card_style())
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fixed(90.0)),
    ]
    .spacing(20)
    .into()
}

fn create_configuration_row(state: &State) -> Element<Message> {
    row![
        // Active Power Profile - Wider card with more info
        container(
            row![
                column![
                    text("Power Profile").size(14).color([0.3, 0.8, 1.0]),
                    text(format!("{:?}", state.active_profile)).size(18).color([1.0, 1.0, 1.0]),
                ]
                .spacing(2),
                Space::with_width(Length::Fill),
                column![
                    text(if state.batt_status == "Discharging" { "On Battery" } else { "AC Power" })
                        .size(12).color(if state.batt_status == "Discharging" { [1.0, 0.8, 0.3] } else { [0.3, 1.0, 0.5] }),
                    text("Status").size(10).color([0.6, 0.6, 0.6]),
                ]
                .spacing(2)
                .align_x(Horizontal::Right),
            ]
            .align_y(Vertical::Center)
        )
        .style(stat_tip_style())
        .padding(16)
        .width(Length::FillPortion(2))  // Make profile card wider
        .height(Length::Fixed(70.0)),
        
        // Fast Limit
        container(
            row![
                column![
                    text("Fast Limit").size(12).color([0.6, 0.6, 0.6]),
                    text(format!("{} W", state.curr_fast_limit)).size(16).color([0.3, 1.0, 0.6]),
                ]
                .align_x(Horizontal::Center)
            ]
            .align_y(Vertical::Center)
        )
        .style(stat_tip_style())
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fixed(70.0)),
        
        // Slow Limit  
        container(
            row![
                column![
                    text("Slow Limit").size(12).color([0.6, 0.6, 0.6]),
                    text(format!("{} W", state.curr_slow_limit)).size(16).color([1.0, 0.8, 0.3]),
                ]
                .align_x(Horizontal::Center)
            ]
            .align_y(Vertical::Center)
        )
        .style(stat_tip_style())
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fixed(70.0)),
        
        // STAPM Limit
        container(
            row![
                column![
                    text("STAPM Limit").size(12).color([0.6, 0.6, 0.6]),
                    text(format!("{} W", state.curr_stapm_limit)).size(16).color([1.0, 0.5, 0.3]),
                ]
                .align_x(Horizontal::Center)
            ]
            .align_y(Vertical::Center)
        )
        .style(stat_tip_style())
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fixed(70.0)),
        
        // Temperature Limit
        container(
            row![
                column![
                    text("Temp Limit").size(12).color([0.6, 0.6, 0.6]),
                    text(format!("{}°C", state.curr_tctl_limit)).size(16).color([1.0, 0.5, 0.3]),
                ]
                .align_x(Horizontal::Center)
            ]
            .align_y(Vertical::Center)
        )
        .style(stat_tip_style())
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fixed(70.0)),
    ]
    .spacing(16)
    .into()
}