use iced::widget::{text, text_input, button};
use iced::widget::container::Style;
use iced::{Theme, Border, Color, Shadow, Background, border, Vector};

pub fn header_style() -> impl Fn(&Theme) -> Style {
    |theme| Style {
        text_color: None,
        background: Some(Background::Color(theme.extended_palette().background.weak.color)),
        border: Border {
            radius: border::Radius::new(4.0),
            width: 0.0, 
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 12.0,
        },
    }
}

pub fn card_style() -> impl Fn(&Theme) -> Style {
    |theme| Style {
        text_color: None,
        background: Some(Background::Color(theme.extended_palette().background.weak.color)),
        border: Border {
            radius: border::Radius::new(16.0),
            width: 0.0, 
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 12.0,
        },
    }
}

pub fn hint_text_style() -> impl Fn(&Theme) -> text::Style {
    |theme| text::Style {
        color: Some(theme.extended_palette().primary.strong.color),
        ..text::Style::default()
    }
}

pub fn text_input_style() -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    |theme, status| text_input::Style {
        background: Background::Color(theme.extended_palette().background.base.color),
        border: Border {
            radius: border::Radius::new(8.0),
            width: 1.0,
            color: match status {
                text_input::Status::Focused => theme.extended_palette().primary.base.color,
                _ => theme.extended_palette().secondary.weak.color,
            },
        },
        icon: theme.extended_palette().background.strong.color,
        placeholder: theme.extended_palette().background.strong.color,
        value: theme.extended_palette().primary.base.color,
        selection: theme.extended_palette().primary.weak.color,
    }
}

pub fn tab_style(selected: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| {
        let palette = theme.extended_palette();
        
        button::Style {
            background: if selected {
                Some(Background::Color(palette.primary.base.color))
            } else {
                Some(Background::Color(palette.background.weak.color))
            },
            border: Border {
                radius: border::Radius::new(0.),
                width: 1.0,
                color: palette.primary.base.color,
            },
            text_color: if selected {
                Color::WHITE
            } else {
                palette.primary.base.color
            },
            ..button::Style::default()
        }
    }
 }