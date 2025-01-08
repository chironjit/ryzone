use iced::widget::{button, column, row, text, text_input, container, Space, tooltip}; 
use iced::alignment::{self, Horizontal, Vertical};
use iced::{Theme, Length, Element, Border, Color, Shadow, Background, border, Vector};
use iced::widget::container::Style;

use crate::model::State;
use crate::update::Message;

fn format_frequency(freq: u32) -> String {
    if freq < 1000 {  // Less than 1000 MHz (1 GHz)
        format!("{} MHz", freq)
    } else {
        format!("{:.2} GHz", freq as f32 / 1000.0)
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
pub fn view(state: &State) -> Element<Message> {
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
                    // Top section with title and main speed reading
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
            
                    // Bottom section with minor details
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
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(140.0)),

            // GPU container
            container(
                column![
                    // Top section with title and main speed reading
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
            
                    // Bottom section with minor details
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
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(140.0)),

            // Power container
            container(
                column![
                    // Top section with title and main power reading
                    row![
                        column![
                            text("Power").size(20),
                            text("APU Total").size(10),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text("15.1 W").size(28),
                        ]
                    ]
                    .align_y(alignment::Vertical::Center)
                    .spacing(2),
            
                    Space::with_height(8),
            
                    // Bottom section with power details
                    row![
                        column![
                            text("System total").size(12),
                            text("Source (battery)").size(12),
                            text("Source (external)").size(12),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text("20.0 W").size(12),
                            text("20.0 W").size(12),
                            text("00.0 W").size(12),
                        ]
                        .spacing(2)
                    ]
                ]
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(140.0))
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
                    container(text(format!("{:.1} W", state.curr_stapm_value as f32 / 1000.0)).size(16))
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