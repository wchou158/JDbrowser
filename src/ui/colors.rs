use ratatui::style::{Color, Style};

pub const PRIMARY_COLOR: Color = Color::Blue;
pub const SECONDARY_COLOR: Color = Color::Yellow;
pub const TEXT_COLOR: Color = Color::White;
pub const HIGHLIGHTED_COLOR: Color = Color::Green;

pub const HIGHLIGHT_STYLE: Style = Style::new().fg(HIGHLIGHTED_COLOR);
