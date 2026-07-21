use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Spacing},
    style::Stylize,
    text::Span,
};

use crate::tui::{
    components::inputs::{Input, InputType, WidgetInput},
    state::{
        ApplicationState,
        lighting::LightingPageInput,
    },
    style::ACCENT_COLOR,
    utils::Action,
    view::page::PageView,
};

#[derive(Default)]
pub struct LightingPage;

impl LightingPage {
    fn cursor(&self, is_focused: bool) -> String {
        if is_focused {
            return "▸".to_string();
        }
        return "".to_string();
    }

    fn render_page(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        type RenderFn = fn(&LightingPage, &mut Frame, Rect, &mut ApplicationState);
        // hmm. do i switch to closures here?
        let input_render_function: HashMap<LightingPageInput, RenderFn> = HashMap::from([
            (
                LightingPageInput::Target,
                LightingPage::render_target_input as RenderFn,
            ),
            (LightingPageInput::Effect, LightingPage::render_effect_input),
            (
                LightingPageInput::Brightness,
                LightingPage::render_brightness_input,
            ),
            (LightingPageInput::Speed, LightingPage::render_speed_input),
            (
                LightingPageInput::EffectDirection,
                LightingPage::render_effect_direction_input,
            ),
            (LightingPageInput::Color, LightingPage::render_color_input),
        ]);
        let layout: [Rect; 6] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .spacing(Spacing::Space(1))
        .areas(rect);

        // Get all visible inputs based on the current state. I need to define a hashmap that links each input to its render function.
        // But is it better to define the hashmap inside the function or outside?
        for (index, input) in state
            .lighting_page_state
            .visible_inputs()
            .iter()
            .enumerate()
        {
            if let Some(render_fn) = input_render_function.get(&input) {
                render_fn(self, frame, layout[index], state);
            }
        }
    }

    fn render_effect_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        self.render_input(
            frame,
            rect,
            "Effect",
            state.lighting_page_state.active_input == LightingPageInput::Effect,
            InputType::Line(&mut state.lighting_page_state.effect_input),
        );
    }

    fn render_effect_direction_input(
        &self,
        frame: &mut Frame,
        rect: Rect,
        state: &mut ApplicationState,
    ) {
        self.render_input(
            frame,
            rect,
            "Direction",
            state.lighting_page_state.active_input == LightingPageInput::EffectDirection,
            InputType::Line(&mut state.lighting_page_state.effect_direction_input),
        );
    }
    fn render_brightness_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        //
        self.render_input(
            frame,
            rect,
            "Brightness",
            state.lighting_page_state.active_input == LightingPageInput::Brightness,
            InputType::Widget(&mut state.lighting_page_state.brightness_input),
        );
    }

    fn render_speed_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        //
        self.render_input(
            frame,
            rect,
            "Speed",
            state.lighting_page_state.active_input == LightingPageInput::Speed,
            InputType::Widget(&mut state.lighting_page_state.speed_input),
        );
    }

    fn render_color_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        self.render_input(
            frame,
            rect,
            "Color",
            state.lighting_page_state.active_input == LightingPageInput::Color,
            InputType::Line(&mut state.lighting_page_state.color_input),
        );
    }

    fn render_input(
        &self,
        frame: &mut Frame,
        rect: Rect,
        title: &'static str,
        is_active: bool,
        input: InputType,
    ) {
        let layout: [Rect; 2] =
            Layout::horizontal([Constraint::Length(20), Constraint::Fill(1)]).areas(rect);
        let title = format!("{} {}", self.cursor(is_active), title);
        let mut title_span = Span::from(title);

        if is_active {
            title_span = title_span.fg(ACCENT_COLOR);
        }

        // Render the title
        frame.render_widget(title_span, layout[0]);

        match input {
            InputType::Line(field) => {
                frame.render_widget(field.render(), layout[1]);
            }
            InputType::Widget(widget) => widget.render(frame, layout[1]),
        }
    }
    fn render_target_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        self.render_input(
            frame,
            rect,
            "Target",
            state.lighting_page_state.active_input == LightingPageInput::Target,
            InputType::Line(&mut state.lighting_page_state.target_input),
        );
    }
}

impl PageView for LightingPage {
    fn render(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        let layout: [Rect; 1] = Layout::vertical([Constraint::Fill(1)]).areas(rect);
        self.render_page(frame, layout[0], state);
    }

    fn actions(&self, state: &ApplicationState) -> Vec<Action> {
        let mut actions = vec![Action::new("↑↓", "Move")];
        // check the active input
        match state.lighting_page_state.active_input {
            LightingPageInput::Effect => actions.push(Action::new("←→", "Change Effect")),
            LightingPageInput::Brightness => actions.push(Action::new("←→", "Change Brightness")),
            LightingPageInput::Speed => actions.push(Action::new("←→", "Change Speed")),
            LightingPageInput::EffectDirection => {
                actions.push(Action::new("←→", "Change Direction"))
            }
            LightingPageInput::Color => actions.push(Action::new("Space", "Toggle Color Picker")),
            _ => {}
        }

        actions
    }
}
