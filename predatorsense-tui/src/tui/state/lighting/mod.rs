use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, FromRepr, VariantNames};

use crate::tui::state::{
    color_picker::ColorPickerState,
    fields::{ColorField, Field, OptionField},
    lighting::{
        profile::LightingProfile,
        targets::{KeyboardState, LogoState, ModeButtonState},
    },
};

pub mod effects;
pub mod profile;
pub mod targets;

#[derive(
    Debug, Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames, Display,
)]
pub enum Target {
    #[default]
    Keyboard,
    Logo,
    #[strum(serialize = "Mode Button")]
    ModeButton,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, EnumCount, EnumIter, FromRepr, Hash)]
pub enum LightingPageInput {
    #[default]
    Target,
    Effect,
    Brightness,
    Speed,
    EffectDirection,
    Color,
}

pub struct LightingPageState {
    pub active_input: LightingPageInput,
    pub target_input: OptionField<Target>,
    pub keyboard: KeyboardState,
    pub logo: LogoState,
    pub mode_button: ModeButtonState,
    pub color_picker: Option<ColorPickerState>,
}

impl LightingPageState {
    pub fn new(profile: &LightingProfile) -> Self {
        let mut state = Self {
            active_input: LightingPageInput::default(),
            keyboard: KeyboardState::default(),
            logo: LogoState::default(),
            mode_button: ModeButtonState::default(),
            target_input: OptionField::new(Target::iter().collect()),
            color_picker: None,
        };

        profile.apply_to(&mut state);

        state
    }

    pub fn selected_target(&self) -> Target {
        self.target_input.value()
    }
    pub fn get_active_input_mut(&mut self) -> &mut dyn Field {
        if self.active_input == LightingPageInput::Target {
            return &mut self.target_input;
        }

        let field = match self.selected_target() {
            Target::Keyboard => self.keyboard.field_mut(self.active_input),

            Target::Logo => self.logo.field_mut(self.active_input),

            Target::ModeButton => self.mode_button.field_mut(self.active_input),
        };

        field.expect("field is not valid for the selected target")
    }

    pub fn active_color_field_mut(&mut self) -> &mut ColorField {
        match self.selected_target() {
            Target::Keyboard => &mut self.keyboard.color,
            Target::Logo => &mut self.logo.color,
            Target::ModeButton => &mut self.mode_button.color,
        }
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Target];

        inputs.extend(match self.selected_target() {
            Target::Keyboard => self.keyboard.visible_inputs(),
            Target::Logo => self.logo.visible_inputs(),
            Target::ModeButton => self.mode_button.visible_inputs(),
        });

        inputs
    }

    pub fn next_input(&self) -> LightingPageInput {
        let visible_inputs = self.visible_inputs();
        let current_index = visible_inputs
            .iter()
            .position(|&input| input == self.active_input)
            .unwrap_or(0);
        let next_index = (current_index + 1).min(visible_inputs.len() - 1);
        visible_inputs[next_index]
    }

    pub fn prev_input(&self) -> LightingPageInput {
        let visible_inputs = self.visible_inputs();
        let current_index = visible_inputs
            .iter()
            .position(|&input| input == self.active_input)
            .unwrap_or(0);
        let prev_index = current_index.saturating_sub(1);
        visible_inputs[prev_index]
    }
}

impl Default for LightingPageState {
    fn default() -> Self {
        Self::new(&LightingProfile::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::state::lighting::effects::LightingEffect;
    use ratatui::style::Color;

    fn state_for(target: Target) -> LightingPageState {
        let mut state = LightingPageState::default();
        state.target_input.select_value(target);
        state
    }

    // new

    #[test]
    fn new_applies_profile() {
        let profile: LightingProfile = toml::from_str("[logo]\nbrightness = 40\n").unwrap();
        let state = LightingPageState::new(&profile);
        assert_eq!(state.logo.brightness.value, 40);
    }

    // selected_target

    #[test]
    fn selected_target_follows_target_input() {
        assert_eq!(state_for(Target::Logo).selected_target(), Target::Logo);
    }

    // get_active_input_mut

    #[test]
    fn active_input_target_returns_target_field() {
        let mut state = LightingPageState::default();
        state.get_active_input_mut().increment();
        assert_eq!(state.selected_target(), Target::Logo);
    }

    #[test]
    fn active_input_uses_selected_targets_field() {
        let mut state = state_for(Target::Logo);
        state.active_input = LightingPageInput::Brightness;
        let keyboard_before = state.keyboard.brightness.value;

        state.get_active_input_mut().decrement();

        assert_eq!(state.logo.brightness.value, 99);
        assert_eq!(state.keyboard.brightness.value, keyboard_before);
    }

    // active_color_field_mut

    #[test]
    fn active_color_field_belongs_to_selected_target() {
        for target in [Target::Keyboard, Target::Logo, Target::ModeButton] {
            let mut state = state_for(target);
            state.active_color_field_mut().color = Color::Rgb(1, 2, 3);

            let colors = [
                (Target::Keyboard, state.keyboard.color.color),
                (Target::Logo, state.logo.color.color),
                (Target::ModeButton, state.mode_button.color.color),
            ];
            for (owner, color) in colors {
                assert_eq!(color == Color::Rgb(1, 2, 3), owner == target, "{target}: {owner}");
            }
        }
    }

    // visible_inputs

    #[test]
    fn visible_inputs_follow_selected_target() {
        let mut state = state_for(Target::Logo);
        state.logo.effect.select_value(LightingEffect::Breathing);
        let mut expected = vec![LightingPageInput::Target];
        expected.extend(state.logo.visible_inputs());
        assert_eq!(state.visible_inputs(), expected);
    }

    // next_input / prev_input

    #[test]
    fn next_input_moves_to_next_visible() {
        let state = LightingPageState::default();
        assert_eq!(state.next_input(), LightingPageInput::Effect);
    }

    #[test]
    fn next_input_stops_at_last_visible() {
        let mut state = LightingPageState::default();
        state.active_input = LightingPageInput::Color;
        assert_eq!(state.next_input(), LightingPageInput::Color);
    }

    #[test]
    fn prev_input_moves_to_previous_visible() {
        let mut state = LightingPageState::default();
        state.active_input = LightingPageInput::Effect;
        assert_eq!(state.prev_input(), LightingPageInput::Target);
    }

    #[test]
    fn prev_input_stops_at_first_visible() {
        let state = LightingPageState::default();
        assert_eq!(state.prev_input(), LightingPageInput::Target);
    }

    #[test]
    fn next_input_skips_hidden_speed() {
        let mut state = LightingPageState::default();
        state.keyboard.effect.select_value(LightingEffect::Static);
        state.active_input = LightingPageInput::Effect;
        assert_eq!(state.next_input(), LightingPageInput::Brightness);
    }

    #[test]
    fn next_input_from_hidden_input_restarts_from_first() {
        let mut state = LightingPageState::default();
        state.keyboard.effect.select_value(LightingEffect::Static);
        state.active_input = LightingPageInput::Speed; // not visible for Static
        assert_eq!(state.next_input(), LightingPageInput::Effect);
    }
}
