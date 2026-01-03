use ratatui::style::{Color, Modifier, Style};

pub struct Theme;

impl Theme {
    // Header
    pub fn header() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_admin() -> Style {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_no_admin() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    // Groups
    pub fn group_header() -> Style {
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    }

    pub fn group_collapsed() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    // Items
    pub fn item_normal() -> Style {
        Style::default().fg(Color::White)
    }

    pub fn item_selected() -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub fn item_disabled() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn item_enabled() -> Style {
        Style::default().fg(Color::Green)
    }

    pub fn item_requires_admin() -> Style {
        Style::default().fg(Color::Yellow)
    }

    pub fn item_file_missing() -> Style {
        Style::default().fg(Color::Red)
    }

    // Checkbox
    pub fn checkbox_enabled() -> Style {
        Style::default().fg(Color::Green)
    }

    pub fn checkbox_disabled() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    // Status bar
    pub fn status_bar() -> Style {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    }

    pub fn status_key() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub fn status_description() -> Style {
        Style::default().fg(Color::White)
    }

    // Details panel
    pub fn detail_label() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub fn detail_value() -> Style {
        Style::default().fg(Color::White)
    }

    // Messages
    pub fn error() -> Style {
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success() -> Style {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    }

    pub fn warning() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    pub fn info() -> Style {
        Style::default().fg(Color::Cyan)
    }

    // Borders
    pub fn border() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn border_focused() -> Style {
        Style::default().fg(Color::Cyan)
    }

    // Help overlay
    pub fn help_title() -> Style {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    }

    pub fn help_key() -> Style {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    }

    pub fn help_description() -> Style {
        Style::default().fg(Color::White)
    }
}
