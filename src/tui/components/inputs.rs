use ratatui::text::{Line, Span};

use crate::tui::{
    state::fields::{BooleanField, NumericField},
    style::{TEXT_STYLE_ACTIVE, TEXT_STYLE_DIM},
};

pub trait Input {
    fn render(&self) -> Line<'static>;
}

impl Input for BooleanField {
    fn render(&self) -> Line<'static> {
        let text = if self.value {
            self.on_text
        } else {
            self.off_text
        };

        let style = if self.value {
            TEXT_STYLE_ACTIVE
        } else {
            TEXT_STYLE_DIM
        };
        let line = Line::from(vec![
            Span::from("["),
            Span::from(if self.value { " ● " } else { " ○ " }).style(style),
            Span::from("] "),
            Span::from(text).style(style),
        ]);

        line
    }
}

impl Input for NumericField {
    fn render(&self) -> Line<'static> {
        let line = Line::from(vec![
            Span::from("< "),
            Span::from(format!("{}{}", self.value, self.unit)).style(TEXT_STYLE_ACTIVE),
            Span::from(" >"),
        ]);

        line
    }
}
