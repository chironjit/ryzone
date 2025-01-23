use iced::widget::column;
use iced::{Element, Length};

use crate::model::State;
use crate::update::Message;
use crate::views::{top, tab, custom_overrides};

pub fn view(state: &State) -> Element<Message> {
    column![
        top::view(state),
        tab::view(state),
        custom_overrides::view(state)
    ]
    .spacing(10)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}