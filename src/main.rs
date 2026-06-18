use crate::tui::{app::App, view::View};

mod hid;

mod error;

mod commands;

mod tui;
fn main() {
    // Here we jsut call the App run
    ratatui::run(|terminal| App::default().run(terminal, &View::default()));
}
