use ratatui::style::Color;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter, FromRepr, VariantNames};

use crate::tui::state::{
    color_picker::ColorPickerState,
    fields::{ColorField, Field, OptionField, SliderField},
};

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

pub struct KeyboardState {
    pub effect: OptionField<LightingEffect>,
    pub direction: OptionField<EffectDirection>,
    pub speed: SliderField,
    pub brightness: SliderField,
    pub color: ColorField,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            effect: OptionField::new(LightingEffect::iter().collect()),
            direction: OptionField::new(EffectDirection::iter().collect()),
            speed: SliderField {
                value: 1,
                min: 1,
                max: 9,
                step: 2,
                unit: "",
                ranges: vec![],
            },
            brightness: SliderField {
                value: 100,
                min: 0,
                max: 100,
                step: 1,
                unit: "%",
                ranges: vec![],
            },
            color: ColorField {
                color: Color::Rgb(255, 0, 0),
            },
        }
    }
}

impl KeyboardState {
    fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Effect => &mut self.effect,
            LightingPageInput::EffectDirection => &mut self.direction,
            LightingPageInput::Speed => &mut self.speed,
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Effect];
        match self.effect.value().capabilities() {
            LightingEffectConfig {
                has_speed: true,
                has_direction: true,
            } => {
                inputs.push(LightingPageInput::Speed);
                inputs.push(LightingPageInput::EffectDirection);
            }
            LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            } => {
                inputs.push(LightingPageInput::Speed);
            }
            LightingEffectConfig {
                has_speed: false,
                has_direction: true,
            } => {
                inputs.push(LightingPageInput::EffectDirection);
            }
            LightingEffectConfig {
                has_speed: false,
                has_direction: false,
            } => {}
        }

        inputs.push(LightingPageInput::Brightness);
        inputs.push(LightingPageInput::Color);
        inputs
    }
}

pub struct LogoState {
    pub effect: OptionField<LightingEffect>,
    pub speed: SliderField,
    pub brightness: SliderField,
    pub color: ColorField,
}

impl Default for LogoState {
    fn default() -> Self {
        Self {
            effect: OptionField::new(vec![
                LightingEffect::Static,
                LightingEffect::Breathing,
                LightingEffect::Neon,
            ]),
            speed: SliderField {
                value: 1,
                min: 1,
                max: 9,
                step: 2,
                unit: "",
                ranges: vec![],
            },
            brightness: SliderField {
                value: 100,
                min: 0,
                max: 100,
                step: 1,
                unit: "%",
                ranges: vec![],
            },
            color: ColorField {
                color: Color::Rgb(255, 0, 0),
            },
        }
    }
}
impl LogoState {
    fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Effect => &mut self.effect,
            LightingPageInput::Speed => &mut self.speed,
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Effect];
        if self.effect.value() != LightingEffect::Static {
            inputs.push(LightingPageInput::Speed);
        }
        inputs.push(LightingPageInput::Brightness);
        inputs.push(LightingPageInput::Color);
        inputs
    }
}

pub struct TurboButtonState {
    pub brightness: SliderField,
    pub color: ColorField,
}

impl Default for TurboButtonState {
    fn default() -> Self {
        Self {
            brightness: SliderField {
                value: 100,
                min: 0,
                max: 100,
                step: 1,
                unit: "%",
                ranges: vec![],
            },
            color: ColorField {
                color: Color::Rgb(255, 0, 0),
            },
        }
    }
}
impl TurboButtonState {
    fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    fn visible_inputs(&self) -> Vec<LightingPageInput> {
        vec![LightingPageInput::Brightness, LightingPageInput::Color]
    }
}

#[derive(
    Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames, Display,
)]
pub enum Target {
    #[default]
    Keyboard,
    Logo,
    #[strum(serialize = "Turbo Button")]
    TurboButton,
}

#[derive(
    Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames, Display,
)]
pub enum EffectDirection {
    #[default]
    #[strum(serialize = "Left to Right")]
    LeftToRight,
    #[strum(serialize = "Right to Left")]
    RightToLeft,
}

struct LightingEffectConfig {
    pub has_speed: bool,
    pub has_direction: bool,
}

#[derive(
    Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, VariantNames, Display,
)]
pub enum LightingEffect {
    #[default]
    Off,
    Static,
    Breathing,
    Neon,
    Wave,
    Zoom,
    Snake,
    Disco,
    Ripple,
}

impl LightingEffect {
    pub fn capabilities(&self) -> LightingEffectConfig {
        match self {
            LightingEffect::Off => LightingEffectConfig {
                has_speed: false,
                has_direction: false,
            },
            LightingEffect::Static => LightingEffectConfig {
                has_speed: false,
                has_direction: false,
            },
            LightingEffect::Breathing => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Neon => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Wave => LightingEffectConfig {
                has_speed: true,
                has_direction: true,
            },

            LightingEffect::Zoom => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Disco => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Snake => LightingEffectConfig {
                has_speed: true,
                has_direction: false,
            },
            LightingEffect::Ripple => LightingEffectConfig {
                has_speed: true,
                has_direction: true,
            },
        }
    }
}
pub struct LightingPageState {
    pub active_input: LightingPageInput,
    pub target_input: OptionField<Target>,
    pub keyboard: KeyboardState,
    pub logo: LogoState,
    pub turbo_button: TurboButtonState,
    pub color_picker: Option<ColorPickerState>,
}

impl LightingPageState {
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

            Target::TurboButton => self.turbo_button.field_mut(self.active_input),
        };

        field.expect("field is not valid for the selected target")
    }

    pub fn active_color_field_mut(&mut self) -> &mut ColorField {
        match self.selected_target() {
            Target::Keyboard => &mut self.keyboard.color,
            Target::Logo => &mut self.logo.color,
            Target::TurboButton => &mut self.turbo_button.color,
        }
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Target];

        inputs.extend(match self.selected_target() {
            Target::Keyboard => self.keyboard.visible_inputs(),
            Target::Logo => self.logo.visible_inputs(),
            Target::TurboButton => self.turbo_button.visible_inputs(),
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
        Self {
            active_input: LightingPageInput::default(),
            keyboard: KeyboardState::default(),
            logo: LogoState::default(),
            turbo_button: TurboButtonState::default(),
            target_input: OptionField::new(Target::iter().collect()),
            color_picker: None,
        }
    }
}
