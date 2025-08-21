use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text, text_input, tooltip};
use iced::{Element, Length};

use crate::gui::style::{card_style, hint_text_style, text_input_style, tab_content_style};
use crate::model::State;
use crate::updates::Message;

pub fn view(state: &State) -> Element<Message> {
    let content = column![
                                        
        container(
            column![
                container(text("Settings").size(20))
                    .padding([0, 10])
                    .align_x(Horizontal::Left), 
                text("1. Use the Profiles section to set power profiles").size(12),
                text("2. Use the Custom Overrides section to set custom power profiles").size(12),
                text("3. Enter power limits in miliWatts(mW)").size(12),
                text("4. Enter temp limits in Celsius(°C)").size(12),
            ]
            .spacing(10)
        )
        .style(card_style())
        .width(Length::Fill)
        .padding(10)
    ]
    .spacing(20)
    .padding(10)
    .width(Length::Fill);

    container(content)
        .style(tab_content_style())
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .into()
}
