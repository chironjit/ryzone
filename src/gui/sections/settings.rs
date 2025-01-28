use iced::widget::{container, row, column, text, text_input, button, tooltip};
use iced::alignment::{Horizontal, Vertical};
use iced::{Element, Length};

use crate::model::State;
use crate::updates::Message;
use crate::gui::style::{card_style, hint_text_style, text_input_style};

pub fn view(state: &State) -> Element<Message> {
   column![
       text("Just a placeholder")
   ]
   .spacing(10)
   .into()
}
