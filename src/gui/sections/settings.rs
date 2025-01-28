use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text, text_input, tooltip};
use iced::{Element, Length};

use crate::gui::style::{card_style, hint_text_style, text_input_style};
use crate::model::State;
use crate::updates::Message;

pub fn view(state: &State) -> Element<Message> {
    column![text("Just a placeholder")].spacing(10).into()
}
