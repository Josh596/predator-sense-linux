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
    use crate::commands::power::PerfMode;

    #[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
    pub struct PerformancePage {
        // I am using a struct in case I need to add extra things, if i don't need anything else, i should change to just the enum PerfMode
        pub perfomance_mode: PerfMode,
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]

enum RunningState {
    Done,
    #[default]
    Running,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
enum Pages {
    Battery,
    Lighting,
    #[default]
    Perfomance,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
struct ApplicationState {
    active_page: Pages,
    running_state: RunningState,
    performance_page: performance::PerformancePage,
    battery_page: battery::BatteryPage,
    lighting_page: lighting::LightingPage,
}
