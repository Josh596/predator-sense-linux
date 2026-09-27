use crate::tui::{app::App, view::View};

mod services;
mod tui;

use hidapi::HidApi;
use log4rs;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default());
    log::info!("Logging init");
    // List all devices into log
    let api = HidApi::new().unwrap();
    let device_list = api.device_list();
    device_list
        .into_iter()
        .for_each(|device| log::info!("{:?}", device));
    // Here we jsut call the App run
    ratatui::run(|terminal| App::default().run(terminal, &View::default()));
}
