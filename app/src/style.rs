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

pub fn style_input_title(title: &str) -> Text {
    Text::new(title)
        .width(Length::Units(150))
        .horizontal_alignment(HorizontalAlignment::Left)
        .size(25)
}

pub fn style_chain_text(chain_id: &str) -> Text {
    Text::new(chain_id)
        .width(Length::Units(200))
        .horizontal_alignment(HorizontalAlignment::Left)
        .size(25)
}

pub fn button_icon(title: &str, width: u16) -> Text {
    Text::new(title)
        .width(Length::Units(width))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}
