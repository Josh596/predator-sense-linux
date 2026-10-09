use ratatui::style::Color;

use crate::tui::state::{
    ApplicationState,
    lighting::{
        LightingPageState,
        effects::{EffectDirection, LightingEffect},
    },
};
use predatorsense::{
    commands::{
        battery::ChargingLimit,
        lighting::{Direction, Effect, LightingCommand, Rgb, Speed, Target, Zone},
        performance::PerfMode,
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
    vec![
        LightingCommand {
            target: Target::Keyboard,
            effect: get_effect_from_state(&state.keyboard.effect.value()),
            brightness: state.keyboard.brightness.value as u8,
            speed: Speed::new(state.keyboard.speed.value as u8),
            direction: get_direction_from_state(state.keyboard.direction.value()),
            color: get_color_from_state(&state.keyboard.color.color),
            zone: Zone::All,
        },
        LightingCommand {
            target: Target::BackLogo,
            effect: get_effect_from_state(&state.logo.effect.value()),
            brightness: state.logo.brightness.value as u8,
            speed: Speed::new(state.logo.speed.value as u8),
            direction: Direction::None,
            color: get_color_from_state(&state.logo.color.color),
            zone: Zone::None,
        },
        LightingCommand {
            target: Target::PerformanceModeButton,
            effect: get_effect_from_state(&state.mode_button.effect.value()),
            brightness: state.mode_button.brightness.value as u8,
            speed: Speed::new(state.mode_button.speed.value as u8),
            direction: Direction::None,
            color: get_color_from_state(&state.mode_button.color.color),
            zone: Zone::None,
        },
    ]
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
        new_state.perf.apply(config.system()?);
    }

    if old_state.battery != new_state.battery {
        new_state.battery.apply(config.system()?);
    }

    for (old_command, new_command) in old_state.lighting.iter().zip(&new_state.lighting) {
        if old_command != new_command {
            new_command.apply(config.rgb()?)?;
        }
    }

    Ok(())
}
