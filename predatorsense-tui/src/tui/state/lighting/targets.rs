use ratatui::style::Color;
use strum::IntoEnumIterator;

use crate::tui::state::{
    fields::{ColorField, Field, OptionField, SliderField},
    lighting::{
        LightingPageInput,
        effects::{EffectDirection, LightingEffect},
    },
};

const PLACEHOLDER_COLOR: Color = Color::Rgb(255, 255, 255);
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
            speed: SliderField::new(0, 9, 1, ""),
            brightness: SliderField::new(0, 100, 1, "%"),
            color: ColorField {
                color: PLACEHOLDER_COLOR,
            },
        }
    }
}

impl KeyboardState {
    pub fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Effect => &mut self.effect,
            LightingPageInput::EffectDirection => &mut self.direction,
            LightingPageInput::Speed => &mut self.speed,
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let caps = self.effect.value().capabilities();

        let mut inputs = vec![LightingPageInput::Effect];
        if caps.has_speed {
            inputs.push(LightingPageInput::Speed);
        }
        if caps.has_direction {
            inputs.push(LightingPageInput::EffectDirection);
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
                LightingEffect::Off,
                LightingEffect::Static,
                LightingEffect::Breathing,
                LightingEffect::Neon,
            ]),
            speed: SliderField::new(0, 9, 1, ""),
            brightness: SliderField::new(0, 100, 1, "%"),
            color: ColorField {
                color: PLACEHOLDER_COLOR,
            },
        }
    }
}
impl LogoState {
    pub fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Effect => &mut self.effect,
            LightingPageInput::Speed => &mut self.speed,
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Effect];
        if self.effect.value().capabilities().has_speed {
            inputs.push(LightingPageInput::Speed);
        }
        inputs.push(LightingPageInput::Brightness);
        inputs.push(LightingPageInput::Color);
        inputs
    }
}

pub struct ModeButtonState {
    pub brightness: SliderField,
    pub effect: OptionField<LightingEffect>,
    pub speed: SliderField,
    pub color: ColorField,
}

impl Default for ModeButtonState {
    fn default() -> Self {
        Self {
            effect: OptionField::new(vec![
                LightingEffect::Off,
                LightingEffect::Static,
                LightingEffect::Breathing,
                LightingEffect::Neon,
            ]),
            speed: SliderField::new(0, 9, 1, ""),
            brightness: SliderField::new(0, 100, 1, "%"),
            color: ColorField {
                color: PLACEHOLDER_COLOR,
            },
        }
    }
}
impl ModeButtonState {
    pub fn field_mut(&mut self, input: LightingPageInput) -> Option<&mut dyn Field> {
        Some(match input {
            LightingPageInput::Effect => &mut self.effect,
            LightingPageInput::Speed => &mut self.speed,
            LightingPageInput::Brightness => &mut self.brightness,
            LightingPageInput::Color => &mut self.color,
            _ => return None,
        })
    }

    pub fn visible_inputs(&self) -> Vec<LightingPageInput> {
        let mut inputs = vec![LightingPageInput::Effect];
        if self.effect.value().capabilities().has_speed {
            inputs.push(LightingPageInput::Speed);
        }
        inputs.push(LightingPageInput::Brightness);
        inputs.push(LightingPageInput::Color);
        inputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LightingPageInput::{Brightness, Color, Effect, EffectDirection, Speed, Target};

    // KeyboardState

    fn keyboard_with(effect: LightingEffect) -> KeyboardState {
        let mut state = KeyboardState::default();
        state.effect.select_value(effect);
        state
    }

    #[test]
    fn keyboard_field_mut_target_is_none() {
        assert!(KeyboardState::default().field_mut(Target).is_none());
    }

    #[test]
    fn keyboard_field_mut_has_every_other_input() {
        let mut state = KeyboardState::default();
        for input in [Effect, EffectDirection, Speed, Brightness, Color] {
            assert!(state.field_mut(input).is_some());
        }
    }

    #[test]
    fn keyboard_field_mut_speed_and_brightness_are_not_swapped() {
        let mut state = KeyboardState::default();
        state.field_mut(Speed).unwrap().increment();
        assert_eq!((state.speed.value, state.brightness.value), (1, 0));
        state.field_mut(Brightness).unwrap().increment();
        assert_eq!((state.speed.value, state.brightness.value), (1, 1));
    }

    #[test]
    fn keyboard_visible_inputs_static_hides_speed_and_direction() {
        let state = keyboard_with(LightingEffect::Static);
        assert_eq!(state.visible_inputs(), vec![Effect, Brightness, Color]);
    }

    #[test]
    fn keyboard_visible_inputs_breathing_shows_speed() {
        let state = keyboard_with(LightingEffect::Breathing);
        assert_eq!(state.visible_inputs(), vec![Effect, Speed, Brightness, Color]);
    }

    #[test]
    fn keyboard_visible_inputs_wave_shows_speed_and_direction() {
        let state = keyboard_with(LightingEffect::Wave);
        assert_eq!(
            state.visible_inputs(),
            vec![Effect, Speed, EffectDirection, Brightness, Color]
        );
    }

    // LogoState

    fn logo_with(effect: LightingEffect) -> LogoState {
        let mut state = LogoState::default();
        state.effect.select_value(effect);
        state
    }

    #[test]
    fn logo_field_mut_target_and_direction_are_none() {
        let mut state = LogoState::default();
        assert!(state.field_mut(Target).is_none());
        assert!(state.field_mut(EffectDirection).is_none());
    }

    #[test]
    fn logo_field_mut_speed_and_brightness_are_not_swapped() {
        let mut state = LogoState::default();
        state.field_mut(Speed).unwrap().increment();
        assert_eq!((state.speed.value, state.brightness.value), (1, 0));
        state.field_mut(Brightness).unwrap().increment();
        assert_eq!((state.speed.value, state.brightness.value), (1, 1));
    }

    #[test]
    fn logo_visible_inputs_static_hides_speed() {
        let state = logo_with(LightingEffect::Static);
        assert_eq!(state.visible_inputs(), vec![Effect, Brightness, Color]);
    }

    #[test]
    fn logo_visible_inputs_breathing_shows_speed() {
        let state = logo_with(LightingEffect::Breathing);
        assert_eq!(state.visible_inputs(), vec![Effect, Speed, Brightness, Color]);
    }

    // ModeButtonState

    fn mode_button_with(effect: LightingEffect) -> ModeButtonState {
        let mut state = ModeButtonState::default();
        state.effect.select_value(effect);
        state
    }

    #[test]
    fn mode_button_field_mut_target_and_direction_are_none() {
        let mut state = ModeButtonState::default();
        assert!(state.field_mut(Target).is_none());
        assert!(state.field_mut(EffectDirection).is_none());
    }

    #[test]
    fn mode_button_field_mut_speed_and_brightness_are_not_swapped() {
        let mut state = ModeButtonState::default();
        state.field_mut(Speed).unwrap().increment();
        assert_eq!((state.speed.value, state.brightness.value), (1, 0));
        state.field_mut(Brightness).unwrap().increment();
        assert_eq!((state.speed.value, state.brightness.value), (1, 1));
    }

    #[test]
    fn mode_button_visible_inputs_static_hides_speed() {
        let state = mode_button_with(LightingEffect::Static);
        assert_eq!(state.visible_inputs(), vec![Effect, Brightness, Color]);
    }

    #[test]
    fn mode_button_visible_inputs_breathing_shows_speed() {
        let state = mode_button_with(LightingEffect::Breathing);
        assert_eq!(state.visible_inputs(), vec![Effect, Speed, Brightness, Color]);
    }
}
