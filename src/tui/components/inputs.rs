use std::default;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{HighlightSpacing, List, ListItem, ListState},
};

use crate::tui::{
    state::fields::{BooleanField, ListField, NumericField},
    style::{ACCENT_COLOR, TEXT_STYLE_ACTIVE, TEXT_STYLE_DIM},
};

pub trait Input {
    fn render(&self) -> Line<'static>;
}

pub trait WidgetInput {
    fn render(&mut self, frame: &mut Frame, rect: Rect);
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

impl ListField {
    fn get_list_item<'a>(is_selected: bool, text: &'a str) -> ListItem<'a> {
        // Create the List Item
        let indicator = match is_selected {
            true => Span::from("●").fg(ACCENT_COLOR),
            false => Span::from("○"),
        };

        let content = Span::from(text);

        ListItem::new(Line::from(vec![indicator, Span::raw("  "), content]))
    }
}

impl WidgetInput for ListField {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        // If no item is selected, select the first one

        let list_items: Vec<ListItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(index, option)| {
                ListField::get_list_item(Some(index) == self.state.selected(), option)
            })
            .collect();

        let list_widget = List::new(list_items)
            .highlight_style(Style::default().fg(ACCENT_COLOR))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(list_widget, rect, &mut self.state);
    }
}
