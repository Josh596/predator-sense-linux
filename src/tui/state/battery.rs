use strum_macros::{EnumIter, FromRepr};

use crate::tui::{
    components::inputs::Input,
    state::fields::{BooleanField, Field, NumericField},
};

#[derive(Default, EnumIter, FromRepr, PartialEq, Eq, Clone, Copy)]
pub enum BatteryPageInput {
    #[default]
    EnableChargingLimit,
    UpperChargingLimit,
    LowerChargingLimit,
}

pub struct BatteryPageState {
    pub active_input: BatteryPageInput,
    pub enable_charging_limit: BooleanField,
    pub upper_charging_limit: NumericField,
    pub lower_charging_limit: NumericField,
}

impl BatteryPageState {
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
