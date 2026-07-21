use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Spacing},
    style::Stylize,
    text::Span,
};

use crate::tui::state::lighting::Target;
use crate::tui::{
    components::inputs::InputType,
    state::{
        ApplicationState,
        lighting::{
            LightingPageInput::{self},
            LightingPageState,
        },
    },
    style::ACCENT_COLOR,
    utils::Action,
    view::page::PageView,
};

#[derive(Default)]
pub struct LightingPage;

impl LightingPage {
    fn input_for<'a>(
        state: &'a mut LightingPageState,
        input: LightingPageInput,
    ) -> (&'static str, InputType<'a>) {
        use LightingPageInput as I;
        if input == I::Target {
            return ("Target", InputType::Line(&state.target_input));
        }
        match state.selected_target() {
            Target::Keyboard => {
                let kb = &mut state.keyboard;
                match input {
                    I::Effect => ("Effect", InputType::Line(&kb.effect)),
                    I::EffectDirection => ("Direction", InputType::Line(&kb.direction)),
                    I::Speed => ("Speed", InputType::Widget(&mut kb.speed)),
                    I::Brightness => ("Brightness", InputType::Widget(&mut kb.brightness)),
                    I::Color => ("Color", InputType::Line(&kb.color)),
                    _ => unreachable!(),
                }
            }
            Target::Logo => {
                /* same shape, minus Direction */
                let logo = &mut state.logo;
                match input {
                    I::Effect => ("Effect", InputType::Line(&logo.effect)),
                    I::Speed => ("Speed", InputType::Widget(&mut logo.speed)),
                    I::Brightness => ("Brightness", InputType::Widget(&mut logo.brightness)),
                    I::Color => ("Color", InputType::Line(&logo.color)),
                    _ => unreachable!(),
                }
            }
            Target::TurboButton => {
                let tb = &mut state.turbo_button;
                match input {
                    I::Brightness => ("Brightness", InputType::Widget(&mut tb.brightness)),
                    I::Color => ("Color", InputType::Line(&tb.color)),
                    _ => unreachable!(),
                }
                /* Brightness and Color only */
            }
        }
    }

    fn cursor(&self, is_focused: bool) -> String {
        if is_focused {
            return "▸".to_string();
        }
        return "".to_string();
    }

    fn render_page(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        let visible = state.lighting_page_state.visible_inputs();
        let layout = Layout::vertical(vec![Constraint::Length(2); visible.len()])
            .spacing(Spacing::Space(1))
            .split(rect);

        for (index, &input) in visible.iter().enumerate() {
            let is_active = state.lighting_page_state.active_input == input;
            let (label, input_type) = Self::input_for(&mut state.lighting_page_state, input);
            self.render_input(frame, layout[index], label, is_active, input_type);
        }
    }
    // fn render_effect_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
    //     self.render_input(
    //         frame,
    //         rect,
    //         "Effect",
    //         state.lighting_page_state.active_input == LightingPageInput::Effect,
    //         InputType::Line(&mut state.lighting_page_state.effect_input),
    //     );
    // }

    // fn render_effect_direction_input(
    //     &self,
    //     frame: &mut Frame,
    //     rect: Rect,
    //     state: &mut ApplicationState,
    // ) {
    //     self.render_input(
    //         frame,
    //         rect,
    //         "Direction",
    //         state.lighting_page_state.active_input == LightingPageInput::EffectDirection,
    //         InputType::Line(&mut state.lighting_page_state.effect_direction_input),
    //     );
    // }
    // fn render_brightness_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
    //     //
    //     self.render_input(
    //         frame,
    //         rect,
    //         "Brightness",
    //         state.lighting_page_state.active_input == LightingPageInput::Brightness,
    //         InputType::Widget(&mut state.lighting_page_state.brightness_input),
    //     );
    // }

    // fn render_speed_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
    //     //
    //     self.render_input(
    //         frame,
    //         rect,
    //         "Speed",
    //         state.lighting_page_state.active_input == LightingPageInput::Speed,
    //         InputType::Widget(&mut state.lighting_page_state.speed_input),
    //     );
    // }

    // fn render_color_input(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
    //     self.render_input(
    //         frame,
    //         rect,
    //         "Color",
    //         state.lighting_page_state.active_input == LightingPageInput::Color,
    //         InputType::Line(&mut state.lighting_page_state.color_input),
    //     );
    // }

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
