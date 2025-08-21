use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, checkbox, column, container, row, text, text_input, tooltip};
use iced::{Element, Length};

use crate::gui::style::{card_style, hint_text_style, tab_content_style, text_input_style};
use crate::model::state::ActiveSection;
use crate::model::State;
use crate::updates::Message;
use crate::utils::units::{format_power, format_temperature};
pub fn view(state: &State) -> Element<Message> {
    container(if state.active_section != ActiveSection::Custom {
        create_profile_enabler_overlay(state)
    } else {
        column![
            create_column_titles(),
            create_fast_limit_row(state),
            create_slow_limit_row(state),
            create_stapm_limit_row(state),
            create_tctl_limit_row(state)
        ]
        .spacing(10)
        .padding(20)
        .into()
    })
    .style(tab_content_style())
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn create_profile_enabler_overlay(state: &State) -> Element<Message> {
    container(
        column![checkbox(
            "Enable Custom Overrides",
            state.active_section == ActiveSection::Custom,
        )
        .on_toggle(|_| Message::SetActiveSection(ActiveSection::Custom))]
        .align_x(Horizontal::Center),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}

fn create_column_titles<'a>() -> Element<'a, Message> {
    container(
        row![
            container(
                container(text("").size(14))
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
            )
            .width(Length::FillPortion(4)),
            container(
                container(text("Current").size(14))
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
            )
            .width(Length::FillPortion(2)),
            container(
                container(text("Limit").size(14))
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
            )
            .width(Length::FillPortion(2)),
            container(
                container(text("Set New Limit").size(14))
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
            )
            .width(Length::FillPortion(2)),
            container(
                container(text("").size(14))
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
            )
            .width(Length::FillPortion(2)),
            container(
                container(text("").size(14))
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
            )
            .width(Length::FillPortion(1)),
        ]
        .spacing(20),
    )
    .style(container::transparent)
    .padding(10)
    .width(Length::Fill)
    .into()
}

fn create_fast_limit_row(state: &State) -> Element<Message> {
    container(
        row![
            container(
                container(text("Fast Limit").size(16))
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .width(Length::FillPortion(4)),
            container(
                container(text(format_power(state.curr_fast_value, state.power_display_unit)).size(16))
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .width(Length::FillPortion(2)),
            container(
                container(text(format_power(state.curr_fast_limit, state.power_display_unit)).size(16))
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .width(Length::FillPortion(2)),
            container(
                container(
                    text_input("", &state.fast_input)
                        .style(text_input_style())
                        .on_input(Message::FastLimitInputChanged)
                        .align_x(Horizontal::Center)
                )
                .align_x(Horizontal::Center)
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .width(Length::FillPortion(2)),
            container(
                container(
                    button("Set")
                        .on_press(Message::SetFastLimit(state.fast_input.parse().unwrap_or(0)))
                )
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .width(Length::FillPortion(2)),
            create_tooltip(
                match state.power_display_unit {
                    crate::model::PowerUnit::MilliWatt => "Enter value in mW.\nAccepted value range\nbetween 4000 & 65000.",
                    crate::model::PowerUnit::Watt => "Enter value in W.\nAccepted value range\nbetween 4 & 65.",
                }
            )
            .width(Length::FillPortion(1))
        ]
        .spacing(20),
    )
    .style(card_style())
    .padding(10)
    .width(Length::Fill)
    .height(Length::Fixed(50.0))
    .into()
}

fn create_slow_limit_row(state: &State) -> Element<Message> {
    container(
       row![
           create_label_container("Slow Limit"),
           create_value_container(format_power(state.curr_slow_value, state.power_display_unit)),
           create_value_container(format_power(state.curr_slow_limit, state.power_display_unit)),
           create_input_container(&state.slow_input, Message::SlowLimitInputChanged),
           create_button_container(Message::SetSlowLimit(state.slow_input.parse().unwrap_or(0))),
           create_tooltip(
               match state.power_display_unit {
                   crate::model::PowerUnit::MilliWatt => "Enter value in mW.\nAccepted value range\nbetween 4000 & 65000.\nSlow limit must be less\nthan or equal to fast limit.",
                   crate::model::PowerUnit::Watt => "Enter value in W.\nAccepted value range\nbetween 4 & 65.\nSlow limit must be less\nthan or equal to fast limit.",
               }
           )
               .width(Length::FillPortion(1))
       ]
       .spacing(20)
   )
   .style(card_style())
   .padding(10)
   .width(Length::Fill)
   .height(Length::Fixed(50.0))
   .into()
}

fn create_stapm_limit_row(state: &State) -> Element<Message> {
    container(
       row![
           create_label_container("STAPM Limit"),
           create_value_container(format_power(state.curr_stapm_value, state.power_display_unit)),
           create_value_container(format_power(state.curr_stapm_limit, state.power_display_unit)),
           create_input_container(&state.stapm_input, Message::StapmLimitInputChanged),
           create_button_container(Message::SetStapmLimit(state.stapm_input.parse().unwrap_or(0))),
           create_tooltip(
               match state.power_display_unit {
                   crate::model::PowerUnit::MilliWatt => "Enter value in mW.\nAccepted value range\nbetween 4000 & 65000.\nStapm limit must be less\nthan or equal to slow limit.",
                   crate::model::PowerUnit::Watt => "Enter value in W.\nAccepted value range\nbetween 4 & 65.\nStapm limit must be less\nthan or equal to slow limit.",
               }
           )
               .width(Length::FillPortion(1))
       ]
       .spacing(20)
   )
   .style(card_style())
   .padding(10)
   .width(Length::Fill)
   .height(Length::Fixed(50.0))
   .into()
}

fn create_tctl_limit_row(state: &State) -> Element<Message> {
    container(
        row![
            create_label_container("TCTL Limit"),
            create_value_container(format_temperature(state.curr_tctl_value, state.temperature_display_unit)),
            create_value_container(format_temperature(state.curr_tctl_limit, state.temperature_display_unit)),
            create_input_container(&state.tctl_input, Message::TctlLimitInputChanged),
            create_button_container(Message::SetTctlLimit(state.tctl_input.parse().unwrap_or(0))),
            create_tooltip(
                match state.temperature_display_unit {
                    crate::model::TemperatureUnit::Celsius => "Enter value in °C.\nAccepted value range\nbetween 40 and 100.",
                    crate::model::TemperatureUnit::Fahrenheit => "Enter value in °F.\nAccepted value range\nbetween 104 and 212.",
                }
            )
                .width(Length::FillPortion(1))
        ]
        .spacing(20),
    )
    .style(card_style())
    .padding(10)
    .width(Length::Fill)
    .height(Length::Fixed(50.0))
    .into()
}

fn create_label_container<'a>(label: &'a str) -> container::Container<'a, Message> {
    container(
        container(text(label).size(16))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::FillPortion(4))
}

fn create_value_container<'a>(value: String) -> container::Container<'a, Message> {
    container(
        container(text(value).size(16))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::FillPortion(2))
}

// In create_input_container function:
fn create_input_container<'a>(
    input_value: &str,
    on_input: fn(String) -> Message,
) -> container::Container<'a, Message> {
    container(
        container(
            text_input("", input_value)
                .style(text_input_style())
                .on_input(on_input) // Changed this line
                .align_x(Horizontal::Center),
        )
        .align_x(Horizontal::Center)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::FillPortion(2))
}

fn create_button_container<'a>(on_press: Message) -> container::Container<'a, Message> {
    container(
        container(button("Set").on_press(on_press))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::FillPortion(2))
}

fn create_tooltip<'a>(tooltip_text: &'a str) -> container::Container<'a, Message> {
    container(
        container(tooltip(
            text("ⓘ")
                .size(16)
                .style(hint_text_style())
                .shaping(text::Shaping::Advanced),
            container(text(tooltip_text).size(12))
                .style(card_style())
                .padding(10),
            tooltip::Position::Top,
        ))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill),
    )
}
