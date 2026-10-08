use predatorsense::commands::battery::ChargingLimit;
use strum::{EnumCount, EnumIter, FromRepr};

use crate::tui::state::fields::{BooleanField, Field, NumericField};

#[derive(Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy, EnumCount)]
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
    pub fn load_from(&mut self, limit: ChargingLimit) {
        self.enable_charging_limit.value = limit.enabled;
        self.upper_charging_limit.value = limit.upper.into();
        self.lower_charging_limit.value = limit.lower.into();
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
        // let charging_limit = ChargingLimit
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
