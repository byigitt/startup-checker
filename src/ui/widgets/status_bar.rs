use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::{state::MessageType, AppState, Icons, Theme};

pub fn render_status_bar(frame: &mut Frame, area: Rect, state: &AppState) {
    // Show message if present
    if let Some((message, msg_type)) = &state.status_message {
        let (icon, style) = match msg_type {
            MessageType::Info => (Icons::INFO, Theme::info()),
            MessageType::Success => (Icons::CHECK, Theme::success()),
            MessageType::Warning => (Icons::MISSING, Theme::warning()),
            MessageType::Error => (Icons::CROSS, Theme::error()),
        };

        let paragraph = Paragraph::new(Line::from(vec![
            Span::styled(format!(" {} ", icon), style),
            Span::styled(message.clone(), style),
        ]));
        frame.render_widget(paragraph, area);
        return;
    }

    // Modern pill-style keybindings
    let bindings = vec![
        ("Space", "Toggle"),
        ("Tab", "Expand"),
        ("a", "Apply"),
        ("r", "Refresh"),
        ("b", "Backup"),
        ("?", "Help"),
        ("q", "Quit"),
    ];

    let mut spans = Vec::new();
    spans.push(Span::raw(" "));

    for (i, (key, desc)) in bindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(format!("  {} ", Icons::DOT), Theme::status_separator()));
        }
        // Pill-style key indicator
        spans.push(Span::styled(format!(" {} ", key), Theme::status_key()));
        spans.push(Span::styled(format!(" {}", desc), Theme::status_description()));
    }

    let paragraph = Paragraph::new(Line::from(spans)).style(Theme::status_bar());
    frame.render_widget(paragraph, area);
}
