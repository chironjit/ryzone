use iced::widget::column;
use iced::{Element, Length};

use crate::model::{State, Tab};
use crate::update::Message;
use crate::views::{top, tab, custom_overrides, profiles, settings};

pub fn view(state: &State) -> Element<Message> {
    column![
        top::view(state),
        tab::view(state),
        match state.active_tab {
            Tab::Profiles => profiles::view(state),
            Tab::Overrides => custom_overrides::view(state),
            Tab::Settings => settings::view(state)
        }
        
    ]
    .spacing(10)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}