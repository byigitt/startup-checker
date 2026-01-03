use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemStatus {
    Enabled,
    Disabled,
    Unknown,
}

impl ItemStatus {
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }

    pub fn toggle(&self) -> Self {
        match self {
            Self::Enabled => Self::Disabled,
            Self::Disabled | Self::Unknown => Self::Enabled,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
            Self::Unknown => "Unknown",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Enabled => "[x]",
            Self::Disabled => "[ ]",
            Self::Unknown => "[?]",
        }
    }
}

impl Default for ItemStatus {
    fn default() -> Self {
        Self::Unknown
    }
}
