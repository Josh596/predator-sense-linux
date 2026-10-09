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
    Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames, Display,
)]
pub enum Target {
    #[default]
    Keyboard,
    Logo,
    #[strum(serialize = "Mode Button")]
    ModeButton,
}

#[derive(Default, PartialEq, Eq, Clone, Copy, EnumCount, EnumIter, FromRepr, Hash)]
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
