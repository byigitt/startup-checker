use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::models::ItemStatus;
use crate::ui::{state::DisplayItem, AppState, Theme};

pub fn render_list(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let items: Vec<ListItem> = state
        .display_list
        .iter()
        .enumerate()
        .map(|(index, display_item)| {
            let is_selected = state.list_state.selected() == Some(index);

            match display_item {
                DisplayItem::GroupHeader {
                    source,
                    count,
                    collapsed,
                } => {
                    let arrow = if *collapsed { "▶" } else { "▼" };
                    let text = format!(" {} {} [{} items]", arrow, source.display_name(), count);

                    let style = if is_selected {
                        Theme::item_selected()
                    } else {
                        Theme::group_header()
                    };

                    ListItem::new(Line::from(Span::styled(text, style)))
                }
                DisplayItem::Item(item) => {
                    let effective_status = state.get_effective_status(item);
                    let has_pending = state.pending_changes.contains_key(&item.id);

                    // Checkbox
                    let checkbox = match effective_status {
                        ItemStatus::Enabled => "[x]",
                        ItemStatus::Disabled => "[ ]",
                        ItemStatus::Unknown => "[?]",
                    };

                    let checkbox_style = if effective_status.is_enabled() {
                        Theme::checkbox_enabled()
                    } else {
                        Theme::checkbox_disabled()
                    };

                    // Name
                    let name = &item.name;
                    let name_style = if is_selected {
                        Theme::item_selected()
                    } else if !item.file_exists {
                        Theme::item_file_missing()
                    } else if item.requires_admin && !state.is_admin {
                        Theme::item_requires_admin()
                    } else if effective_status.is_enabled() {
                        Theme::item_enabled()
                    } else {
                        Theme::item_disabled()
                    };

                    // Pending indicator
                    let pending_indicator = if has_pending { " *" } else { "" };

                    // Admin indicator
                    let admin_indicator = if item.requires_admin { " [A]" } else { "" };

                    let mut spans = vec![
                        Span::raw("   "),
                        Span::styled(checkbox, if is_selected { Theme::item_selected() } else { checkbox_style }),
                        Span::raw(" "),
                        Span::styled(name.clone(), name_style),
                    ];

                    if !pending_indicator.is_empty() {
                        spans.push(Span::styled(
                            pending_indicator,
                            Style::default().fg(ratatui::style::Color::Yellow).add_modifier(Modifier::BOLD),
                        ));
                    }

                    if !admin_indicator.is_empty() && !is_selected {
                        spans.push(Span::styled(admin_indicator, Theme::item_requires_admin()));
                    }

                    ListItem::new(Line::from(spans))
                }
            }
        })
        .collect();

    // Title with counts
    let total = state.total_items();
    let pending = state.pending_change_count();
    let title = if pending > 0 {
        format!(" Startup Items ({} total, {} pending) ", total, pending)
    } else {
        format!(" Startup Items ({} total) ", total)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Theme::border_focused())
                .title(title)
                .title_style(Theme::header()),
        )
        .highlight_style(Theme::item_selected());

    frame.render_stateful_widget(list, area, &mut state.list_state);
}
