use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::error::Result;
use crate::models::StartupItem;
use crate::operations::create_backup;
use crate::sources::{modify_item, scan_all_sources};
use crate::ui::state::MessageType;
use crate::ui::widgets::{render_help, render_list, render_status_bar};
use crate::ui::{AppState, Icons, Theme, ViewMode};

pub fn run_app(items: Vec<StartupItem>) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut state = AppState::new(items);

    // Main loop
    let result = run_loop(&mut terminal, &mut state);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|frame| render_ui(frame, state))?;

        // Handle input with timeout (for message clearing)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Clear any message on keypress
                state.clear_message();

                match state.view_mode {
                    ViewMode::Help => match key.code {
                        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
                            state.view_mode = ViewMode::List;
                        }
                        _ => {}
                    },
                    ViewMode::List => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if state.has_pending_changes() {
                                state.set_message(
                                    format!("{} Pending changes! Press 'a' to apply or 'u' to undo", Icons::MISSING),
                                    MessageType::Warning,
                                );
                            } else {
                                return Ok(());
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            state.move_up();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            state.move_down();
                        }
                        KeyCode::Home => {
                            state.list_state.select(Some(0));
                        }
                        KeyCode::End => {
                            let last = state.display_list.len().saturating_sub(1);
                            state.list_state.select(Some(last));
                        }
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            state.toggle_selected();
                        }
                        KeyCode::Tab => {
                            state.toggle_selected();
                        }
                        KeyCode::Char('a') => {
                            apply_changes(state);
                        }
                        KeyCode::Char('u') => {
                            state.clear_pending_changes();
                            state.set_message(format!("{} Pending changes discarded", Icons::CHECK), MessageType::Info);
                        }
                        KeyCode::Char('r') => {
                            refresh(state);
                        }
                        KeyCode::Char('b') => {
                            create_backup_action(state);
                        }
                        KeyCode::Char('?') => {
                            state.view_mode = ViewMode::Help;
                        }
                        _ => {}
                    },
                    ViewMode::Confirm => {
                        // Handle confirmation dialogs
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Enter => {
                                state.view_mode = ViewMode::List;
                            }
                            KeyCode::Char('n') | KeyCode::Esc => {
                                state.view_mode = ViewMode::List;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn render_ui(frame: &mut Frame, state: &mut AppState) {
    let size = frame.area();

    // Modern layout with better spacing
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main list
            Constraint::Length(6),  // Details
            Constraint::Length(1),  // Status bar
        ])
        .split(size);

    // Header
    render_header(frame, chunks[0], state);

    // Main list
    render_list(frame, chunks[1], state);

    // Details panel
    render_details(frame, chunks[2], state);

    // Status bar
    render_status_bar(frame, chunks[3], state);

    // Help overlay
    if state.view_mode == ViewMode::Help {
        render_help(frame, size);
    }
}

fn render_header(frame: &mut Frame, area: Rect, state: &AppState) {
    let admin_indicator = if state.is_admin {
        vec![
            Span::styled(" ", Theme::header_admin()),
            Span::styled(Icons::CHECK, Theme::header_admin()),
            Span::styled(" Admin ", Theme::header_admin()),
        ]
    } else {
        vec![
            Span::styled(" ", Theme::header_no_admin()),
            Span::styled(Icons::CROSS, Theme::header_no_admin()),
            Span::styled(" User ", Theme::header_no_admin()),
        ]
    };

    let title_spans = vec![
        Span::styled(format!(" {} ", Icons::LOGO), Theme::logo()),
        Span::styled("Startup Manager", Theme::header()),
        Span::styled(format!(" v{}", env!("CARGO_PKG_VERSION")), Theme::version()),
        Span::raw("  "),
    ];

    let mut all_spans = title_spans;
    all_spans.extend(admin_indicator);

    let title = Line::from(all_spans);

    let header = Paragraph::new(title)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED)
                .border_style(Theme::border()),
        );

    frame.render_widget(header, area);
}

fn render_details(frame: &mut Frame, area: Rect, state: &AppState) {
    let content = if let Some(item) = state.selected_startup_item() {
        let status = state.get_effective_status(item);
        let has_pending = state.pending_changes.contains_key(&item.id);

        let status_icon = if status.is_enabled() { Icons::ENABLED } else { Icons::DISABLED };
        let status_style = if status.is_enabled() { Theme::item_enabled() } else { Theme::item_disabled() };

        vec![
            Line::from(vec![
                Span::styled("  Name     ", Theme::detail_label()),
                Span::styled(&item.name, Theme::detail_value()),
                if has_pending {
                    Span::styled(format!("  {} modified", Icons::MODIFIED), Theme::item_pending())
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled("  Command  ", Theme::detail_label()),
                Span::styled(truncate_str(&item.display_command(), area.width.saturating_sub(14) as usize), Theme::detail_muted()),
            ]),
            Line::from(vec![
                Span::styled("  Status   ", Theme::detail_label()),
                Span::styled(format!("{} ", status_icon), status_style),
                Span::styled(status.display(), status_style),
                Span::styled(format!("  {}  Source  ", Icons::SEPARATOR), Theme::detail_muted()),
                Span::styled(item.source.short_name(), Theme::detail_value()),
                if item.requires_admin {
                    Span::styled(format!("  {} Requires Admin", Icons::ADMIN), Theme::icon_admin())
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled("  Location ", Theme::detail_label()),
                Span::styled(truncate_str(&item.source_location, area.width.saturating_sub(14) as usize), Theme::detail_muted()),
            ]),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("  {} Select an item to see details", Icons::INFO),
                Theme::detail_muted(),
            )),
        ]
    };

    let details = Paragraph::new(content).block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .border_style(Theme::border())
            .title(format!(" {} Details ", Icons::INFO))
            .title_style(Theme::detail_label()),
    );

    frame.render_widget(details, area);
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len > 3 {
        format!("{}...", &s[..max_len - 3])
    } else {
        s[..max_len].to_string()
    }
}

fn apply_changes(state: &mut AppState) {
    if !state.has_pending_changes() {
        state.set_message(format!("{} No pending changes to apply", Icons::INFO), MessageType::Info);
        return;
    }

    let changes: Vec<_> = state.pending_changes.values().cloned().collect();
    let total = changes.len();
    let mut success = 0;
    let mut failed = 0;

    // Create backup first
    let all_items: Vec<_> = state
        .items_by_source
        .values()
        .flatten()
        .cloned()
        .collect();

    if let Err(e) = create_backup(&all_items, Some("Before applying changes".to_string())) {
        state.set_message(
            format!("{} Backup failed: {}. Aborting.", Icons::CROSS, e),
            MessageType::Error,
        );
        return;
    }

    // Apply each change
    for change in changes {
        // Find the item
        let item = state
            .items_by_source
            .values()
            .flatten()
            .find(|i| i.id == change.item_id);

        if let Some(item) = item {
            match modify_item(item, change.new_status) {
                Ok(()) => success += 1,
                Err(e) => {
                    failed += 1;
                    eprintln!("Failed to modify {}: {}", item.name, e);
                }
            }
        }
    }

    // Refresh to get updated state
    refresh(state);

    if failed == 0 {
        state.set_message(
            format!("{} Applied {} changes successfully", Icons::CHECK, success),
            MessageType::Success,
        );
    } else {
        state.set_message(
            format!("{} Applied {}/{} changes ({} failed)", Icons::MISSING, success, total, failed),
            MessageType::Warning,
        );
    }
}

fn refresh(state: &mut AppState) {
    let items = scan_all_sources();
    state.refresh(items);
    state.set_message(format!("{} Refreshed", Icons::CHECK), MessageType::Info);
}

fn create_backup_action(state: &mut AppState) {
    let all_items: Vec<_> = state
        .items_by_source
        .values()
        .flatten()
        .cloned()
        .collect();

    match create_backup(&all_items, Some("Manual backup".to_string())) {
        Ok(path) => {
            state.set_message(
                format!("{} Backup created: {}", Icons::CHECK, path.file_name().unwrap_or_default().to_string_lossy()),
                MessageType::Success,
            );
        }
        Err(e) => {
            state.set_message(format!("{} Backup failed: {}", Icons::CROSS, e), MessageType::Error);
        }
    }
}
