use ratatui::style::{Color, Modifier, Style};

/// Modern OpenCode-inspired theme with true color support
pub struct Theme;

impl Theme {
    // ═══════════════════════════════════════════════════════════════════════
    // COLOR PALETTE - Modern dark theme with vibrant accents
    // ═══════════════════════════════════════════════════════════════════════

    // Base colors
    const BG_PRIMARY: Color = Color::Rgb(13, 17, 23);      // Deep dark background
    const BG_SECONDARY: Color = Color::Rgb(22, 27, 34);    // Slightly lighter
    const BG_TERTIARY: Color = Color::Rgb(33, 38, 45);     // Surface color
    const BG_HIGHLIGHT: Color = Color::Rgb(48, 54, 61);    // Hover/selection bg

    // Text colors
    const TEXT_PRIMARY: Color = Color::Rgb(230, 237, 243);  // Primary text
    const TEXT_SECONDARY: Color = Color::Rgb(139, 148, 158); // Muted text
    const TEXT_MUTED: Color = Color::Rgb(110, 118, 129);    // Very muted

    // Accent colors - vibrant and modern
    const ACCENT_CYAN: Color = Color::Rgb(88, 166, 255);    // Primary accent
    const ACCENT_GREEN: Color = Color::Rgb(63, 185, 80);    // Success/enabled
    const ACCENT_YELLOW: Color = Color::Rgb(210, 153, 34);  // Warning
    const ACCENT_RED: Color = Color::Rgb(248, 81, 73);      // Error/danger
    const ACCENT_PURPLE: Color = Color::Rgb(163, 113, 247); // Special
    const ACCENT_ORANGE: Color = Color::Rgb(219, 109, 40);  // Pending changes

    // Border colors
    const BORDER_DEFAULT: Color = Color::Rgb(48, 54, 61);
    const BORDER_FOCUSED: Color = Color::Rgb(88, 166, 255);

    // ═══════════════════════════════════════════════════════════════════════
    // BRANDING & HEADER
    // ═══════════════════════════════════════════════════════════════════════

    pub fn logo() -> Style {
        Style::default()
            .fg(Self::ACCENT_CYAN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header() -> Style {
        Style::default()
            .fg(Self::TEXT_PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_accent() -> Style {
        Style::default()
            .fg(Self::ACCENT_CYAN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_admin() -> Style {
        Style::default()
            .fg(Self::ACCENT_GREEN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_no_admin() -> Style {
        Style::default()
            .fg(Self::ACCENT_YELLOW)
    }

    pub fn version() -> Style {
        Style::default()
            .fg(Self::TEXT_MUTED)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // GROUP HEADERS
    // ═══════════════════════════════════════════════════════════════════════

    pub fn group_header() -> Style {
        Style::default()
            .fg(Self::ACCENT_PURPLE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn group_collapsed() -> Style {
        Style::default()
            .fg(Self::TEXT_MUTED)
    }

    pub fn group_count() -> Style {
        Style::default()
            .fg(Self::TEXT_SECONDARY)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // LIST ITEMS
    // ═══════════════════════════════════════════════════════════════════════

    pub fn item_normal() -> Style {
        Style::default()
            .fg(Self::TEXT_PRIMARY)
    }

    pub fn item_selected() -> Style {
        Style::default()
            .fg(Self::BG_PRIMARY)
            .bg(Self::ACCENT_CYAN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn item_selected_bg() -> Style {
        Style::default()
            .bg(Self::BG_HIGHLIGHT)
    }

    pub fn item_disabled() -> Style {
        Style::default()
            .fg(Self::TEXT_MUTED)
    }

    pub fn item_enabled() -> Style {
        Style::default()
            .fg(Self::ACCENT_GREEN)
    }

    pub fn item_requires_admin() -> Style {
        Style::default()
            .fg(Self::ACCENT_YELLOW)
    }

    pub fn item_file_missing() -> Style {
        Style::default()
            .fg(Self::ACCENT_RED)
    }

    pub fn item_pending() -> Style {
        Style::default()
            .fg(Self::ACCENT_ORANGE)
            .add_modifier(Modifier::BOLD)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CHECKBOXES & ICONS
    // ═══════════════════════════════════════════════════════════════════════

    pub fn checkbox_enabled() -> Style {
        Style::default()
            .fg(Self::ACCENT_GREEN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn checkbox_disabled() -> Style {
        Style::default()
            .fg(Self::TEXT_MUTED)
    }

    pub fn icon_enabled() -> Style {
        Style::default()
            .fg(Self::ACCENT_GREEN)
    }

    pub fn icon_disabled() -> Style {
        Style::default()
            .fg(Self::TEXT_MUTED)
    }

    pub fn icon_admin() -> Style {
        Style::default()
            .fg(Self::ACCENT_YELLOW)
    }

    pub fn icon_missing() -> Style {
        Style::default()
            .fg(Self::ACCENT_RED)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // STATUS BAR
    // ═══════════════════════════════════════════════════════════════════════

    pub fn status_bar() -> Style {
        Style::default()
            .fg(Self::TEXT_SECONDARY)
            .bg(Self::BG_SECONDARY)
    }

    pub fn status_bar_bg() -> Color {
        Self::BG_SECONDARY
    }

    pub fn status_key() -> Style {
        Style::default()
            .fg(Self::BG_PRIMARY)
            .bg(Self::ACCENT_CYAN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn status_description() -> Style {
        Style::default()
            .fg(Self::TEXT_SECONDARY)
    }

    pub fn status_separator() -> Style {
        Style::default()
            .fg(Self::BORDER_DEFAULT)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DETAILS PANEL
    // ═══════════════════════════════════════════════════════════════════════

    pub fn detail_label() -> Style {
        Style::default()
            .fg(Self::ACCENT_CYAN)
    }

    pub fn detail_value() -> Style {
        Style::default()
            .fg(Self::TEXT_PRIMARY)
    }

    pub fn detail_muted() -> Style {
        Style::default()
            .fg(Self::TEXT_SECONDARY)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MESSAGES & ALERTS
    // ═══════════════════════════════════════════════════════════════════════

    pub fn error() -> Style {
        Style::default()
            .fg(Self::ACCENT_RED)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success() -> Style {
        Style::default()
            .fg(Self::ACCENT_GREEN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn warning() -> Style {
        Style::default()
            .fg(Self::ACCENT_YELLOW)
            .add_modifier(Modifier::BOLD)
    }

    pub fn info() -> Style {
        Style::default()
            .fg(Self::ACCENT_CYAN)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // BORDERS
    // ═══════════════════════════════════════════════════════════════════════

    pub fn border() -> Style {
        Style::default()
            .fg(Self::BORDER_DEFAULT)
    }

    pub fn border_focused() -> Style {
        Style::default()
            .fg(Self::BORDER_FOCUSED)
    }

    pub fn border_dim() -> Style {
        Style::default()
            .fg(Self::BG_TERTIARY)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // HELP OVERLAY
    // ═══════════════════════════════════════════════════════════════════════

    pub fn help_title() -> Style {
        Style::default()
            .fg(Self::ACCENT_CYAN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn help_key() -> Style {
        Style::default()
            .fg(Self::ACCENT_PURPLE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn help_description() -> Style {
        Style::default()
            .fg(Self::TEXT_SECONDARY)
    }

    pub fn help_section() -> Style {
        Style::default()
            .fg(Self::TEXT_MUTED)
    }

    pub fn help_bg() -> Color {
        Self::BG_SECONDARY
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SPECIAL ELEMENTS
    // ═══════════════════════════════════════════════════════════════════════

    pub fn spinner() -> Style {
        Style::default()
            .fg(Self::ACCENT_CYAN)
    }

    pub fn progress_bar() -> Style {
        Style::default()
            .fg(Self::ACCENT_GREEN)
    }

    pub fn progress_bg() -> Style {
        Style::default()
            .fg(Self::BG_TERTIARY)
    }

    pub fn tag_admin() -> Style {
        Style::default()
            .fg(Self::BG_PRIMARY)
            .bg(Self::ACCENT_YELLOW)
    }

    pub fn tag_service() -> Style {
        Style::default()
            .fg(Self::BG_PRIMARY)
            .bg(Self::ACCENT_PURPLE)
    }

    pub fn tag_registry() -> Style {
        Style::default()
            .fg(Self::BG_PRIMARY)
            .bg(Self::ACCENT_CYAN)
    }
}

/// Modern Unicode symbols for the UI
pub struct Icons;

impl Icons {
    // Checkboxes
    pub const CHECKBOX_ON: &'static str = "◉";
    pub const CHECKBOX_OFF: &'static str = "○";
    pub const CHECKBOX_UNKNOWN: &'static str = "◌";

    // Group arrows
    pub const ARROW_RIGHT: &'static str = "▸";
    pub const ARROW_DOWN: &'static str = "▾";

    // Status indicators
    pub const ENABLED: &'static str = "●";
    pub const DISABLED: &'static str = "○";
    pub const PENDING: &'static str = "◐";
    pub const MODIFIED: &'static str = "✱";

    // Tags/badges
    pub const ADMIN: &'static str = "⚡";
    pub const MISSING: &'static str = "⚠";
    pub const SERVICE: &'static str = "⚙";
    pub const TASK: &'static str = "⏱";
    pub const FOLDER: &'static str = "📁";
    pub const REGISTRY: &'static str = "📝";

    // Actions
    pub const CHECK: &'static str = "✓";
    pub const CROSS: &'static str = "✗";
    pub const INFO: &'static str = "ℹ";

    // Decorative
    pub const DOT: &'static str = "·";
    pub const BULLET: &'static str = "•";
    pub const SEPARATOR: &'static str = "│";

    // Branding
    pub const LOGO: &'static str = "◆";
}

/// Rounded border characters for modern look
pub struct Borders;

impl Borders {
    pub const TOP_LEFT: &'static str = "╭";
    pub const TOP_RIGHT: &'static str = "╮";
    pub const BOTTOM_LEFT: &'static str = "╰";
    pub const BOTTOM_RIGHT: &'static str = "╯";
    pub const HORIZONTAL: &'static str = "─";
    pub const VERTICAL: &'static str = "│";
    pub const CROSS: &'static str = "┼";
    pub const T_LEFT: &'static str = "├";
    pub const T_RIGHT: &'static str = "┤";
    pub const T_TOP: &'static str = "┬";
    pub const T_BOTTOM: &'static str = "┴";
}
