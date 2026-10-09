use ratatui::widgets::ListState;
use strum::{IntoEnumIterator, VariantNames};

use crate::tui::state::fields::ListField;
use predatorsense::commands::performance::PerfMode;
pub struct PerformancePageState {
    pub mode_input: ListField,
}

impl PerformancePageState {
    pub fn new(mode: PerfMode) -> Self {
        let options = PerfMode::VARIANTS.iter().map(|v| v.to_string()).collect();

        let mut state = ListState::default();
        state.select(PerfMode::iter().position(|m| m == mode));

        Self {
            mode_input: ListField { options, state },
        }
    }
}

impl Default for PerformancePageState {
    fn default() -> Self {
        Self::new(PerfMode::default())
    }
}
