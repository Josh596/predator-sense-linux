use ratatui::style::Color;

use crate::tui::state::{
    ApplicationState,
    lighting::{EffectDirection, LightingEffect, Target as TargetState},
};
use predatorsense::{
    commands::{
        battery::ChargingLimit,
        lighting::{Direction, Effect, LightingCommand, Rgb, Speed, Target, Zone},
        power::PerfMode,
    },
    config::Config,
    error::Error,
};

fn get_effect_from_state(effect: &LightingEffect) -> Effect {
    match effect {
        LightingEffect::Off => Effect::Off,
        LightingEffect::Static => Effect::Static,
        LightingEffect::Breathing => Effect::Breathing,
        LightingEffect::Neon => Effect::Neon,
        // LightingEffect::PowerProfile => Effect::PowerProfile,
        LightingEffect::Wave => Effect::Wave,
        LightingEffect::Ripple => Effect::Ripple,
        LightingEffect::Zoom => Effect::Zoom,
        LightingEffect::Snake => Effect::Snake,
        LightingEffect::Disco => Effect::Disco,
        // LightingEffect::Shifting => Effect::Shifting,
    }
}

fn get_color_from_state(color: &Color) -> Rgb {
    match color {
        Color::Rgb(r, g, b) => Rgb {
            r: *r as u8,
            g: *g as u8,
            b: *b as u8,
        },
        _ => Rgb { r: 0, g: 0, b: 0 },
    }
}

fn get_direction_from_state(direction: EffectDirection) -> Direction {
    match direction {
        EffectDirection::LeftToRight => Direction::Right,
        EffectDirection::RightToLeft => Direction::Left,
    }
}

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
            Some(index) => PerfMode::from_repr(index).unwrap_or(PerfMode::default()),
            None => PerfMode::default(),
        };

        let battery_limit = ChargingLimit {
            enabled: state.battery_page_state.enable_charging_limit.value,
            upper: state.battery_page_state.upper_charging_limit.value as u8,
            lower: state.battery_page_state.lower_charging_limit.value as u8,
        };

        // so each target and each zone is a different lighting command
        // so for active target, get the command for it. if the active target is Keyboard, then
        // it mighht have multiple zones, so we get the command for each zone and add it to the Vec
        // 1. Get the active target from the state
        let lighting_command = match state.lighting_page_state.selected_target() {
            TargetState::Keyboard => LightingCommand {
                target: Target::Keyboard,
                effect: get_effect_from_state(&state.lighting_page_state.keyboard.effect.value()),
                brightness: state.lighting_page_state.keyboard.brightness.value as u8,
                speed: Speed::new(state.lighting_page_state.keyboard.speed.value as u8),
                direction: get_direction_from_state(
                    state.lighting_page_state.keyboard.direction.value(),
                ),
                color: get_color_from_state(&state.lighting_page_state.keyboard.color.color),
                zone: Zone::All,
            },
            TargetState::TurboButton => LightingCommand {
                target: Target::PowerProfileButton,
                effect: Effect::Off,
                brightness: state.lighting_page_state.turbo_button.brightness.value as u8,
                speed: Speed::new(0),
                direction: Direction::None,
                color: get_color_from_state(&state.lighting_page_state.turbo_button.color.color),
                zone: Zone::None,
            },

            TargetState::Logo => LightingCommand {
                target: Target::BackLogo,
                effect: get_effect_from_state(&state.lighting_page_state.logo.effect.value()),
                brightness: state.lighting_page_state.logo.brightness.value as u8,
                speed: Speed::new(state.lighting_page_state.logo.speed.value as u8),
                direction: Direction::None,
                color: get_color_from_state(&state.lighting_page_state.logo.color.color),
                zone: Zone::None,
            },
        };

        Applied {
            perf,
            battery: battery_limit,
            lighting: vec![lighting_command],
        }
    }
}

// i need a function that gets feature report from the hid devices, returns an Applied and then to convert that into an ApplicationState object.

pub fn execute(old_state: Applied, new_state: Applied, config: &Config) -> Result<(), Error> {
    // compare the two and execute the necessary commands
    if old_state == new_state {
        return Ok(());
    }

    // Check perf
    if old_state.perf != new_state.perf {
        new_state.perf.apply(config.system()?);
    }

    if old_state.battery != new_state.battery {
        new_state.battery.apply(config.system()?);
    }

    if old_state.lighting != new_state.lighting {
        for command in new_state.lighting {
            log::info!("Applying lighting command");
            command.apply(config.rgb().unwrap());
        }
    }

    Ok(())
}
