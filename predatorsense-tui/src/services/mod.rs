use crate::tui::state::{ApplicationState, lighting::LightingPageState};
use predatorsense::{
    commands::{battery::ChargingLimit, lighting::LightingCommand, performance::PerfMode},
    config::Config,
    error::Error,
    profile::LightingProfile,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Applied {
    perf: PerfMode,
    battery: ChargingLimit,
    lighting: Vec<LightingCommand>,
}

impl Applied {
    pub fn desired(state: &ApplicationState) -> Self {
        // handle the perf_mode
        // get the selected perf mode from the state
        let perf = match state.perf_page_state.mode_input.state.selected() {
            Some(index) => PerfMode::from_repr(index as u8).unwrap_or(PerfMode::default()),
            None => PerfMode::default(),
        };

        let battery_limit = ChargingLimit {
            enabled: state.battery_page_state.enable_charging_limit.value,
            upper: state.battery_page_state.upper_charging_limit.value as u8,
            lower: state.battery_page_state.lower_charging_limit.value as u8,
        };

        Applied {
            perf,
            battery: battery_limit,
            lighting: lighting_commands(&state.lighting_page_state),
        }
    }
}

fn lighting_commands(state: &LightingPageState) -> Vec<LightingCommand> {
    LightingProfile::from(state).commands()
}

// i need a function that gets feature report from the hid devices, returns an Applied and then to convert that into an ApplicationState object.

pub fn execute(
    old_state: Option<Applied>,
    new_state: Applied,
    config: &Config,
) -> Result<(), Error> {
    let Some(old_state) = old_state else {
        log::info!("No previous state known; asserting everything");
        new_state.perf.apply(config.system()?)?;
        new_state.battery.apply(config.system()?)?;
        for command in &new_state.lighting {
            command.apply(config.rgb()?)?;
        }
        return Ok(());
    };
    // compare the two and execute the necessary commands
    if old_state == new_state {
        return Ok(());
    }

    // Check perf
    if old_state.perf != new_state.perf {
        new_state.perf.apply(config.system()?)?;
    }

    if old_state.battery != new_state.battery {
        new_state.battery.apply(config.system()?)?;
    }

    for (old_command, new_command) in old_state.lighting.iter().zip(&new_state.lighting) {
        if old_command != new_command {
            new_command.apply(config.rgb()?)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use predatorsense::{
        commands::lighting::{Direction, Effect, Rgb, Speed, Target, Zone},
        effects::{EffectDirection, LightingEffect},
    };
    use ratatui::style::Color;

    // lighting_commands

    #[test]
    fn lighting_commands_cover_each_target_once() {
        let commands = lighting_commands(&LightingPageState::default());
        let targets: Vec<Target> = commands.iter().map(|c| c.target).collect();
        assert_eq!(
            targets,
            vec![
                Target::Keyboard,
                Target::BackLogo,
                Target::PerformanceModeButton
            ]
        );
    }

    #[test]
    fn lighting_commands_use_expected_zones() {
        let commands = lighting_commands(&LightingPageState::default());
        let zones: Vec<Zone> = commands.iter().map(|c| c.zone).collect();
        assert_eq!(zones, vec![Zone::All, Zone::None, Zone::None]);
    }

    #[test]
    fn lighting_commands_copy_keyboard_values() {
        let mut state = LightingPageState::default();
        state.keyboard.effect.select_value(LightingEffect::Wave);
        state
            .keyboard
            .direction
            .select_value(EffectDirection::RightToLeft);
        state.keyboard.brightness.set_value(40);
        state.keyboard.speed.set_value(5);
        state.keyboard.color.color = Color::Rgb(1, 2, 3);

        let keyboard = lighting_commands(&state)[0];

        assert_eq!(keyboard.effect, Effect::Wave);
        assert_eq!(keyboard.direction, Direction::Left);
        assert_eq!(keyboard.brightness, 40);
        assert_eq!(keyboard.speed, Speed::new(5));
        assert_eq!(keyboard.color, Rgb { r: 1, g: 2, b: 3 });
    }

    // Applied::desired

    #[test]
    fn desired_maps_selected_perf_index() {
        let mut state = ApplicationState::default();
        state.perf_page_state.mode_input.state.select(Some(3));
        assert_eq!(Applied::desired(&state).perf, PerfMode::Quiet);
    }

    #[test]
    fn desired_out_of_range_perf_index_falls_back_to_default() {
        let mut state = ApplicationState::default();
        state.perf_page_state.mode_input.state.select(Some(10));
        assert_eq!(Applied::desired(&state).perf, PerfMode::default());
    }

    #[test]
    fn desired_no_perf_selection_falls_back_to_default() {
        let mut state = ApplicationState::default();
        state.perf_page_state.mode_input.state.select(None);
        assert_eq!(Applied::desired(&state).perf, PerfMode::default());
    }

    #[test]
    fn desired_copies_battery_fields() {
        let mut state = ApplicationState::default();
        state.battery_page_state.enable_charging_limit.value = true;
        state.battery_page_state.upper_charging_limit.value = 80;
        state.battery_page_state.lower_charging_limit.value = 60;

        assert_eq!(
            Applied::desired(&state).battery,
            ChargingLimit {
                enabled: true,
                upper: 80,
                lower: 60
            }
        );
    }

    // execute (offline config: any attempt to touch hardware is an error)

    fn applied() -> Applied {
        Applied::desired(&ApplicationState::default())
    }

    #[test]
    fn execute_unchanged_state_touches_no_device() {
        let result = execute(Some(applied()), applied(), &Config::offline());
        assert!(result.is_ok());
    }

    #[test]
    fn execute_without_old_state_applies_everything() {
        let result = execute(None, applied(), &Config::offline());
        assert!(matches!(result, Err(Error::Unavailable { .. })));
    }

    #[test]
    fn execute_changed_lighting_reports_device_error() {
        let mut new = applied();
        new.lighting[0].brightness = 1;
        let result = execute(Some(applied()), new, &Config::offline());
        assert!(matches!(
            result,
            Err(Error::Unavailable { role: "rgb", .. })
        ));
    }

    #[test]
    fn execute_changed_perf_reports_device_error() {
        let mut new = applied();
        new.perf = PerfMode::Turbo;
        let result = execute(Some(applied()), new, &Config::offline());
        assert!(
            matches!(result, Err(Error::Unavailable { .. })),
            "got {result:?}"
        );
    }

    #[test]
    fn execute_changed_battery_reports_device_error() {
        let mut new = applied();
        new.battery.enabled = !new.battery.enabled;
        let result = execute(Some(applied()), new, &Config::offline());
        assert!(
            matches!(result, Err(Error::Unavailable { .. })),
            "got {result:?}"
        );
    }
}
