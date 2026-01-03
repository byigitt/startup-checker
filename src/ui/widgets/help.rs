use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::Theme;

pub fn render_help(frame: &mut Frame, area: Rect) {
    // Center the help dialog
    let popup_area = centered_rect(60, 70, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let help_text = vec![
        ("Navigation", vec![
            ("↑/k", "Move up"),
            ("↓/j", "Move down"),
            ("Home", "Go to first item"),
            ("End", "Go to last item"),
        ]),
        ("Actions", vec![
            ("Space", "Toggle enable/disable"),
            ("Tab", "Collapse/expand group"),
            ("Enter", "Show item details"),
            ("a", "Apply pending changes"),
            ("u", "Undo pending changes"),
        ]),
        ("Other", vec![
            ("r", "Refresh list"),
            ("b", "Create backup"),
            ("?", "Toggle help"),
            ("q/Esc", "Quit"),
        ]),
    ];

    let mut lines = vec![
        Line::from(Span::styled("Keyboard Shortcuts", Theme::help_title())),
        Line::from(""),
    ];

    for (section, bindings) in help_text {
        lines.push(Line::from(Span::styled(
            format!("─── {} ───", section),
            Theme::group_header(),
        )));

        for (key, description) in bindings {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("{:12}", key), Theme::help_key()),
                Span::styled(description, Theme::help_description()),
            ]));
        }

        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "Press ? or Esc to close",
        Theme::info(),
    )));

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_focused())
            .title(" Help ")
            .title_style(Theme::header()),
    );

    frame.render_widget(paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
