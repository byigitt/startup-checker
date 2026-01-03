use ratatui::{
    layout::Rect,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::models::{ItemStatus, SourceType};
use crate::ui::{state::DisplayItem, AppState, Icons, Theme};

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
                } => render_group_header(source, *count, *collapsed, is_selected),
                DisplayItem::Item(item) => {
                    let effective_status = state.get_effective_status(item);
                    let has_pending = state.pending_changes.contains_key(&item.id);
                    render_item(item, effective_status, has_pending, is_selected, state.is_admin)
                }
            }
        })
        .collect();

    // Title with counts and modern styling
    let total = state.total_items();
    let pending = state.pending_change_count();

    let title = if pending > 0 {
        format!(" {} Startup Items  {} total · {} pending ", Icons::LOGO, total, pending)
    } else {
        format!(" {} Startup Items  {} total ", Icons::LOGO, total)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED)
                .border_style(Theme::border_focused())
                .title(title)
                .title_style(Theme::header_accent()),
        )
        .highlight_style(Theme::item_selected());

    frame.render_stateful_widget(list, area, &mut state.list_state);
}

fn render_group_header(
    source: &SourceType,
    count: usize,
    collapsed: bool,
    is_selected: bool,
) -> ListItem<'static> {
    let arrow = if collapsed { Icons::ARROW_RIGHT } else { Icons::ARROW_DOWN };
    let icon = get_source_icon(source);

    let style = if is_selected {
        Theme::item_selected()
    } else {
        Theme::group_header()
    };

    let count_style = if is_selected {
        Theme::item_selected()
    } else {
        Theme::group_count()
    };

    let spans = vec![
        Span::styled(format!(" {} ", arrow), style),
        Span::styled(format!("{} ", icon), style),
        Span::styled(source.display_name().to_string(), style),
        Span::styled(format!("  {} items", count), count_style),
    ];

    ListItem::new(Line::from(spans))
}

fn render_item(
    item: &crate::models::StartupItem,
    effective_status: ItemStatus,
    has_pending: bool,
    is_selected: bool,
    is_admin: bool,
) -> ListItem<'static> {
    // Modern checkbox style
    let checkbox = match effective_status {
        ItemStatus::Enabled => Icons::CHECKBOX_ON,
        ItemStatus::Disabled => Icons::CHECKBOX_OFF,
        ItemStatus::Unknown => Icons::CHECKBOX_UNKNOWN,
    };

    // Determine styles based on state
    let (checkbox_style, name_style) = if is_selected {
        (Theme::item_selected(), Theme::item_selected())
    } else if !item.file_exists {
        (Theme::checkbox_disabled(), Theme::item_file_missing())
    } else if item.requires_admin && !is_admin {
        (
            if effective_status.is_enabled() { Theme::checkbox_enabled() } else { Theme::checkbox_disabled() },
            Theme::item_requires_admin(),
        )
    } else if effective_status.is_enabled() {
        (Theme::checkbox_enabled(), Theme::item_enabled())
    } else {
        (Theme::checkbox_disabled(), Theme::item_disabled())
    };

    let mut spans = vec![
        Span::raw("    "),
        Span::styled(checkbox, checkbox_style),
        Span::raw("  "),
        Span::styled(item.name.clone(), name_style),
    ];

    // Add pending indicator with modern style
    if has_pending && !is_selected {
        spans.push(Span::styled(
            format!(" {}", Icons::MODIFIED),
            Theme::item_pending(),
        ));
    }

    // Add admin indicator with icon
    if item.requires_admin && !is_selected {
        spans.push(Span::styled(
            format!(" {}", Icons::ADMIN),
            Theme::icon_admin(),
        ));
    }

    // Add file missing indicator
    if !item.file_exists && !is_selected {
        spans.push(Span::styled(
            format!(" {}", Icons::MISSING),
            Theme::icon_missing(),
        ));
    }

    ListItem::new(Line::from(spans))
}

fn get_source_icon(source: &SourceType) -> &'static str {
    match source {
        SourceType::RegistryCurrentUserRun
        | SourceType::RegistryCurrentUserRunOnce
        | SourceType::RegistryLocalMachineRun
        | SourceType::RegistryLocalMachineRunOnce
        | SourceType::RegistryLocalMachineWow6432 => Icons::REGISTRY,
        SourceType::StartupFolderUser | SourceType::StartupFolderAllUsers => Icons::FOLDER,
        SourceType::ScheduledTask => Icons::TASK,
        SourceType::WindowsService => Icons::SERVICE,
    }
}
