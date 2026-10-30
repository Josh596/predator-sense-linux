use std::{path::PathBuf, process::ExitCode};

use crate::tui::{app::App, view::View};
use predatorsense::store;

mod apply;
mod logging;
mod services;
mod tui;

// use hidapi::HidApi;

const USAGE: &str = "usage: predatorsense [apply [PROFILE]]";

fn main() -> ExitCode {
    if let Err(e) = logging::init() {
        eprintln!("could not initialise logging: {e}");
    }
    log::info!("Logging init");

    // args_os, not args: paths need not be valid UTF-8
    let mut args = std::env::args_os().skip(1);

    match args.next() {
        None => run_tui(),
        Some(command) if command == "apply" => {
            let profile_path = args.next().map(PathBuf::from);
            if args.next().is_some() {
                eprintln!("{USAGE}");
                return ExitCode::FAILURE;
            }

            match profile_path.or_else(default_profile_path) {
                Some(path) => apply::run(&path),
                None => ExitCode::FAILURE,
            }
        }
        Some(_) => {
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn run_tui() -> ExitCode {
    let Some(profile_path) = default_profile_path() else {
        return ExitCode::FAILURE;
    };

    let profile = match store::load(&profile_path) {
        Ok(profile) => Some(profile),
        Err(e) => {
            log::warn!("{e}; starting from defaults");
            None
        }
    };

    // Here we jsut call the App run
    let app = App::new(profile_path, profile);
    if let Err(e) = app.restore() {
        log::error!("could not restore state to the hardware: {e}");
    }

    ratatui::run(|terminal| app.run(terminal, &View::default()));

    ExitCode::SUCCESS
}

fn default_profile_path() -> Option<PathBuf> {
    store::default_path()
        .inspect_err(|e| eprintln!("could not determine where to store the lighting profile: {e}"))
        .ok()
}
