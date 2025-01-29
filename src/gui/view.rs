use iced::widget::column;
use iced::{Element, Length};

use crate::gui::sections::{custom, profiles, settings, tab, top};
use crate::model::state::{State, Tab};
use crate::updates::update::Message;

pub fn view(state: &State) -> Element<Message> {
    column![
        top::view(state),
        tab::view(state),
        match state.active_tab {
            Tab::Profiles => profiles::view(state),
            Tab::Overrides => custom::view(state),
            Tab::Settings => settings::view(state),
        }
    ]
    .spacing(10)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
