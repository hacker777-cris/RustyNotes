// src/ui/util.rs
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders};

// Helper function to create a centered rect using up certain percentage of the available rect
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

// === THEME COLORS === //
pub const PRIMARY_COLOR: Color = Color::Rgb(121, 184, 81);     // Light green
pub const BORDER_COLOR: Color = Color::Rgb(88, 118, 184);      // Soft blue
pub const BACKGROUND_COLOR: Color = Color::Black;
pub const HIGHLIGHT_COLOR: Color = Color::Rgb(255, 183, 77);   // Warm amber
pub const TEXT_COLOR: Color = Color::White;
pub const SECONDARY_TEXT: Color = Color::Rgb(153, 153, 153);   // Lighter gray
pub const ACCENT_COLOR: Color = Color::Rgb(191, 97, 106);      // Soft red
pub const SUBTLE_TEXT: Color = Color::DarkGray;

// Optional helper for consistent blocks with borders
pub fn themed_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER_COLOR))
        .border_type(BorderType::Rounded)
}
