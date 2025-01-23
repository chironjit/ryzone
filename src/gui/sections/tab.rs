use iced::widget::{container, row, column, text, Space, button};
use iced::{Element, Length};
use iced::alignment::{Horizontal, Vertical};

use crate::model::{State, Tab};
use crate::update::Message;
use crate::gui::styles::styles::tab_style;

pub fn view(state: &State) -> Element<Message> {
    row![
        button(
            text("Profiles")
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
        )
        .width(Length::Fill)
        .height(Length::Fixed(40.0))
        .style(tab_style(state.active_tab == Tab::Profiles))
        .on_press(Message::TabSelected(Tab::Profiles)),
        

        button(
            text("Custom Overrides")
                .size(16)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
        )
        .width(Length::Fill)
        .height(Length::Fixed(40.0))
        .style(tab_style(state.active_tab == Tab::Overrides))
        .on_press(Message::TabSelected(Tab::Overrides)),


        button(
            text("Settings")
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
        )
        .width(Length::Fill)
        .height(Length::Fixed(40.0))
        .style(tab_style(state.active_tab == Tab::Settings))
        .on_press(Message::TabSelected(Tab::Settings))

    ]
    .padding([10, 0]) // Padding only for top and bottom
    .width(Length::Fill)
    .into()
 }