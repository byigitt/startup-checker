use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

use crate::models::{ItemStatus, SourceType, StartupItem};
use crate::permissions::is_elevated;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Help,
    Confirm,
}

#[derive(Debug, Clone)]
pub enum DisplayItem {
    GroupHeader {
        source: SourceType,
        count: usize,
        collapsed: bool,
    },
    Item(StartupItem),
}

#[derive(Debug, Clone)]
pub struct PendingChange {
    pub item_id: String,
    pub old_status: ItemStatus,
    pub new_status: ItemStatus,
}

pub struct AppState {
    /// All startup items grouped by source
    pub items_by_source: HashMap<SourceType, Vec<StartupItem>>,

    /// Flat list for display (with group headers)
    pub display_list: Vec<DisplayItem>,

    /// List widget state
    pub list_state: ListState,

    /// Items with pending changes
    pub pending_changes: HashMap<String, PendingChange>,

    /// Current view mode
    pub view_mode: ViewMode,

    /// Is running as admin
    pub is_admin: bool,

    /// Status message
    pub status_message: Option<(String, MessageType)>,

    /// Collapsed groups
    pub collapsed_groups: HashSet<SourceType>,

    /// Show confirmation dialog
    pub confirm_action: Option<ConfirmAction>,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    ApplyChanges,
    DiscardChanges,
    Quit,
}

impl AppState {
    pub fn new(items: Vec<StartupItem>) -> Self {
        let is_admin = is_elevated();

        // Group items by source
        let mut items_by_source: HashMap<SourceType, Vec<StartupItem>> = HashMap::new();
        for item in items {
            items_by_source
                .entry(item.source)
                .or_default()
                .push(item);
        }

        // Sort items within each group
        for items in items_by_source.values_mut() {
            items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }

        let mut state = Self {
            items_by_source,
            display_list: Vec::new(),
            list_state: ListState::default(),
            pending_changes: HashMap::new(),
            view_mode: ViewMode::List,
            is_admin,
            status_message: None,
            collapsed_groups: HashSet::new(),
            confirm_action: None,
        };

        state.rebuild_display_list();

        // Select first item
        if !state.display_list.is_empty() {
            state.list_state.select(Some(0));
        }

        state
    }

    pub fn rebuild_display_list(&mut self) {
        self.display_list.clear();

        // Get sorted source types
        let mut sources: Vec<_> = self.items_by_source.keys().copied().collect();
        sources.sort();

        for source in sources {
            if let Some(items) = self.items_by_source.get(&source) {
                let collapsed = self.collapsed_groups.contains(&source);

                // Add group header
                self.display_list.push(DisplayItem::GroupHeader {
                    source,
                    count: items.len(),
                    collapsed,
                });

                // Add items if not collapsed
                if !collapsed {
                    for item in items {
                        self.display_list.push(DisplayItem::Item(item.clone()));
                    }
                }
            }
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    pub fn selected_item(&self) -> Option<&DisplayItem> {
        self.selected_index()
            .and_then(|i| self.display_list.get(i))
    }

    pub fn selected_startup_item(&self) -> Option<&StartupItem> {
        match self.selected_item()? {
            DisplayItem::Item(item) => Some(item),
            DisplayItem::GroupHeader { .. } => None,
        }
    }

    pub fn move_up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.display_list.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn move_down(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.display_list.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn toggle_selected(&mut self) {
        let Some(index) = self.selected_index() else {
            return;
        };

        match &self.display_list[index] {
            DisplayItem::GroupHeader { source, .. } => {
                // Toggle collapse
                let source = *source;
                if self.collapsed_groups.contains(&source) {
                    self.collapsed_groups.remove(&source);
                } else {
                    self.collapsed_groups.insert(source);
                }
                self.rebuild_display_list();

                // Keep selection on the same group header
                for (i, item) in self.display_list.iter().enumerate() {
                    if let DisplayItem::GroupHeader { source: s, .. } = item {
                        if *s == source {
                            self.list_state.select(Some(i));
                            break;
                        }
                    }
                }
            }
            DisplayItem::Item(item) => {
                // Toggle item status
                let item_id = item.id.clone();

                // Check if we can modify this item
                if item.requires_admin && !self.is_admin {
                    self.set_message(
                        "Cannot modify: Administrator privileges required".to_string(),
                        MessageType::Warning,
                    );
                    return;
                }

                // Get current effective status (considering pending changes)
                let current_status = self
                    .pending_changes
                    .get(&item_id)
                    .map(|c| c.new_status)
                    .unwrap_or(item.status);

                let new_status = current_status.toggle();

                // Update or create pending change
                if new_status == item.status {
                    // Change would revert to original, remove pending change
                    self.pending_changes.remove(&item_id);
                } else {
                    self.pending_changes.insert(
                        item_id.clone(),
                        PendingChange {
                            item_id,
                            old_status: item.status,
                            new_status,
                        },
                    );
                }
            }
        }
    }

    pub fn get_effective_status(&self, item: &StartupItem) -> ItemStatus {
        self.pending_changes
            .get(&item.id)
            .map(|c| c.new_status)
            .unwrap_or(item.status)
    }

    pub fn has_pending_changes(&self) -> bool {
        !self.pending_changes.is_empty()
    }

    pub fn pending_change_count(&self) -> usize {
        self.pending_changes.len()
    }

    pub fn clear_pending_changes(&mut self) {
        self.pending_changes.clear();
    }

    pub fn set_message(&mut self, message: String, msg_type: MessageType) {
        self.status_message = Some((message, msg_type));
    }

    pub fn clear_message(&mut self) {
        self.status_message = None;
    }

    pub fn total_items(&self) -> usize {
        self.items_by_source.values().map(|v| v.len()).sum()
    }

    pub fn refresh(&mut self, items: Vec<StartupItem>) {
        // Clear pending changes
        self.pending_changes.clear();

        // Rebuild items
        self.items_by_source.clear();
        for item in items {
            self.items_by_source
                .entry(item.source)
                .or_default()
                .push(item);
        }

        // Sort items within each group
        for items in self.items_by_source.values_mut() {
            items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }

        // Rebuild display
        let selected = self.selected_index();
        self.rebuild_display_list();

        // Try to restore selection
        if let Some(index) = selected {
            let new_index = index.min(self.display_list.len().saturating_sub(1));
            self.list_state.select(Some(new_index));
        }
    }
}
