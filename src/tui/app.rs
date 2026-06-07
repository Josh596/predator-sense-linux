use strum_macros::{Display, EnumIter, FromRepr, VariantArray, VariantNames};

mod lighting {
    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub struct LightingState {
        pub active_pane: LightingPagePane,
        pub active_input: LightingPageInput,
    }

    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub enum LightingPageInput {
        #[default]
        Target,
        Zone,
        Effect,
        Color,
        Speed,
        Direction,
        Timeout, // this uses a different hid device. i wonder how i'll handle this.
    }

    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub enum LightingPagePane {
        #[default]
        Left,
        Right,
    }
}

pub mod battery {
    use strum_macros::{EnumCount, EnumIter, FromRepr};

    use crate::commands::battery::ChargingLimit;

    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub struct BatteryState {
        pub battery_charging: ChargingLimit,
        pub active_input: BatteryPageInput,
    }

    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy, EnumCount, FromRepr)]
    pub enum BatteryPageInput {
        #[default]
        EnableCharging,
        MaxLimit,
        MinLimit,
    }
}

pub mod performance {
    use ratatui::widgets::ListState;
    use strum::IntoEnumIterator;

    use crate::commands::power::PerfMode;

    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub struct PerformanceState {
        // I am using a struct in case I need to add extra things, if i don't need anything else, i should change to just the enum PerfMode
        pub perfomance_mode: PerfMode,
        pub mode_list_state: ListState,
    }

    impl PerformanceState {
        pub fn set_performance_mode(&mut self) {
            if let Some(i) = self.mode_list_state.selected() {
                let mode = PerfMode::iter().nth(i);

                // TODO: HANDLE EXCEPTIONS
                self.perfomance_mode = mode.unwrap();
            }
        }
    }

    impl Default for PerformanceState {
        fn default() -> Self {
            let mut state = ListState::default();

            // 2. Set the initial selected item to index 0
            state.select(Some(0));

            Self {
                perfomance_mode: PerfMode::default(),
                mode_list_state: state,
            }
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub enum RunningState {
    Done,
    #[default]
    Running,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Display, FromRepr, VariantNames)]
pub enum Page {
    #[default]
    Performance,
    Battery,
    Lighting,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct ApplicationState {
    pub active_page: Page,
    pub running_state: RunningState,
    pub performance_state: performance::PerformanceState,
    pub battery_state: battery::BatteryState,
    pub lighting_state: lighting::LightingState,
}
