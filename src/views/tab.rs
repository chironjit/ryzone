use iced::widget::{container, row, column, text, Space, button};
use iced::{Element, Length};

use crate::model::{State, Tab};
use crate::update::Message;
use crate::views::styles::styles::button_style;

pub fn view(state: &State) -> Element<Message> {
   row![
       button("Profiles")
           .width(Length::Fill)
           .height(Length::Fixed(40.0))
           .style(button_style(state.active_tab == Tab::Profiles)),
           
       button("Custom Overrides")
           .width(Length::Fill)
           .height(Length::Fixed(40.0))
           .style(button_style(state.active_tab == Tab::Overrides))
           .on_press(Message::TabSelected(Tab::Overrides)),
           
       button("Settings")
           .width(Length::Fill)
           .height(Length::Fixed(40.0))
           .style(button_style(state.active_tab == Tab::Settings))
           .on_press(Message::TabSelected(Tab::Settings))
   ]
   .spacing(10)
   .padding(10)
   .width(Length::Fill)
   .into()
}