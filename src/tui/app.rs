mod lighting {
    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub struct LightingPage {
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
    use crate::commands::battery::ChargingLimit;

    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub struct BatteryPage {
        pub battery_charging: ChargingLimit,
        pub active_input: BatteryPageInput,
    }

    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub enum BatteryPageInput {
        #[default]
        EnableCharging,
        MaxLimit,
        MinLimit,
    }
}

pub mod performance {
    use ratatui::widgets::ListState;

    use crate::commands::power::PerfMode;

    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub struct PerformancePage {
        // I am using a struct in case I need to add extra things, if i don't need anything else, i should change to just the enum PerfMode
        pub perfomance_mode: PerfMode,
        pub mode_list_state: ListState,
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
enum RunningState {
    Done,
    #[default]
    Running,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub enum ActivePage {
    Battery,
    Lighting,
    #[default]
    Perfomance,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct ApplicationState {
    pub active_page: ActivePage,
    pub running_state: RunningState,
    pub performance_page: performance::PerformancePage,
    pub battery_page: battery::BatteryPage,
    pub lighting_page: lighting::LightingPage,
}
