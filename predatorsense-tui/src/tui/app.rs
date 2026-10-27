use std::path::PathBuf;
use std::time::Duration;

use predatorsense::commands::battery::ChargingLimit;
use predatorsense::commands::performance::PerfMode;
use predatorsense::error::Error;
use predatorsense::store;
use ratatui::crossterm::event::{self};

use ratatui::DefaultTerminal;

use crate::services;
use crate::services::Applied;
use crate::tui::event::EventHandler;
use crate::tui::state::battery::BatteryPageState;
use crate::tui::state::lighting::LightingPageState;
use predatorsense::profile::LightingProfile;
use crate::tui::state::performance::PerformancePageState;
use crate::tui::state::{ApplicationState, Page, RunningState};
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

        let mut lighting_page_state = LightingPageState::new(&profile.unwrap_or_default());
        let last_saved = LightingProfile::from(&lighting_page_state);

        let state = ApplicationState {
            running_state: RunningState::default(),
            active_page: Page::default(),
            battery_page_state: load_battery(&config),
            perf_page_state: load_performance(&config),
            lighting_page_state,
        };

        Self {
            state,
            config,
            profile_path,
            last_saved,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal, view: &View) {
        const IDLE: Duration = Duration::from_millis(400);

        while self.state.running_state != RunningState::Done {
            terminal
                .draw(|frame| view.render(frame, &mut self.state))
                .expect("ratatui could not draw frame");

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

        log::info!("Persisitng to disk");

        match store::save(&profile, &self.profile_path) {
            Ok(()) => {
                self.last_saved = profile;
                log::info!("Saved profile to {}", self.profile_path.display())
            }
            Err(e) => log::error!("could not save lighting profile: {e}"),
        }
    }
}

fn load_battery(config: &Config) -> BatteryPageState {
    match config.system().and_then(ChargingLimit::from_system) {
        Ok(limit) => BatteryPageState::new(limit),
        Err(e) => {
            log::warn!("could not read charging limit: {e}; using defaults");
            BatteryPageState::default()
        }
    }
}

fn load_performance(config: &Config) -> PerformancePageState {
    match config.system().and_then(PerfMode::from_system) {
        Ok(mode) => PerformancePageState::new(mode),
        Err(e) => {
            log::warn!("could not read performance mode: {e}; using default");
            PerformancePageState::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;
    use std::fs;

    /// A fresh directory under the system temp dir, deleted when dropped.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "predatorsense-app-test-{}-{name}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// Builds an `App` directly, skipping `App::new`, which needs real devices.
    fn offline_app(profile_path: PathBuf) -> App {
        let state = ApplicationState::default();
        let last_saved = LightingProfile::from(&state.lighting_page_state);
        App {
            state,
            config: Config::offline(),
            profile_path,
            last_saved,
        }
    }

    // persist_lighting

    #[test]
    fn persist_unchanged_profile_writes_nothing() {
        let dir = TempDir::new("unchanged");
        let path = dir.0.join("lighting.toml");
        let mut app = offline_app(path.clone());

        app.persist_lighting();

        assert!(!path.exists());
    }

    #[test]
    fn persist_changed_profile_writes_file() {
        let dir = TempDir::new("changed");
        let path = dir.0.join("lighting.toml");
        let mut app = offline_app(path.clone());
        app.state.lighting_page_state.keyboard.color.color = Color::Rgb(1, 2, 3);

        app.persist_lighting();

        let expected = LightingProfile::from(&app.state.lighting_page_state);
        assert_eq!(store::load(&path).unwrap(), expected);
    }

    #[test]
    fn persist_changed_profile_updates_last_saved() {
        let dir = TempDir::new("last-saved");
        let mut app = offline_app(dir.0.join("lighting.toml"));
        app.state
            .lighting_page_state
            .keyboard
            .brightness
            .set_value(40);

        app.persist_lighting();

        assert_eq!(
            app.last_saved,
            LightingProfile::from(&app.state.lighting_page_state)
        );
    }

    #[test]
    fn persist_failed_save_keeps_last_saved() {
        let dir = TempDir::new("failed");
        // The parent "directory" is a file, so the save must fail.
        let blocker = dir.0.join("not-a-dir");
        fs::write(&blocker, "").unwrap();
        let mut app = offline_app(blocker.join("lighting.toml"));
        let before = LightingProfile::from(&app.state.lighting_page_state);
        app.state
            .lighting_page_state
            .keyboard
            .brightness
            .set_value(40);

        app.persist_lighting();

        assert_eq!(app.last_saved, before);
    }

    // load_battery / load_performance

    #[test]
    fn load_battery_offline_uses_defaults() {
        let state = load_battery(&Config::offline());
        let default = BatteryPageState::default();
        assert_eq!(
            state.enable_charging_limit.value,
            default.enable_charging_limit.value
        );
        assert_eq!(
            state.upper_charging_limit.value,
            default.upper_charging_limit.value
        );
        assert_eq!(
            state.lower_charging_limit.value,
            default.lower_charging_limit.value
        );
    }

    #[test]
    fn load_performance_offline_uses_default_mode() {
        let state = load_performance(&Config::offline());
        assert_eq!(
            state.mode_input.state.selected(),
            PerformancePageState::default().mode_input.state.selected()
        );
    }
}
