use std::{path::Path, process::ExitCode};

use predatorsense::{config::Config, store};

pub fn run(profile_path: &Path) -> ExitCode {
    // load the profile from the path
    let profile = match store::load(profile_path) {
        Ok(profile) => profile,
        Err(e) => {
            log::error!("{e}");
            return ExitCode::FAILURE;
        }
    };

    let config = Config::open();
    let device = match config.rgb() {
        Ok(device) => device,
        Err(e) => {
            log::error!("{e}");
            return ExitCode::FAILURE;
        }
    };

    // apply the profile to the hardware
    let mut failed = false;
    for command in profile.commands() {
        if let Err(e) = command.apply(device) {
            log::error!("could not apply {} lighting: {e}", command.target);
            failed = true;
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        log::info!("applied lighting from {}", profile_path.display());
        ExitCode::SUCCESS
    }
}
