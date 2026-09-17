use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Clear, Padding},
};

use crate::tui::{
    components::inputs::{RenderOption, WidgetInput},
    state::color_picker::{ColorChannel, ColorPickerInput, ColorPickerState, Preset},
    style::{ACCENT_COLOR, TEXT_STYLE_ACTIVE, TEXT_STYLE_DIM},
    utils::Action,
};

pub struct ColorPickerPopup;

impl ColorPickerPopup {
    fn get_title_span(&self, title: &str, is_active: bool) -> Span<'static> {
        let cursor = if is_active { ">" } else { " " };
        let content = format!("{} {}", cursor, title);
        Span::from(content).fg(ACCENT_COLOR)
    }
    pub fn render(&self, frame: &mut Frame, area: Rect, state: &mut ColorPickerState) {
        // So get a layout that is centered in the current frame.
        let area = area.centered(Constraint::Length(60), Constraint::Length(20));

        let block = Block::bordered()
            .title("Pick Color")
            .title_bottom(self.get_actions_text(state))
            .border_style(ACCENT_COLOR)
            .padding(Padding::uniform(2));
        let inner_area = block.inner(area);

        let layout: [Rect; 7] = Layout::vertical(vec![
            Constraint::Length(1), // Preset
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // R
            Constraint::Length(1), // G
            Constraint::Length(1), // B
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Swatch
        ])
        .areas(inner_area);

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);
        // Presets;
        // Render the preset input
        self.render_preset_input(frame, layout[0], state);
        self.render_color_slider(frame, layout[2], state, ColorChannel::Red);
        self.render_color_slider(frame, layout[3], state, ColorChannel::Green);
        self.render_color_slider(frame, layout[4], state, ColorChannel::Blue);
        self.render_color_swatch(frame, layout[6], state);
    }

    fn render_preset_input(&self, frame: &mut Frame, area: Rect, state: &ColorPickerState) {
        let layout: [Rect; 2] =
            Layout::horizontal([Constraint::Length(11), Constraint::Fill(1)]).areas(area);

        let title_span =
            self.get_title_span("Preset", state.active_input == ColorPickerInput::Preset);

        // Manually render the preset options so i can have it have a state where nothing is selected.

        let current = state.current_color();
        let mut spans = Vec::new();
        for p in &state.preset_input.options {
            spans.push(p.render_option(p.0 == current));
            spans.push(Span::raw("  "));
        }

        frame.render_widget(Line::from(spans), layout[1]);
        frame.render_widget(title_span, layout[0]);
        // frame.render_widget(content, layout[1]);
    }

    fn render_color_slider(
        &self,
        frame: &mut Frame,
        area: Rect,
        state: &mut ColorPickerState,
        channel: ColorChannel,
    ) {
        let (title, is_active) = match channel {
            ColorChannel::Red => ("R", state.active_input == ColorPickerInput::R),
            ColorChannel::Green => ("G", state.active_input == ColorPickerInput::G),
            ColorChannel::Blue => ("B", state.active_input == ColorPickerInput::B),
        };

        let layout: [Rect; 3] = Layout::horizontal([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        let title_span = self
            .get_title_span(title, is_active)
            .into_left_aligned_line();

        match channel {
            ColorChannel::Red => state.r_color.render(frame, layout[2]),
            ColorChannel::Green => state.g_color.render(frame, layout[2]),
            ColorChannel::Blue => state.b_color.render(frame, layout[2]),
        };

        frame.render_widget(title_span, layout[0]);
    }

    fn render_color_swatch(&self, frame: &mut Frame, area: Rect, state: &ColorPickerState) {
        let swatch_width: usize = 12; // Number of blocks to display in the swatch
        let rect = area.centered_horizontally(Constraint::Length(swatch_width as u16));

        let color = ratatui::style::Color::Rgb(
            state.r_color.value as u8,
            state.g_color.value as u8,
            state.b_color.value as u8,
        );

        let swatch_span = Span::from("█".repeat(swatch_width)).fg(color);

        frame.render_widget(swatch_span, rect);
    }

    fn get_actions_text<'a>(&self, state: &ColorPickerState) -> Line<'a> {
        let mut lines = Vec::new();

        for action in self.actions(state) {
            let key = Span::from(format!("[{}]", action.key)).style(TEXT_STYLE_ACTIVE);
            let action_text = Span::from(action.action).style(TEXT_STYLE_DIM);

            let line =
                Line::from(vec![key, Span::raw(" "), action_text, Span::raw("  ")]).left_aligned();
            lines.push(line);
        }

        return lines.into_iter().fold(Line::default(), |mut acc, line| {
            acc.extend(line);
            acc
        });
    }

    fn actions(&self, state: &ColorPickerState) -> Vec<Action> {
        let mut actions = vec![Action::new("Esc", "Close"), Action::new("↑/↓", "Move")];

        match state.active_input {
            ColorPickerInput::Preset => actions.push(Action::new("←/→", "Change Preset")),
            ColorPickerInput::R => actions.push(Action::new("←/→", "Change Red Value")),
            ColorPickerInput::G => actions.push(Action::new("←/→", "Change Green Value")),
            ColorPickerInput::B => actions.push(Action::new("←/→", "Change Blue Value")),
        }

        actions
    }
}

impl RenderOption for Preset {
    fn render_option(&self, is_selected: bool) -> ratatui::text::Span<'static> {
        let content = if is_selected {
            "[▣ ██]"
        } else {
            "██"
        };
        let mut span = ratatui::text::Span::from(content);
        span.style.fg = Some(self.0);
        span
    }
}
