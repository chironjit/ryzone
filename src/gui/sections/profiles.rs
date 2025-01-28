use iced::widget::{container, row, column, text, text_input, button, checkbox, scrollable};
use iced::alignment::{Horizontal, Vertical};
use iced::{Element, Length};

use crate::model::State;
use crate::updates::Message;
use crate::gui::style::{card_style, stat_tip_style, text_input_style};

pub fn view(state: &State) -> Element<Message> {
    column![
        create_current_profile(state),

        scrollable(
            column![
                create_header(),
                column![
                    create_battery_profile(state),
                    create_power_profile(state),

                ]
                .spacing(10)
                
            ]
            .padding(5)
        )
        .spacing(10)
        .height(Length::Fill)
        
    ]
    .spacing(10)
    .into()
    
}

fn create_current_profile(state: &State) -> Element<Message> {
    container(
        row![

            // Current fast limit
            container(
                row![
                    text("Fast\nLimit")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("100 W")
                        .size(14)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    
                ].align_y(Vertical::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 10])
            .width(Length::Fill)
            .height(Length::Fixed(50.0)),

            // Current Slow limit
            container(
                row![
                    text("Slow\nLimit")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("100 W")
                        .size(14)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    
                ].align_y(Vertical::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 10])
            .width(Length::Fill)
            .height(Length::Fixed(50.0)),

            // Current Stapm limit
            container(
                row![
                    text("STAPM\nLimit")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("100 W")
                        .size(14)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    
                ].align_y(Vertical::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 10])
            .width(Length::Fill)
            .height(Length::Fixed(50.0)),

            // Current power status
            container(
                row![
                    text("Temp\nLimit")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("100°C")
                        .size(14)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    
                ].align_y(Vertical::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 10])
            .width(Length::Fill)
            .height(Length::Fixed(50.0)),

            // Current power source
            container(
                row![
                    text("Power\nSource")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("🔋") // 🔌
                        .shaping(text::Shaping::Advanced)
                        .size(14)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    
                ].align_y(Vertical::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 10])
            .width(Length::Fill)
            .height(Length::Fixed(50.0)),

            // Current power profile
            container(
                row![
                    text("Power\nProfile")
                        .size(10)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    text("OS") // OS / Bat (Batt) / Pow(Power) / Cus(Custom) / Sav (Saver) / Tur (Turbo)
                        .size(18)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    
                ].align_y(Vertical::Center)

            )
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center)
            .style(stat_tip_style())
            .padding([10, 10])
            .width(Length::Fill)
            .height(Length::Fixed(50.0)),
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
                    .spacing(10),
                ].spacing(10)
            )
        ]
    )
    .padding([0, 20])
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
                        text("Battery")
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(1)),

                        text("")
                        .align_x(Horizontal::Left)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(1)),

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
                            checkbox(
                                "",  // Empty string for checkbox
                                true
                            ),
                            text("Enable Low Battery (Saver) Profile")
                                .size(10)
                                .align_y(Vertical::Center)
                    ]
                    .align_y(Vertical::Center),

                    // Saver profile
                    row![
                        text("Saver")
                        .align_x(Horizontal::Center)
                        .width(Length::FillPortion(1)),

                        text_input("Batt %", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("100000", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("100000", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("100000", "")
                        .align_x(Horizontal::Left)
                        .style(text_input_style())
                        .width(Length::FillPortion(1)),

                        text_input("100", "")
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
                    .align_y(Vertical::Center)
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
                        text("Power")
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(1)),

                        text("")
                        .align_x(Horizontal::Left)
                        .align_y(Vertical::Center)
                        .width(Length::FillPortion(1)),

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