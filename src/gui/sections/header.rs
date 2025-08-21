use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length};

use crate::gui::style::header_style;
use crate::model::State;
use crate::updates::Message;

pub fn view(state: &State) -> Element<Message> {
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
    .width(Length::Fill)
    .into()
}