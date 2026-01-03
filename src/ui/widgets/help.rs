use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::{Icons, Theme};

pub fn render_help(frame: &mut Frame, area: Rect) {
    // Center the help dialog
    let popup_area = centered_rect(55, 65, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let help_sections = vec![
        (
            "Navigation",
            vec![
                ("↑  k", "Move up"),
                ("↓  j", "Move down"),
                ("Home", "Jump to first"),
                ("End", "Jump to last"),
            ],
        ),
        (
            "Actions",
            vec![
                ("Space", "Toggle item"),
                ("Tab", "Expand/collapse"),
                ("a", "Apply changes"),
                ("u", "Undo pending"),
            ],
        ),
        (
            "Other",
            vec![
                ("r", "Refresh list"),
                ("b", "Create backup"),
                ("?", "Toggle help"),
                ("q", "Quit app"),
            ],
        ),
    ];

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(format!("  {} ", Icons::LOGO), Theme::logo()),
            Span::styled("Keyboard Shortcuts", Theme::help_title()),
        ]),
        Line::from(""),
    ];

    for (section, bindings) in help_sections {
        // Section header with modern styling
        lines.push(Line::from(vec![
            Span::styled("  ", Theme::help_section()),
            Span::styled(format!("─── {} ", section), Theme::help_section()),
            Span::styled("───────────────────", Theme::border_dim()),
        ]));
        lines.push(Line::from(""));

        for (key, description) in bindings {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(format!(" {} ", key), Theme::status_key()),
                Span::raw("  "),
                Span::styled(description, Theme::help_description()),
            ]));
        }

        lines.push(Line::from(""));
    }

    // Color legend
    lines.push(Line::from(vec![
        Span::styled("  ", Theme::help_section()),
        Span::styled("─── Colors ", Theme::help_section()),
        Span::styled("────────────────────", Theme::border_dim()),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled(Icons::ENABLED, Theme::item_enabled()),
        Span::styled(" Green", Theme::item_enabled()),
        Span::styled("  Enabled", Theme::help_description()),
    ]));
    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled(Icons::ADMIN, Theme::icon_admin()),
        Span::styled(" Yellow", Theme::icon_admin()),
        Span::styled(" Needs Admin", Theme::help_description()),
    ]));
    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled(Icons::MISSING, Theme::icon_missing()),
        Span::styled(" Red", Theme::icon_missing()),
        Span::styled("    Missing file", Theme::help_description()),
    ]));
    lines.push(Line::from(""));

    // Footer
    lines.push(Line::from(vec![
        Span::styled(
            format!("  Press {} or Esc to close", "?"),
            Theme::detail_muted(),
        ),
    ]));

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .border_style(Theme::border_focused())
            .title(format!(" {} Help ", Icons::INFO))
            .title_style(Theme::help_title()),
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
