// Add lifetimes

use ratatui::text::Span;

use crate::tui::style::TEXT_STYLE_ACTIVE;

pub struct Action {
    pub key: String,
    pub action: String,
}

impl Action {
    pub fn new(key: impl Into<String>, action: impl Into<String>) -> Self {
        Action {
            key: key.into(),
            action: action.into(),
        }
    }

    pub fn to_str(&self) -> String {
        return format!("[{}] {}", self.key, self.action);
    }
}

pub fn cursor(is_focused: bool) -> Span<'static> {
    if is_focused {
        Span::raw("▸ ").patch_style(TEXT_STYLE_ACTIVE)
    } else {
        Span::raw("  ")
    }
}
