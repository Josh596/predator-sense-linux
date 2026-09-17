use ratatui::style::Color;

use crate::{
    commands::{
        battery::ChargingLimit,
        lighting::{Direction, Effect, LightingCommand, Rgb, Speed, Target, Zone},
        power::PerfMode,
    },
    tui::state::{
        ApplicationState,
        lighting::{EffectDirection, LightingEffect, Target as TargetState},
    },
};

fn get_effect_from_state(effect: &LightingEffect) -> Effect {
    match effect {
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

fn execute(
    old_state: Option<&ApplicationState>,
    new_state: &ApplicationState,
) -> Result<(), String> {
    let old_applied = old_state.map(|s| Applied::desired(s));
    let new_applied = Applied::desired(new_state);

    // compare the two and execute the necessary commands
    if old_applied != Some(new_applied.clone()) {
        // execute the commands
        // new_applied.perf.apply(device);
    }

    Ok(())
}
