use strum_macros::{Display, EnumCount, EnumIter, FromRepr, VariantNames};

pub mod fields;

pub mod battery;
pub mod performance;
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(
    Debug, Default, Eq, PartialEq, Clone, Copy, Display, FromRepr, VariantNames, EnumIter, EnumCount,
)]
pub enum Page {
    #[default]
    Dashboard,
    Performance,
    Battery,
    Lighting,
}

#[derive(Default)]
pub struct ApplicationState {
    pub running_state: RunningState,
    pub active_page: Page,
    pub battery_page_state: battery::BatteryPageState,
    pub perf_page_state: performance::PerformancePageState,
}

impl ApplicationState {}
