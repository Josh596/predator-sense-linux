use std::str::FromStr;

use ratatui::style::{Color, Style};

pub const BG_COLOR: Color = Color::Rgb(0x0A, 0x0E, 0x12);
const BG__COLOR_PANEL: Color = Color::Rgb(0x11, 0x17, 0x1D);
const BORDER_COLOR: Color = Color::Rgb(0xFF, 0xFF, 0xFF);
const TEXT_COLOR: Color = Color::Rgb(0xC7, 0xD1, 0xD9);
pub const TEXT_DIM_COLOR: Color = Color::Rgb(0x5A, 0x6B, 0x77);
pub const ACCENT_COLOR: Color = Color::Rgb(0x00, 0xE5, 0xD0);
const GAUGE_FILL_COLOR: Color = Color::Rgb(0x0A, 0xB4, 0xFF);
pub const WARNING_COLOR: Color = Color::Rgb(0xFF, 0xB0, 0x20);
pub const DANGER_COLOR: Color = Color::Rgb(0xFF, 0x3B, 0x47);
pub const OK_COLOR: Color = Color::Rgb(0x4A, 0xE2, 0x6A);

// Text styles
pub const TEXT_STYLE: Style = Style::new().fg(TEXT_COLOR);
pub const TEXT_STYLE_ACTIVE: Style = Style::new().fg(ACCENT_COLOR);
pub const TEXT_STYLE_DIM: Style = Style::new().fg(TEXT_DIM_COLOR);

// Border styles
pub const BORDER_STYLE: Style = Style::new().fg(BORDER_COLOR);
pub const BORDER_STYLE_ACTIVE: Style = Style::new().fg(ACCENT_COLOR);

// Guage style
pub const GAUGE_STYLE: Style = Style::new().fg(GAUGE_FILL_COLOR);
