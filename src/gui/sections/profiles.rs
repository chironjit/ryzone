use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, checkbox, column, container, row, scrollable, text, text_input, tooltip,
};
use iced::{Element, Length};

use crate::gui::style::{
    card_style, primary_button_style, secondary_button_style, stat_tip_style, tab_content_style,
    text_input_style,
};
use crate::model::state::ActiveSection;
use crate::model::State;
use crate::updates::Message;

pub fn view(state: &State) -> Element<Message> {
    container(if state.active_section != ActiveSection::Profiles {
        create_profile_enabler_overlay(state)
    } else {
        scrollable(
            column![
                create_header(),
                column![create_battery_profile(state), create_power_profile(state),].spacing(10)
            ]
            .padding(5),
        )
        .spacing(10)
        .height(Length::Fill)
        .into()
    })
    .style(tab_content_style())
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn create_profile_enabler_overlay(state: &State) -> Element<Message> {
    container(
        column![checkbox(
            "Enable Battery & Power Profiles",
            state.active_section == ActiveSection::Profiles,
        )
        .on_toggle(|_| Message::SetActiveSection(ActiveSection::Profiles))]
        .align_x(Horizontal::Center),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}

fn create_header<'a>() -> Element<'a, Message> {
    container(column![
        // Header Section
        container(
            column![row![
                text("Profile")
                    .size(10)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::FillPortion(2)),
                text("Fast Limit")
                    .size(10)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::FillPortion(1)),
                text("Slow Limit")
                    .size(10)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::FillPortion(1)),
                text("STAPM Limit")
                    .size(10)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::FillPortion(1)),
                text("Temp Limit")
                    .size(10)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::FillPortion(1)),
                text("")
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::FillPortion(1)),
            ]
            .spacing(10),]
            .spacing(10)
        )
    ])
    .padding([0, 20])
    .width(Length::Fill)
    .into()
}

fn create_battery_profile(state: &State) -> Element<Message> {
    container(column![
        // Battery Section
        container(
            column![
                row![
                    text("Battery")
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(1)),
                    text("")
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(1)),
                    text_input(
                        &format!(
                            "{}",
                            if state.batt_fast_limit == 0 {
                                15000
                            } else {
                                state.batt_fast_limit
                            }
                        ),
                        &state.batt_fast_input
                    )
                    .on_input(Message::BattFastInputChanged)
                    .align_x(Horizontal::Center)
                    .style(text_input_style())
                    .width(Length::FillPortion(1)),
                    text_input(
                        &format!(
                            "{}",
                            if state.batt_slow_limit == 0 {
                                10000
                            } else {
                                state.batt_slow_limit
                            }
                        ),
                        &state.batt_slow_input
                    )
                    .on_input(Message::BattSlowInputChanged)
                    .align_x(Horizontal::Center)
                    .style(text_input_style())
                    .width(Length::FillPortion(1)),
                    text_input(
                        &format!(
                            "{}",
                            if state.batt_stapm_limit == 0 {
                                15000
                            } else {
                                state.batt_stapm_limit
                            }
                        ),
                        &state.batt_stapm_input
                    )
                    .on_input(Message::BattStapmInputChanged)
                    .align_x(Horizontal::Center)
                    .style(text_input_style())
                    .width(Length::FillPortion(1)),
                    text_input(
                        &format!(
                            "{}",
                            if state.batt_tctl_limit == 0 {
                                70
                            } else {
                                state.batt_tctl_limit
                            }
                        ),
                        &state.batt_tctl_input
                    )
                    .on_input(Message::BattTctlInputChanged)
                    .align_x(Horizontal::Center)
                    .style(text_input_style())
                    .width(Length::FillPortion(1)),
                    button(
                        text("Set")
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                    )
                    .style(primary_button_style())
                    .on_press(Message::SetBattProfile())
                    .width(Length::FillPortion(1))
                ]
                .spacing(10),
                // Low Battery / Saver Section
                row![
                    checkbox(
                        "", // Empty string for checkbox
                        state.enable_saver_profile
                    )
                    .on_toggle(|_| Message::ToggleSaverProfile),
                    text("Enable Low Battery (Saver) Profile")
                        .size(10)
                        .align_y(Vertical::Center)
                ]
                .align_y(Vertical::Center),
                // Saver profile
                if state.enable_saver_profile {
                    row![
                        text("Saver")
                            .align_x(Horizontal::Center)
                            .width(Length::FillPortion(1)),
                        text_input(
                            &format!(
                                "{}",
                                if state.saver_threshold == 0 {
                                    20
                                } else {
                                    state.saver_threshold
                                }
                            ),
                            &state.saver_threshold_input
                        )
                        .on_input(Message::SaverThresholdInputChanged)
                        .align_x(Horizontal::Center)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),
                        text_input(
                            &format!(
                                "{}",
                                if state.saver_fast_limit == 0 {
                                    10000
                                } else {
                                    state.saver_fast_limit
                                }
                            ),
                            &state.saver_fast_input
                        )
                        .on_input(Message::SaverFastInputChanged)
                        .align_x(Horizontal::Center)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),
                        text_input(
                            &format!(
                                "{}",
                                if state.saver_slow_limit == 0 {
                                    8000
                                } else {
                                    state.saver_slow_limit
                                }
                            ),
                            &state.saver_slow_input
                        )
                        .on_input(Message::SaverSlowInputChanged)
                        .align_x(Horizontal::Center)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),
                        text_input(
                            &format!(
                                "{}",
                                if state.saver_stapm_limit == 0 {
                                    10000
                                } else {
                                    state.saver_stapm_limit
                                }
                            ),
                            &state.saver_stapm_input
                        )
                        .on_input(Message::SaverStapmInputChanged)
                        .align_x(Horizontal::Center)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),
                        text_input(
                            &format!(
                                "{}",
                                if state.saver_tctl_limit == 0 {
                                    60
                                } else {
                                    state.saver_tctl_limit
                                }
                            ),
                            &state.saver_tctl_input
                        )
                        .on_input(Message::SaverTctlInputChanged)
                        .align_x(Horizontal::Center)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),
                        button(
                            text("Set")
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                        )
                        .style(primary_button_style())
                        .on_press(Message::SetSaverProfile())
                        .width(Length::FillPortion(1))
                    ]
                    .align_y(Vertical::Center)
                    .spacing(10)
                } else {
                    row![].spacing(0)
                }
            ]
            .spacing(10)
        )
    ])
    .style(card_style())
    .padding(20)
    .width(Length::Fill)
    .into()
}

fn create_power_profile(state: &State) -> Element<Message> {
    container(column![
        // Power profile section
        container(
            column![
                row![
                    text("Power")
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(1)),
                    text("")
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(1)),
                    text_input(
                        &format!(
                            "{}",
                            if state.power_fast_limit == 0 {
                                35000
                            } else {
                                state.power_fast_limit
                            }
                        ),
                        &state.power_fast_input
                    )
                    .on_input(Message::PowerFastInputChanged)
                    .align_x(Horizontal::Center)
                    .style(text_input_style())
                    .width(Length::FillPortion(1)),
                    text_input(
                        &format!(
                            "{}",
                            if state.power_slow_limit == 0 {
                                30000
                            } else {
                                state.power_slow_limit
                            }
                        ),
                        &state.power_slow_input
                    )
                    .on_input(Message::PowerSlowInputChanged)
                    .align_x(Horizontal::Center)
                    .style(text_input_style())
                    .width(Length::FillPortion(1)),
                    text_input(
                        &format!(
                            "{}",
                            if state.power_stapm_limit == 0 {
                                35000
                            } else {
                                state.power_stapm_limit
                            }
                        ),
                        &state.power_stapm_input
                    )
                    .on_input(Message::PowerStapmInputChanged)
                    .align_x(Horizontal::Center)
                    .style(text_input_style())
                    .width(Length::FillPortion(1)),
                    text_input(
                        &format!(
                            "{}",
                            if state.power_tctl_limit == 0 {
                                90
                            } else {
                                state.power_tctl_limit
                            }
                        ),
                        &state.power_tctl_input
                    )
                    .on_input(Message::PowerTctlInputChanged)
                    .align_x(Horizontal::Center)
                    .style(text_input_style())
                    .width(Length::FillPortion(1)),
                    button(
                        text("Set")
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                    )
                    .style(primary_button_style())
                    .on_press(Message::SetPowerProfile())
                    .width(Length::FillPortion(1))
                ]
                .spacing(10),
                row![row![
                    checkbox(
                        "", // Empty string for checkbox
                        state.enable_turbo
                    )
                    .on_toggle(|_| Message::ToggleTurboOption),
                    text("Enable Turbo Profile")
                        .size(10)
                        .align_y(Vertical::Center)
                ]
                .align_y(Vertical::Center),]
                .spacing(20),
                if state.enable_turbo {
                    container(
                        row![
                            button(
                                text("Enable Turbo")
                                    .align_x(Horizontal::Center)
                                    .align_y(Vertical::Center)
                            )
                            .style(primary_button_style())
                            .on_press(Message::EnableTurbo)
                            .width(Length::Fixed(200.0)),
                            button(
                                text("Disable Turbo")
                                    .align_x(Horizontal::Center)
                                    .align_y(Vertical::Center)
                            )
                            .style(secondary_button_style())
                            .on_press(Message::DisableTurbo)
                            .width(Length::Fixed(200.0)),
                        ]
                        .spacing(20),
                    )
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                } else {
                    container(row![].spacing(0))
                }
            ]
            .spacing(10)
        )
    ])
    .style(card_style())
    .padding(20)
    .width(Length::Fill)
    .into()
}
