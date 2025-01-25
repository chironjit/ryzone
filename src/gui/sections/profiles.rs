use iced::widget::{container, row, column, text, text_input, button, checkbox, tooltip, scrollable};
use iced::alignment::{Horizontal, Vertical};
use iced::{Element, Length};

use crate::model::State;
use crate::update::Message;
use crate::gui::styles::styles::{card_style, header_style, hint_text_style, stat_tip_style, text_input_style};

pub fn view(state: &State) -> Element<Message> {
    column![
        create_current_profile(state),
        create_header(),
        scrollable(
            column![
                create_battery_profile(state),
                create_power_profile(state),
            ]
            .spacing(10)
        )
        .spacing(10)
        .height(Length::Fill)
        
    ]
    .spacing(10)
    .padding(10)
    .into()
    
}

fn create_current_profile(state: &State) -> Element<Message> {
    container(
        row![

            // Current fast limit
            container(
                column![
                    text("30 W")
                        .size(20)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("Fast Limit")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                ].align_x(Horizontal::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 20])
            .width(Length::Fill)
            .height(Length::Fixed(60.0)),

            // Current Slow limit
            container(
                column![
                    text("30 W")
                        .size(20)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("Slow Limit")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                ].align_x(Horizontal::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 20])
            .width(Length::Fill)
            .height(Length::Fixed(60.0)),

            // Current Stapm limit
            container(
                column![
                    text("30 W")
                        .size(20)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("STAPM Limit")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                ].align_x(Horizontal::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 20])
            .width(Length::Fill)
            .height(Length::Fixed(60.0)),

            // Current power status
            container(
                column![
                    text("30 °C")
                        .size(20)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("Temp Limit")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                ].align_x(Horizontal::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 20])
            .width(Length::Fill)
            .height(Length::Fixed(60.0)),

            // Current power source
            container(
                column![
                    text("🔋") // 🔌
                        .shaping(text::Shaping::Advanced)
                        .size(20)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("Power Source")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                ].align_x(Horizontal::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 20])
            .width(Length::Fill)
            .height(Length::Fixed(60.0)),

            // Current power profile
            container(
                column![
                    text("Saver") // Batt / Power / Custom / Saver / Turbo
                        .size(20)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("Power Profile")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                ].align_x(Horizontal::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 20])
            .width(Length::Fill)
            .height(Length::Fixed(60.0)),
        ]
        .spacing(10)
    ).into()
}

fn create_header<'a>() -> Element<'a, Message>  {
    container(
        column![
            // Header Section
            container(
                column![
                    row![
                        text("Battery Profile")
                        .align_x(Horizontal::Left)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(2)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("°C", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        button(
                            text("Set")
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                        )
                        .width(Length::FillPortion(1))
                    ]
                    .spacing(10),
                ].spacing(10)
            )
        ]
    )
    .style(card_style())
    .padding(20)
    .width(Length::Fill)
    .into()
    
 }

fn create_battery_profile(state: &State) -> Element<Message> {
    container(
        column![
            // Battery Section
            container(
                column![
                    row![
                        text("Battery Profile")
                        .align_x(Horizontal::Left)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(2)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("°C", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        button(
                            text("Set")
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                        )
                        .width(Length::FillPortion(1))
                    ]
                    .spacing(10),
                    // Low Battery / Saver Section
                    row![
                        row![
                            checkbox(
                                "",  // Empty string for checkbox
                                true
                            ),
                            text("Enable Low Battery (Saver) Profile")
                                .size(10)
                                .align_y(Vertical::Center)
                        ]
                        .align_y(Vertical::Center),
                        text_input(
                            "Threshold %",
                            ""
                        )
                        .style(text_input_style())
                        .width(Length::Fixed(100.0))
                    ].spacing(20),

                    // Saver profile
                    row![
                        text("Saver Profile")
                        .align_x(Horizontal::Left)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(2)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("°C", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        button(
                            text("Set")
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                        )
                        .width(Length::FillPortion(1))
                    ]
                    .spacing(10),
                ].spacing(10)
            )
        ]
    )
    .style(card_style())
    .padding(20)
    .width(Length::Fill)
    .into()
    
 }

fn create_power_profile(state: &State) -> Element<Message> {
    container(
        column![
           // Power profile section
            container(
                column![
                    row![
                        text("Power Profile")
                        .align_x(Horizontal::Left)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(2)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("mW", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("°C", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        button(
                            text("Set")
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                        )
                        .width(Length::FillPortion(1))
                    ]
                    .spacing(10),
                    row![
                        row![
                            checkbox(
                                "",  // Empty string for checkbox
                                true
                            ),
                            text("Enable Turbo Profile")
                                .size(10)
                                .align_y(Vertical::Center)
                        ]
                        .align_y(Vertical::Center),
                    ].spacing(20),
                    container(
                        row![
                            button(
                                text("Enable Turbo")
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                            )
                            .width(Length::Fixed(200.0)),
                            button(
                                text("Disable Turbo")
                                .align_x(Horizontal::Center)
                                .align_y(Vertical::Center)
                            )
                            .width(Length::Fixed(200.0)),
                        ]
                        .spacing(20)
                    )
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                ].spacing(10)
            )
        ]
    )
    .style(card_style())
    .padding(20)
    .width(Length::Fill)
    .into()
    
 }

 fn create_tooltip<'a>(tooltip_text: &'a str) -> container::Container<'a, Message> {
    container(
        container(
            tooltip(
                text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                container(text(tooltip_text).size(12))
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
 }



// pub fn view(state: &State) -> Element<Message> {
//     column![
//         // Battery Profile Section
//         container(
//             column![
//                 text("Battery Profile").size(20),
//                 create_power_limits_row(),
//                 // Low Battery Section
//                 container(
//                     column![
//                         row![
//                             checkbox(
//                                 "Enable Low Battery Profile",
//                                 true),
//                             text_input(
//                                 "Threshold %",
//                                 "20"
//                             )
//                             .style(text_input_style())
//                             .width(Length::Fixed(100.0))
//                         ].spacing(20),
//                         create_power_limits_row()
//                     ].spacing(10)
//                 )
//                 // .style(section_style())
//                 .padding(10)
//             ]
//         )
//         .style(card_style())
//         .padding(20)
//         .width(Length::Fill),

//         // AC Power Profile Section
//         container(
//             column![
//                 row![
//                     text("AC Power Profile").size(20),
//                     button("Turbo Mode")
//                         // .on_press()
//                         .width(Length::Fixed(120.0))
//                 ].spacing(20),
//                 create_power_limits_row()
//             ]
//         )
//         .style(card_style())
//         .padding(20)
//         .width(Length::Fill)
//     ]
//     .spacing(20)
//     .padding(20)
//     .into()
// }

// fn create_power_limits_row<'a>() -> Element<'a, Message> {
//     column![
//         create_limit_row("Fast Limit", "45", "W"),
//         create_limit_row("Slow Limit", "42", "W"),
//         create_limit_row("STAPM Limit", "40", "W"),
//         create_limit_row("Temperature Limit", "95", "°C")
//     ]
//     .spacing(10)
//     .into()
// }

// fn create_limit_row<'a>(
//     label: &'a str,
//     value: &'a str,
//     unit: &'a str,
// ) -> Element<'a, Message> {
//     row![
//         container(text(label).size(16))
//             .width(Length::FillPortion(2)),
//         text_input("", value)
//             .style(text_input_style())
//             .width(Length::Fixed(100.0)),
//         container(text(unit).size(16))
//             .width(Length::Fixed(30.0)),
//         button("Set")
//             .width(Length::Fixed(60.0))
//     ]
//     .spacing(10)
//     .align_y(Vertical::Center)
//     .into()
// }