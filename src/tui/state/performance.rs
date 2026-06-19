use ratatui::widgets::ListState;
use strum::{IntoEnumIterator, VariantNames};

use crate::{commands::power::PerfMode, tui::state::fields::ListField};

pub struct PerformancePageState {
    pub mode_input: ListField,
}

impl Default for PerformancePageState {
    fn default() -> Self {
        let options: Vec<String> = PerfMode::VARIANTS
            .iter()
            .map(|variant| String::from(*variant))
            .collect();

        let mut state = ListState::default();

        // Select the first item for now;
        state.select(Some(0));

        let field = ListField { options, state };

        Self { mode_input: field }
    }
}
