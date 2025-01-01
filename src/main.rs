use iced::widget::{button, column, row, text, text_input, Row, container, Space, tooltip}; 
use iced::{theme, Subscription, Theme};
use iced::time::{self, Duration};
use libryzenadj::RyzenAdj;
use iced::{Length, Fill, Element, window};
use iced::alignment::{self, Horizontal, Vertical};
use iced::{Size}; 
use iced::widget::container::{Style};
use iced::{Border, Color, Shadow, Background, border, Vector, color};

// Constants to limit input values
const FAST_LIMIT_MIN: u32 = 4000;
const FAST_LIMIT_MAX: u32 = 65000;
const SLOW_LIMIT_MIN: u32 = 4000;
const SLOW_LIMIT_MAX: u32 = 65000;
const STAPM_LIMIT_MIN: u32 = 4000;
const STAPM_LIMIT_MAX: u32 = 65000;
const TCTL_LIMIT_MIN: u32 = 40;
const TCTL_LIMIT_MAX: u32 = 100;

#[derive(Debug, Clone)]
pub enum Message {
    SetFastLimit(u32),
    SetSlowLimit(u32),
    SetStapmLimit(u32),
    SetTctlLimit(u32),
    FastLimitInputChanged(String),
    SlowLimitInputChanged(String),
    StapmLimitInputChanged(String),
    TctlLimitInputChanged(String),
    UpdateStateValues,
}

// All the states managed by the app
#[derive(Default, Debug, Clone)]
struct State {
    curr_fast_value: u32,
    curr_slow_value: u32,
    curr_stapm_value: u32,
    curr_tctl_value: u32,
    curr_fast_limit: u32,
    curr_slow_limit: u32,
    curr_stapm_limit: u32,
    curr_tctl_limit: u32,
    fast_input: String,
    slow_input: String,
    stapm_input: String,
    tctl_input: String,
    manual_fast_limit: u32,
    manual_slow_limit: u32,
    manual_stapm_limit: u32,
    manual_tctl_limit: u32,
}

// Standalone update function
fn update(
    state: &mut State,
    message: Message
) {
    match message {
        Message::SetFastLimit(value) => {
            if value >= FAST_LIMIT_MIN && value <= FAST_LIMIT_MAX {
                state.manual_fast_limit = value.into();
            }
        }
        Message::SetSlowLimit(value) => {
            if value >= SLOW_LIMIT_MIN && value <= SLOW_LIMIT_MAX {
                state.manual_slow_limit = value.into();
            }
        }
        Message::SetStapmLimit(value) => {
            if value >= STAPM_LIMIT_MIN && value <= STAPM_LIMIT_MAX {
                state.manual_stapm_limit = value.into();
            }
        }
        Message::SetTctlLimit(value) => {
            if value >= TCTL_LIMIT_MIN && value <= TCTL_LIMIT_MAX {
                state.manual_tctl_limit = value.into();
            }
        }
        Message::FastLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u32>() {
                    if num <= FAST_LIMIT_MAX {
                        state.fast_input = value;
                    }
                }
            }
        }
        Message::SlowLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u32>() {
                    if num <= SLOW_LIMIT_MAX {
                        state.slow_input = value;
                    }
                }
            }
        }
        Message::StapmLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u32>() {
                    if num <= STAPM_LIMIT_MAX {
                        state.stapm_input = value;
                    }
                }
            }
        }
        Message::TctlLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u32>() {
                    if num <= TCTL_LIMIT_MAX {
                        state.tctl_input = value;
                    }
                }
            }
        }
        Message::UpdateStateValues => {
            // Every second, check what the actual values are
            // If values don't match the override
            // (and the manual values are not 0), 
            // try to apply the override value
            // Then check the latest figures and then update the state

            let ryzen = RyzenAdj::new().unwrap();


            state.curr_fast_limit = (ryzen.get_fast_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_fast_value = (ryzen.get_fast_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_slow_limit = (ryzen.get_slow_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_slow_value = (ryzen.get_slow_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_stapm_limit = (ryzen.get_stapm_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_stapm_value = (ryzen.get_stapm_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_tctl_limit = ryzen.get_tctl_temp().unwrap_or_default().round() as u32;
            state.curr_tctl_value = ryzen.get_tctl_temp_value().unwrap_or_default().round() as u32;

            if state.manual_fast_limit != 0 && state.curr_fast_limit != state.manual_fast_limit {
                let _ = ryzen.set_fast_limit(state.manual_fast_limit);
            }

            if state.manual_slow_limit != 0 && state.curr_slow_limit != state.manual_slow_limit {
                let _ = ryzen.set_slow_limit(state.manual_slow_limit);
            }

            if state.manual_stapm_limit != 0 && state.curr_stapm_limit != state.manual_stapm_limit {
                let _ = ryzen.set_stapm_limit(state.manual_stapm_limit);
            }

            if state.manual_tctl_limit != 0 && state.curr_tctl_limit != state.manual_tctl_limit {
                let _ = ryzen.set_tctl_temp(state.manual_tctl_limit);
            }
        }
    }
}



fn card_style() -> impl Fn(&Theme) -> Style {
    |theme| Style {
        text_color: None,
        background: Some(Background::Color(theme.extended_palette().background.weak.color)),
        border: Border {
            radius: border::Radius::new(16.0),
            width: 0.0, 
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 12.0,
        },
    }
}

fn hint_text_style() -> impl Fn(&Theme) -> text::Style {
    |theme| text::Style {
        color: Some(theme.extended_palette().primary.strong.color),
        ..text::Style::default()
    }
}

fn text_input_style() -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    |theme, status| text_input::Style {
        background: Background::Color(theme.extended_palette().background.base.color),
        border: Border {
            radius: border::Radius::new(8.0),
            width: 1.0,
            color: match status {
                text_input::Status::Focused => theme.extended_palette().primary.base.color,
                _ => theme.extended_palette().secondary.weak.color,
            },
        },
        icon: theme.extended_palette().background.strong.color,
        placeholder: theme.extended_palette().background.strong.color,
        value: theme.extended_palette().primary.base.color,
        selection: theme.extended_palette().primary.weak.color,
    }
}

// Standalone view function
fn view(state: &State) -> Element<Message> {
    column![
        // Top full-width container
        container("Ryzone - Adjust mobile Ryzen APU power limits")
            .align_x(alignment::Horizontal::Center)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fixed(50.0)),

        // Row of three equal containers
        row![
            // Processor container
            container(
                column![
                    text("Processor").size(20),
                    row![
                        column![
                            text("Current Speed").size(14),
                            text("3.8 GHz").size(16),
                        ],
                        Space::with_width(Length::Fill),
                        column![
                            text("Peak Speed").size(14),
                            text("4.2 GHz").size(16),
                        ],
                    ],
                ]
                .spacing(10)
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(120.0)),

            // GPU container
            container(
                column![
                    text("GPU").size(20),
                    row![
                        column![
                            text("Current Speed").size(14),
                            text("1.8 GHz").size(16),
                        ],
                        Space::with_width(Length::Fill),
                        column![
                            text("Peak Speed").size(14),
                            text("2.2 GHz").size(16),
                        ],
                    ],
                ]
                .spacing(10)
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(120.0)),

            // Power container
            container(
                column![
                    text("Power").size(20),
                    row![
                        column![
                            text("APU").size(14),
                            text("15W").size(16),
                            text("System").size(14),
                            text("25W").size(16),
                        ],
                        Space::with_width(Length::Fill),
                        column![
                            text("APU").size(14),
                            text("15W").size(16),
                            text("System").size(14),
                            text("25W").size(16),
                        ],
                        Space::with_width(Length::Fill),
                        column![
                            text("Battery").size(14),
                            text("8W").size(16),
                            text("Socket").size(14),
                            text("32W").size(16),
                        ],
                    ],
                ]
                .spacing(10)
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(150.0)),
            ]
            .spacing(20)
            .width(Length::Fill),

        // Column titles row
        container(
            row![
                container(
                    container(text("").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(4)),
                container(
                    container(text("Current").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(2)),
                container(
                    container(text("Limit").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(2)),
                container(
                    container(text("Set New Limit").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(2)),
                container(
                    container(text("").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(2)),
                container(
                    container(text("").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(1)),
            ]
            .spacing(20)
        )
        .style(container::transparent)
        .padding(10)
        .width(Length::Fill),

        // Fast Limit
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
                    container(text(format!("{:.1} W", state.curr_fast_value as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_fast_limit as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        text_input(
                            "",
                            &state.fast_input
                        )
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
                        button("Set").on_press(Message::SetFastLimit(
                            state.fast_input.parse().unwrap_or(0)
                        ))
                    )
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),

                container(
                    container(
                        tooltip(
                            text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                            container(
                                text("Enter value in miliWatts.\nAccepted value range\nbetween 4000 & 65000.").size(12)
                            )
                            .style(container::bordered_box)
                            .padding(10),
                            tooltip::Position::Top
                        )
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::FillPortion(1))
            ]
            .spacing(20)
        )
        .style(card_style())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(50.0)),

        // Slow Limit
        container(
            row![
                container(
                    container(text("Slow Limit").size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(4)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_slow_value as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_slow_limit as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        text_input(
                            "",
                            &state.slow_input
                        )
                            .style(text_input_style())
                            .on_input(Message::SlowLimitInputChanged)
                            .align_x(Horizontal::Center)
                    )
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        button("Set").on_press(Message::SetSlowLimit(
                            state.slow_input.parse().unwrap_or(0)
                        ))
                    )
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),

                container(
                    container(
                        tooltip(
                            text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                            container(
                                text("Enter value in miliWatts.\nAccepted value range\nbetween 4000 & 65000.\nSlow limit must be less\nthan or equal to fast limit.").size(12)
                            )
                            .style(card_style())
                            .padding(10),
                            tooltip::Position::Top
                        )
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::FillPortion(1))
            ]
            .spacing(20)
        )
        .style(card_style())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(50.0)),

        // STAPM Limit
        container(
            row![
                container(
                    container(text("STAPM Limit").size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(4)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_stapm_limit as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_stapm_limit as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        text_input(
                            "",
                            &state.stapm_input
                        )
                            .style(text_input_style())
                            .on_input(Message::StapmLimitInputChanged)
                            .align_x(Horizontal::Center)
                    )
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        button("Set").on_press(Message::SetStapmLimit(
                            state.stapm_input.parse().unwrap_or(0)
                        ))
                    )
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),

                container(
                    container(
                        tooltip(
                            text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                            container(
                                text("Enter value in miliWatts.\nAccepted value range\nbetween 4000 & 65000.\nStapm limit must be less\nthan or equal to slow limit.").size(12)
                            )
                            .style(card_style())
                            .padding(10),
                            tooltip::Position::Top
                        )
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::FillPortion(1))
            ]
            .spacing(20)
        )
        .style(card_style())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(50.0)),

        // TCtl Limit
        container(
            row![
                container(
                    container(text("TCTL Limit").size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(4)),
                
                container(
                    container(text(format!("{}°C", state.curr_tctl_value)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
                
                container(
                    container(text(format!("{}°C", state.curr_tctl_limit)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        text_input(
                            "",
                            &state.tctl_input
                        )
                            .style(text_input_style())
                            .on_input(Message::TctlLimitInputChanged)
                            .align_x(Horizontal::Center)
                    )
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        button("Set").on_press(Message::SetTctlLimit(
                            state.tctl_input.parse().unwrap_or(0)
                        ))
                    )
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),

                container(
                    container(
                        tooltip(
                            text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                            container(
                                text("Enter value in °C.\nAccepted value range\nbetween 40 and 100.").size(12)
                            )
                            .style(card_style())
                            .padding(10),
                            tooltip::Position::Top
                        )
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::FillPortion(1))
            ]
            .spacing(20)
        )
        .style(card_style())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(50.0)),

        // Bottom full-width container
        container("Notes and settings")
            .align_x(alignment::Horizontal::Center)
            .style(card_style())
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fixed(150.0)),

    ]
    .spacing(10)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn update_state_values(_: &State) -> Subscription<Message> {
    time::every(Duration::from_secs(1))
        .map(|_| Message::UpdateStateValues)
}


fn main() -> iced::Result {
    iced::application("Ryzone", update, view)
        .window(window::Settings{
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


