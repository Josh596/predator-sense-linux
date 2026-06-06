use crate::tui::view::App;

mod hid;

mod error;

mod commands;

mod tui;
fn main() {
    // Here we jsut call the App run
    ratatui::run(|terminal| App::default().run(terminal))
}
