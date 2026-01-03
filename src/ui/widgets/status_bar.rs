use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::{state::MessageType, AppState, Theme};

pub fn render_status_bar(frame: &mut Frame, area: Rect, state: &AppState) {
    // Show message if present
    if let Some((message, msg_type)) = &state.status_message {
        let style = match msg_type {
            MessageType::Info => Theme::info(),
            MessageType::Success => Theme::success(),
            MessageType::Warning => Theme::warning(),
            MessageType::Error => Theme::error(),
        };

        let paragraph = Paragraph::new(Line::from(Span::styled(message.clone(), style)));
        frame.render_widget(paragraph, area);
        return;
    }

    // Show key bindings
    let bindings = vec![
        ("Space", "Toggle"),
        ("Tab", "Collapse"),
        ("a", "Apply"),
        ("r", "Refresh"),
        ("b", "Backup"),
        ("?", "Help"),
        ("q", "Quit"),
    ];

    let mut spans = Vec::new();
    for (i, (key, desc)) in bindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" | "));
        }
        spans.push(Span::styled(*key, Theme::status_key()));
        spans.push(Span::raw(": "));
        spans.push(Span::styled(*desc, Theme::status_description()));
    }

    let paragraph = Paragraph::new(Line::from(spans)).style(Theme::status_bar());
    frame.render_widget(paragraph, area);
}
