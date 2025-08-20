use iced::widget::container::Style;
use iced::widget::{button, text, text_input};
use iced::{border, Background, Border, Color, Shadow, Theme, Vector};

pub fn header_style() -> impl Fn(&Theme) -> Style {
    |theme| Style {
        text_color: None,
        background: Some(Background::Color(
            theme.extended_palette().background.base.color,
        )),
        border: Border {
            radius: border::Radius::new(12.0),
            width: 1.0,
            color: theme.extended_palette().background.strong.color,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 20.0,
        },
    }
}

pub fn card_style() -> impl Fn(&Theme) -> Style {
    |theme| Style {
        text_color: None,
        background: Some(Background::Color(
            theme.extended_palette().background.base.color,
        )),
        border: Border {
            radius: border::Radius::new(20.0),
            width: 1.0,
            color: theme.extended_palette().background.strong.color,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 25.0,
        },
    }
}

pub fn stat_tip_style() -> impl Fn(&Theme) -> Style {
    |theme| Style {
        text_color: None,
        background: Some(Background::Color(
            theme.extended_palette().background.weak.color,
        )),
        border: Border {
            radius: border::Radius::new(16.0),
            width: 1.0,
            color: theme.extended_palette().background.strong.color,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 15.0,
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

        if selected {
            button::Style {
                background: Some(Background::Color(palette.primary.base.color)),
                border: Border {
                    radius: border::Radius::new(12.0),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                text_color: Color::WHITE,
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                ..button::Style::default()
            }
        } else {
            let bg_color = match status {
                button::Status::Hovered => palette.background.strong.color,
                _ => palette.background.weak.color,
            };
            
            button::Style {
                background: Some(Background::Color(bg_color)),
                border: Border {
                    radius: border::Radius::new(12.0),
                    width: 1.0,
                    color: palette.background.strong.color,
                },
                text_color: palette.primary.base.color,
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 4.0,
                },
                ..button::Style::default()
            }
        }
    }
}

pub fn metrics_card_style() -> impl Fn(&Theme) -> Style {
    |theme| Style {
        text_color: None,
        background: Some(Background::Color(
            theme.extended_palette().background.base.color,
        )),
        border: Border {
            radius: border::Radius::new(24.0),
            width: 1.0,
            color: theme.extended_palette().background.strong.color,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 32.0,
        },
    }
}

pub fn primary_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    |theme, status| {
        let palette = theme.extended_palette();
        
        let (bg_color, shadow_opacity) = match status {
            button::Status::Hovered => (palette.primary.strong.color, 0.25),
            button::Status::Pressed => (palette.primary.weak.color, 0.15),
            _ => (palette.primary.base.color, 0.2),
        };

        button::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                radius: border::Radius::new(12.0),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::WHITE,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, shadow_opacity),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..button::Style::default()
        }
    }
}

pub fn secondary_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    |theme, status| {
        let palette = theme.extended_palette();
        
        let (bg_color, border_color) = match status {
            button::Status::Hovered => (palette.background.strong.color, palette.primary.base.color),
            button::Status::Pressed => (palette.background.weak.color, palette.primary.strong.color),
            _ => (palette.background.weak.color, palette.background.strong.color),
        };

        button::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                radius: border::Radius::new(12.0),
                width: 2.0,
                color: border_color,
            },
            text_color: palette.primary.base.color,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..button::Style::default()
        }
    }
}
