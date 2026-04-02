// src/theme.rs

use iced::widget::{button, container};
use iced::{Background, Border, Color, Theme};

// ==========================================
// 1. The Design System Palette
// ==========================================
pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub primary: Color,
    pub secondary: Color,
    pub text_main: Color,
    pub border: Color,
}

impl Palette {
    pub const PRO_DARK: Self = Self {
        background: Color::from_rgb(0.03, 0.03, 0.04),
        surface: Color::from_rgb(0.08, 0.08, 0.09),
        primary: Color::from_rgb(0.90, 0.10, 0.20),
        secondary: Color::from_rgb(0.0, 0.55, 0.95),
        text_main: Color::from_rgb(0.92, 0.92, 0.92),
        border: Color::from_rgb(0.15, 0.15, 0.16),
    };
}

// ==========================================
// 2. Containers (Backgrounds & Panels)
// ==========================================
pub fn main_background(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Palette::PRO_DARK.background)),
        text_color: Some(Palette::PRO_DARK.text_main),
        ..Default::default()
    }
}

pub fn panel_background(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Palette::PRO_DARK.surface)),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: Palette::PRO_DARK.border,
        },
        text_color: Some(Palette::PRO_DARK.text_main),
        ..Default::default()
    }
}

// ==========================================
// 3. Buttons
// ==========================================
pub struct PlaybackButton;
impl button::StyleSheet for PlaybackButton {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                radius: 25.0.into(),
                width: 2.0,
                color: Palette::PRO_DARK.primary,
            },
            text_color: Palette::PRO_DARK.primary,
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.90, 0.10, 0.20, 0.15))),
            ..active
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Palette::PRO_DARK.primary)),
            text_color: Color::WHITE,
            ..active
        }
    }
}

pub struct TabButton;
impl button::StyleSheet for TabButton {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                radius: 6.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Palette::PRO_DARK.text_main,
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.55, 0.95, 0.15))),
            ..active
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Palette::PRO_DARK.secondary)),
            ..active
        }
    }
}

// ==========================================
// 4. LED Indicators
// ==========================================
fn colored_box(color: Color) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(color)),
        border: Border {
            radius: 3.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}
pub fn green_active(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.1, 0.9, 0.2))
}
pub fn green_inactive(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.05, 0.25, 0.1))
}
pub fn blue_active(_theme: &Theme) -> container::Appearance {
    colored_box(Palette::PRO_DARK.secondary)
}
pub fn blue_inactive(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.0, 0.2, 0.3))
}
pub fn red_active(_theme: &Theme) -> container::Appearance {
    colored_box(Palette::PRO_DARK.primary)
}
pub fn red_inactive(_theme: &Theme) -> container::Appearance {
    colored_box(Color::from_rgb(0.3, 0.0, 0.05))
}
pub fn led_panel_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Palette::PRO_DARK.surface)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: Palette::PRO_DARK.border,
        },
        ..Default::default()
    }
}

// ==========================================
// 5. Menu Bar Styling (NEW)
// ==========================================
pub fn menu_bar_background(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgb(0.02, 0.02, 0.02))),
        text_color: Some(Palette::PRO_DARK.text_main),
        ..Default::default()
    }
}

pub struct MenuBarButton;
impl button::StyleSheet for MenuBarButton {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                radius: 4.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Palette::PRO_DARK.text_main,
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))),
            ..active
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.2))),
            ..active
        }
    }
}
