use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, row, text};
use iced::{Element, Length};

use crate::gui::style::tab_style;
use crate::model::{State, Tab};
use crate::updates::Message;

pub fn view(state: &State) -> Element<Message> {
    row![
        button(
            text("Profiles")
                .size(16)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
        )
        .width(Length::Fill)
        .height(Length::Fixed(48.0))
        .style(tab_style(state.active_tab == Tab::Profiles))
        .on_press(Message::TabSelected(Tab::Profiles)),
        button(
            text("Custom Overrides")
                .size(16)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
        )
        .width(Length::Fill)
        .height(Length::Fixed(48.0))
        .style(tab_style(state.active_tab == Tab::Overrides))
        .on_press(Message::TabSelected(Tab::Overrides)),
        button(
            text("Settings")
                .size(16)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
        )
        .width(Length::Fill)
        .height(Length::Fixed(48.0))
        .style(tab_style(state.active_tab == Tab::Settings))
        .on_press(Message::TabSelected(Tab::Settings))
    ]
    .spacing(12)
    .padding([16, 0])
    .width(Length::Fill)
    .into()
}
