use ratatui::crossterm::event::{self};

use ratatui::DefaultTerminal;

use crate::tui::event::EventHandler;
use crate::tui::state::{ApplicationState, RunningState};

use crate::tui::view::View;

#[derive(Default)]
pub struct App {
    state: ApplicationState,
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal, view: &View) {
        while !(self.state.running_state == RunningState::Done) {
            terminal.draw(|frame| view.render(frame, &mut self.state));
            let current_event = event::read().expect("Could not read event");
            EventHandler::default().handle_event(current_event, &mut self.state);
        }
    }
}
