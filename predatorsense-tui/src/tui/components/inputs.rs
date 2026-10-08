use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{HighlightSpacing, LineGauge, List, ListItem},
};

use crate::tui::{
    state::fields::{BooleanField, ColorField, ListField, NumericField, OptionField, SliderField},
    style::{ACCENT_COLOR, TEXT_DIM_COLOR, TEXT_STYLE_ACTIVE, TEXT_STYLE_DIM},
};

pub trait Input {
    fn render(&self) -> Line<'_>;
}

pub trait WidgetInput {
    fn render(&mut self, frame: &mut Frame, rect: Rect);
}

pub enum InputType<'a> {
    Widget(&'a mut dyn WidgetInput),
    Line(&'a dyn Input),
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

pub trait RenderOption {
    fn render_option(&self, is_selected: bool) -> Span<'static>;
}

impl<T: std::fmt::Display> RenderOption for T {
    fn render_option(&self, is_selected: bool) -> Span<'static> {
        let content = format!("[{}{}]", if is_selected { "▣ " } else { "" }, self);
        let mut span = Span::from(content);

        if is_selected {
            span = span.fg(ACCENT_COLOR);
        } else {
            span = span.fg(TEXT_DIM_COLOR);
        }

        span
    }
}

impl<T: RenderOption> Input for OptionField<T> {
    fn render(&self) -> Line<'static> {
        let mut spans = Vec::new();
        for (index, option) in self.options.iter().enumerate() {
            let is_selected = index == self.selected_option_index;
            let span = option.render_option(is_selected);
            spans.push(span);
            spans.push(Span::raw("  ")); // Add spacing between options
        }
        //

        Line::from(spans)
    }
}

impl SliderField {
    pub fn color_for(&self, value: usize) -> Option<Color> {
        for range in &self.ranges {
            if value >= range.start && value <= range.end {
                return Some(range.color);
            }
        }
        None
    }
}

impl WidgetInput for SliderField {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let ratio = self.value as f64 / self.max as f64;
        let layout: [Rect; 2] =
            Layout::horizontal([Constraint::Length(20), Constraint::Length(5)]).areas(rect);

        let color = self.color_for(self.value).unwrap_or(ACCENT_COLOR);
        let line_gauge = LineGauge::default()
            .ratio(ratio)
            .unfilled_style(TEXT_DIM_COLOR)
            .filled_symbol("█")
            .unfilled_symbol("░")
            .label("")
            .filled_style(color)
            .bold();

        let text = format!(" {}{}", self.value, self.unit);
        let text_span = Span::from(text).style(TEXT_STYLE_DIM);

        frame.render_widget(line_gauge, layout[0]);
        frame.render_widget(text_span, layout[1]);
    }
}

impl Input for ColorField {
    fn render(&self) -> Line<'static> {
        // let block_span = Span::from(").fg(self.color);
        let line = Line::from("██").fg(self.color);
        line
    }
}
