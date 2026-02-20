use iced::{
    Background, Border, Color, Font, Theme,
    widget::{button, container, text_input},
};

pub const FONT_BYTES: &[u8] = include_bytes!("../assets/Courier New.ttf");
pub const FONT_BOLD_BYTES: &[u8] =
    include_bytes!("../assets/Courier New Bold.ttf");

pub const DEFAULT_FONT: Font = Font {
    family: iced::font::Family::Name("Courier New"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const DEFAULT_BOLD_FONT: Font = Font {
    family: iced::font::Family::Name("Courier New"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

// TODO: Make this configurable in settings
pub const BG_COLOR: Color = Color::from_rgb(0.99, 0.99, 0.99);
pub const BUTTON_COLOR: Color = Color::from_rgb(0.2, 0.2, 0.2);
pub const BUTTON_TEXT_COLOR: Color = Color::WHITE;
pub const SEARCH_BG_COLOR: Color = Color::TRANSPARENT;
pub const SEARCH_FOCUS_COLOR: Color = Color::from_rgb(0.2, 0.2, 0.2);
pub const HIGHLIGHT_COLOR: Color = Color::from_rgb(1.0, 0.8, 0.8); // Pinkish
// pub const HIGHLIGHT_COLOR: Color = Color::from_rgb(1.0, 0.90, 0.45); // Yellowish

pub fn body_button_style(
    _theme: &Theme,
    status: button::Status,
) -> button::Style {
    // On hover make the border visible and background transparent
    let background_color = match status {
        button::Status::Hovered => Color::TRANSPARENT,
        _ => BUTTON_COLOR,
    };

    let text_color = match status {
        button::Status::Hovered => Color::BLACK,
        _ => BUTTON_TEXT_COLOR,
    };

    button::Style {
        background: Some(Background::Color(background_color)),
        text_color: text_color,
        border: Border {
            radius: 0.0.into(),
            width: 1.0,
            color: Color::BLACK,
        },
        ..Default::default()
    }
}

pub fn header_button_style(
    _theme: &Theme,
    status: button::Status,
) -> button::Style {
    // On hover make the border visible and background transparent
    let background_color = match status {
        button::Status::Hovered => HIGHLIGHT_COLOR,
        _ => Color::TRANSPARENT,
    };

    button::Style {
        background: Some(Background::Color(background_color)),
        text_color: Color::BLACK,
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::BLACK,
        },
        ..Default::default()
    }
}

pub fn background_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_COLOR)),
        ..Default::default()
    }
}

pub fn search_input_style(
    _theme: &Theme,
    status: text_input::Status,
) -> text_input::Style {
    text_input::Style {
        background: Background::Color(SEARCH_BG_COLOR),
        border: Border {
            radius: 0.0.into(),
            width: 1.0,
            color: if matches!(status, text_input::Status::Focused) {
                // By default it's an ugly blue color
                SEARCH_FOCUS_COLOR
            } else {
                Color::from_rgb(0.6, 0.6, 0.6)
            },
        },
        icon: Color::BLACK,
        placeholder: Color::from_rgb(0.6, 0.6, 0.6),
        value: Color::BLACK,
        selection: HIGHLIGHT_COLOR,
    }
}
