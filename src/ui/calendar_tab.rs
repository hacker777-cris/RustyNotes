// src/ui/calendar_tab.rs
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, _app: &mut App, area: Rect) {
    let now = chrono::Local::now();
    let current_month = now.format("%B %Y").to_string();
    
    let calendar = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::styled(
                current_month,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(" Su  Mo  Tu  We  Th  Fr  Sa "),
        Line::from("────────────────────────────"),
        // TODO: Implement full calendar rendering
    ]))
    .block(
        Block::default()
            .title("Calendar View")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    
    f.render_widget(calendar, area);
}
