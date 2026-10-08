use crate::tui::{app::App, view::View};

mod services;
mod store;
mod tui;

// use hidapi::HidApi;
use log4rs;

fn main() {
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("could not initialise logging: {e}");
    }
    log::info!("Logging init");
    // List all devices into log
    // let api = HidApi::new().unwrap();
    // let device_list = api.device_list();
    // device_list
    //     .into_iter()
    //     .for_each(|device| log::info!("{:?}", device));
    let profile_path = match store::default_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("could not determine where to store the lighting profile: {e}");
            return;
        }
    };

    let profile = match store::load(&profile_path) {
        Ok(profile) => Some(profile),
        Err(e) => {
            log::warn!(
                "could not load {}: {e}; starting from defaults",
                profile_path.display()
            );
            None
        }
    };

    // Here we jsut call the App run
    let app = App::new(profile_path, profile);
    if let Err(e) = app.restore() {
        log::error!("could not restore state to the hardware: {e}");
    }

    ratatui::run(|terminal| app.run(terminal, &View::default()));
}
