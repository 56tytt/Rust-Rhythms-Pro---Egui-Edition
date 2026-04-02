// src/styles.rs
use iced::widget::{button, container};
use iced::{Background, Border, Color, Theme};

// ==========================================
// 1. פלטת הצבעים
// ==========================================
pub const BG_MAIN: Color = Color::from_rgb(0.04, 0.04, 0.05);
pub const BG_PANEL: Color = Color::from_rgb(0.08, 0.08, 0.09);
pub const ACCENT_RED: Color = Color::from_rgb(0.85, 0.1, 0.1);
pub const ACCENT_BLUE: Color = Color::from_rgb(0.1, 0.5, 0.9);
pub const TEXT_BRIGHT: Color = Color::from_rgb(0.9, 0.9, 0.9);
pub const BORDER_COLOR: Color = Color::from_rgb(0.15, 0.15, 0.15);

// ==========================================
// 2. עיצוב רקעים
// ==========================================
pub fn main_background(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(BG_MAIN)),
        text_color: Some(TEXT_BRIGHT),
        ..Default::default()
    }
}

pub fn panel_background(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(BG_PANEL)),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: BORDER_COLOR,
        },
        text_color: Some(TEXT_BRIGHT),
        ..Default::default() // תיקון שגיאת ה-Shadow
    }
}

// ==========================================
// 3. עיצוב כפתורים (בשיטה הנכונה - Struct)
// ==========================================

// כפתורי הנגן (אדומים)
pub struct PlaybackButton;
impl button::StyleSheet for PlaybackButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                radius: 25.0.into(),
                width: 2.0,
                color: ACCENT_RED,
            },
            text_color: ACCENT_RED,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.85, 0.1, 0.1, 0.15))),
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(ACCENT_RED)),
            text_color: Color::WHITE,
            ..active
        }
    }
}

// כפתורי הטאבים (כחולים)
pub struct TabButton;
impl button::StyleSheet for TabButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                radius: 4.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: TEXT_BRIGHT,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.1, 0.5, 0.9, 0.2))),
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(ACCENT_BLUE)),
            ..active
        }
    }
}

// ==========================================
// 4. עיצוב הלדים
// ==========================================
fn colored_box(color: Color) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(color)),
        border: Border {
            radius: 2.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

pub fn green_active(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.0, 1.0, 0.0))
}
pub fn green_inactive(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.0, 0.3, 0.0))
}
pub fn blue_active(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.0, 0.5, 1.0))
}
pub fn blue_inactive(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.0, 0.15, 0.3))
}
pub fn red_active(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(1.0, 0.0, 0.0))
}
pub fn red_inactive(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.3, 0.0, 0.0))
}

pub fn led_panel_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: Color::from_rgb(0.2, 0.2, 0.2),
        },
        ..Default::default()
    }
}
