// src/ui/ui.rs
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};
use crate::{
    app::App,
    ui::{calendar_tab, entries_tab, help, settings_tab},
    ui::util::{PRIMARY_COLOR, BORDER_COLOR, BACKGROUND_COLOR, HIGHLIGHT_COLOR, TEXT_COLOR, SUBTLE_TEXT},
};

pub fn ui(f: &mut Frame, app: &mut App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Top bar
            Constraint::Min(0),     // Main content
            Constraint::Length(1),  // Status bar
        ])
        .split(f.area());

    // Render tabs
    let titles: Vec<Line> = ["Entries", "Calendar", "Settings"]
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(HIGHLIGHT_COLOR)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(rest, Style::default().fg(TEXT_COLOR)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(BORDER_COLOR))
        )
        .select(app.tab_index)
        .style(Style::default().fg(TEXT_COLOR))
        .highlight_style(
            Style::default()
                .fg(PRIMARY_COLOR)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, chunks[0]);

    match app.tab_index {
        0 => entries_tab::render(f, app, chunks[1]),
        1 => calendar_tab::render(f, app, chunks[1]),
        2 => settings_tab::render(f, app, chunks[1]),
        _ => unreachable!(),
    }

    // Render status bar
    let status = Line::from(vec![
        Span::raw(" "),
        if !app.status_message.is_empty() {
            Span::styled(&app.status_message, Style::default().fg(PRIMARY_COLOR))
        } else {
            Span::styled(
                format!("Press 'h' for help | {} entries", app.entries_list.items.len()),
                Style::default().fg(SUBTLE_TEXT),
            )
        },
    ]);

    let status_bar = Paragraph::new(status)
        .style(Style::default().bg(BACKGROUND_COLOR))
        .alignment(Alignment::Left);

    f.render_widget(status_bar, chunks[2]);

    // Render help overlay if requested
    if app.show_help {
        help::render(f, f.size());
    }
}
