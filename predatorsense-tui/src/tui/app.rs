use std::path::PathBuf;
use std::time::Duration;

use predatorsense::commands::battery::ChargingLimit;
use predatorsense::error::Error;
use ratatui::crossterm::event::{self};

use ratatui::DefaultTerminal;

use crate::services::Applied;
use crate::tui::event::EventHandler;
use crate::tui::state::lighting::profile::LightingProfile;
use crate::tui::state::{ApplicationState, RunningState};
use crate::{services, store};
use predatorsense::config::Config;

use crate::tui::view::View;

pub struct App {
    state: ApplicationState,
    config: Config,
    profile_path: PathBuf,
    /// The profile as it currently stands on disk, so an unchanged session
    /// never rewrites the file.
    last_saved: LightingProfile,
}

impl App {
    pub fn restore(&self) -> Result<(), Error> {
        services::execute(None, Applied::desired(&self.state), &self.config)
    }
    pub fn new(profile_path: PathBuf, profile: Option<LightingProfile>) -> Self {
        let config = Config::default();
        let mut state = ApplicationState::default();

        // Lighting
        if let Some(profile) = profile {
            profile.apply_to(&mut state.lighting_page_state);
        }

        let last_saved = LightingProfile::from(&state.lighting_page_state);

        // Battery
        match config.system().and_then(ChargingLimit::from_system) {
            Ok(limit) => state.battery_page_state.load_from(limit),
            Err(e) => log::warn!("could not read charging limit: {e}; using defaults"),
        }

        Self {
            state,
            config: config,
            profile_path,
            last_saved,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal, view: &View) {
        const IDLE: Duration = Duration::from_millis(400);

        while self.state.running_state != RunningState::Done {
            terminal.draw(|frame| view.render(frame, &mut self.state));

            if event::poll(IDLE).expect("could not poll for events") {
                let current_event = event::read().expect("Could not read event");
                EventHandler::default().handle_event(current_event, &mut self.state, &self.config);
            } else {
                // No input for IDLE, so persist to disk
                self.persist_lighting();
            }
        }

        // Persist on exit
        self.persist_lighting();
    }

    /// Writes the lighting profile only when it differs from what is on disk.
    /// Most events (navigation, redraws, other pages) change nothing here.
    fn persist_lighting(&mut self) {
        let profile = LightingProfile::from(&self.state.lighting_page_state);

        if profile == self.last_saved {
            return;
        }

        match store::save(&profile, &self.profile_path) {
            Ok(()) => self.last_saved = profile,
            Err(e) => log::error!("could not save lighting profile: {e}"),
        }
    }
}
