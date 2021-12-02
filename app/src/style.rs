use iced::{button, Background, Color, Font, HorizontalAlignment, Length, Text, Vector};

pub enum Button {
    Filter { selected: bool },
    Icon,
    Destructive,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Filter { selected } => {
                if *selected {
                    button::Style {
                        background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
                        border_radius: 10.0,
                        text_color: Color::WHITE,
                        ..button::Style::default()
                    }
                } else {
                    button::Style::default()
                }
            }
            Button::Icon => button::Style {
                text_color: Color::from_rgb(0.5, 0.5, 0.5),
                ..button::Style::default()
            },
            Button::Destructive => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                border_radius: 5.0,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 1.0),
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            text_color: match self {
                Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
                Button::Filter { selected } if !selected => Color::from_rgb(0.2, 0.2, 0.7),
                _ => active.text_color,
            },
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }
}

const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

pub fn style_chain_text(chain_id: &str) -> Text {
    Text::new(chain_id)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Left)
        .size(20)
}

pub fn style_detail_button() -> Text {
    Text::new("detail")
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

/// active button icon
pub fn button_active_icon() -> Text {
    icon('\u{8646}')
}

/// ✄
pub fn button_disconnect_icon() -> Text {
    icon('\u{2704}')
}

/// refresh icon ⟳
pub fn button_refresh_icon() -> Text {
    icon('\u{10227}')
}

/// ✓
pub fn status_active_icon() -> Text {
    icon('\u{2713}')
}

/// ⚠
pub fn status_disconnected_icon() -> Text {
    icon('\u{26A0}')
}
