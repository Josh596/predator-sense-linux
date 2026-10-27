use predatorsense::commands::battery::ChargingLimit;
use strum::{EnumCount, EnumIter, FromRepr};

use crate::tui::state::fields::{BooleanField, Field, NumericField};

#[derive(Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount, Debug)]
pub enum BatteryPageInput {
    #[default]
    EnableChargingLimit,
    UpperChargingLimit,
    LowerChargingLimit,
}

impl BatteryPageInput {
    /// Move the cursor to the previous input, clamping at the first.                                                                                                                                                                                          
    pub fn prev(self) -> Self {
        let index = (self as usize).saturating_sub(1);
        Self::from_repr(index).unwrap_or(self)
    }

    /// Move the cursor to the next input, clamping at the last.                                                                                                                                                                                               
    pub fn next(self) -> Self {
        let index = (self as usize + 1).min(BatteryPageInput::COUNT - 1);
        Self::from_repr(index).unwrap_or(self)
    }
}

pub struct BatteryPageState {
    pub active_input: BatteryPageInput,
    pub enable_charging_limit: BooleanField,
    pub upper_charging_limit: NumericField,
    pub lower_charging_limit: NumericField,
}

impl BatteryPageState {
    pub fn new(limit: ChargingLimit) -> Self {
        Self {
            active_input: BatteryPageInput::EnableChargingLimit,
            enable_charging_limit: BooleanField {
                on_text: "Enabled",
                off_text: "Disabled",
                value: limit.enabled,
            },
            upper_charging_limit: NumericField {
                value: limit.upper.into(),
                unit: "%",
                min: 0,
                max: 100,
                ..Default::default()
            },
            lower_charging_limit: NumericField {
                value: limit.lower.into(),
                unit: "%",
                min: 0,
                max: 100,
                ..Default::default()
            },
        }
    }

    pub fn get_active_input_mut(&mut self) -> &mut dyn Field {
        match self.active_input {
            BatteryPageInput::EnableChargingLimit => &mut self.enable_charging_limit,
            BatteryPageInput::UpperChargingLimit => &mut self.upper_charging_limit,
            BatteryPageInput::LowerChargingLimit => &mut self.lower_charging_limit,
        }
    }
}
impl Default for BatteryPageState {
    fn default() -> Self {
        Self {
            active_input: BatteryPageInput::EnableChargingLimit,
            enable_charging_limit: BooleanField {
                on_text: "Enabled",
                off_text: "Disabled",
                value: false,
            },
            upper_charging_limit: NumericField {
                value: 100,
                unit: "%",
                min: 0,
                max: 100,
                ..Default::default()
            },
            lower_charging_limit: NumericField {
                value: 0,
                unit: "%",
                min: 0,
                max: 100,
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prev_moves_to_previous_input() {
        assert_eq!(
            BatteryPageInput::LowerChargingLimit.prev(),
            BatteryPageInput::UpperChargingLimit
        );
    }

    #[test]
    fn prev_clamps_at_first_input() {
        assert_eq!(
            BatteryPageInput::EnableChargingLimit.prev(),
            BatteryPageInput::EnableChargingLimit
        );
    }

    #[test]
    fn next_moves_to_next_input() {
        assert_eq!(
            BatteryPageInput::EnableChargingLimit.next(),
            BatteryPageInput::UpperChargingLimit
        );
    }

    #[test]
    fn next_clamps_at_last_input() {
        assert_eq!(
            BatteryPageInput::LowerChargingLimit.next(),
            BatteryPageInput::LowerChargingLimit
        );
    }

    #[test]
    fn new_copies_limit_values() {
        let state = BatteryPageState::new(ChargingLimit {
            enabled: true,
            upper: 80,
            lower: 60,
        });
        assert!(state.enable_charging_limit.value);
        assert_eq!(state.upper_charging_limit.value, 80);
        assert_eq!(state.lower_charging_limit.value, 60);
    }

    #[test]
    fn default_is_disabled_with_full_range() {
        let state = BatteryPageState::default();
        assert!(!state.enable_charging_limit.value);
        assert_eq!(state.upper_charging_limit.value, 100);
        assert_eq!(state.lower_charging_limit.value, 0);
    }

    fn state_at(active_input: BatteryPageInput) -> BatteryPageState {
        let mut state = BatteryPageState::new(ChargingLimit {
            enabled: false,
            upper: 80,
            lower: 60,
        });
        state.active_input = active_input;
        state
    }

    #[test]
    fn active_input_enable_returns_toggle_field() {
        let mut state = state_at(BatteryPageInput::EnableChargingLimit);
        state.get_active_input_mut().toggle();
        assert!(state.enable_charging_limit.value);
        assert_eq!(state.upper_charging_limit.value, 80);
        assert_eq!(state.lower_charging_limit.value, 60);
    }

    #[test]
    fn active_input_upper_returns_upper_field() {
        let mut state = state_at(BatteryPageInput::UpperChargingLimit);
        state.get_active_input_mut().increment();
        assert!(!state.enable_charging_limit.value);
        assert_eq!(state.upper_charging_limit.value, 81);
        assert_eq!(state.lower_charging_limit.value, 60);
    }

    #[test]
    fn active_input_lower_returns_lower_field() {
        let mut state = state_at(BatteryPageInput::LowerChargingLimit);
        state.get_active_input_mut().increment();
        assert!(!state.enable_charging_limit.value);
        assert_eq!(state.upper_charging_limit.value, 80);
        assert_eq!(state.lower_charging_limit.value, 61);
    }
}
