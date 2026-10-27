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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    // Action::new

    // Action::to_str

    #[test]
    fn action_to_str_wraps_key_in_brackets() {
        assert_eq!(Action::new("q", "Quit").to_str(), "[q] Quit");
    }

    // cursor

    #[test]
    fn cursor_focused_is_highlighted_arrow() {
        let span = cursor(true);
        assert_eq!(span.content, "▸ ");
        assert_eq!(span.style, Style::default().patch(TEXT_STYLE_ACTIVE));
    }

    #[test]
    fn cursor_unfocused_is_same_width_blank() {
        let focused = cursor(true);
        let span = cursor(false);
        assert_eq!(span.content, "  ");
        assert_eq!(span.width(), focused.width());
        assert_eq!(span.style, Style::default());
    }
}
