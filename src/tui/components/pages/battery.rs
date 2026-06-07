use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
};
use strum::EnumCount;

use crate::tui::{
    app::{ApplicationState, battery::BatteryPageInput},
    components::pages::PageUI,
    utils::Action,
};

pub struct BatterPageUI;

impl BatterPageUI {
    fn cursor(is_focused: bool) -> Span<'static> {
        if is_focused {
            Span::raw(">").cyan().bold()
        } else {
            Span::raw(" ")
        }
    }

    fn change_active_input(model: &mut ApplicationState, key: &KeyEvent) {
        let max_index = BatteryPageInput::COUNT - 1;
        let current_index = model.battery_state.active_input as usize;
        let mut index: usize = 0;
        match key.code {
            KeyCode::Down => {
                index = std::cmp::min(max_index, current_index + 1);
            }

            KeyCode::Up => {
                index = current_index.saturating_sub(1);
            }
            _ => return,
        }

        model.battery_state.active_input =
            BatteryPageInput::from_repr(index).expect("Invalid Index")
    }
}

impl PageUI for BatterPageUI {
    fn actions(&self) -> Vec<Action> {
        // Here the actions now depend on the active input; hmm
        vec![] as Vec<Action>
    }

    fn handle_event(&mut self, model: &mut ApplicationState, key: &KeyEvent) {
        // So get the enum as iter; then call .count(); so max_index is count-1;
        // so you're increasing the active_index + 1 on key down and reducing by 1 on key up;
        // Limit your subtraction to 0 and addititon to max_index;
        // then you can then call FromIter; to convert from index to enum; then set your state;
        match key.code {
            KeyCode::Enter => {
                if let BatteryPageInput::EnableCharging = model.battery_state.active_input {
                    model.battery_state.battery_charging.enabled =
                        !model.battery_state.battery_charging.enabled;
                }
            }
            KeyCode::Right => match model.battery_state.active_input {
                BatteryPageInput::MaxLimit => {
                    model.battery_state.battery_charging.upper =
                        std::cmp::min(model.battery_state.battery_charging.upper + 1, 100)
                }
                BatteryPageInput::MinLimit => {
                    model.battery_state.battery_charging.lower = std::cmp::min(
                        model.battery_state.battery_charging.lower + 1,
                        model.battery_state.battery_charging.upper - 1,
                    )
                }
                _ => {}
            },
            KeyCode::Left => match model.battery_state.active_input {
                BatteryPageInput::MaxLimit => {
                    model.battery_state.battery_charging.upper = std::cmp::max(
                        model.battery_state.battery_charging.upper - 1,
                        model.battery_state.battery_charging.lower + 1,
                    )
                }
                BatteryPageInput::MinLimit => {
                    model.battery_state.battery_charging.lower =
                        std::cmp::max(model.battery_state.battery_charging.lower - 1, 0)
                }
                _ => {}
            },
            _ => {}
        }
        BatterPageUI::change_active_input(model, key);
    }

    fn render(&self, model: &mut ApplicationState, frame: &mut Frame, rect: Rect) {
        // So we have a match that checks if input is active input and then calls a helper function
        // called cursor that adds the ">"".

        // i think a function that takes a Line widget, and then the BatteryPageInput and also the current model
        // so the function checks if the BatteryPageInput = model.active_input; if it is, then it calls cursor.
        let [row0, _spacer, row1, row2] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(2), // The empty gap
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(rect);

        let checkbox = if model.battery_state.battery_charging.enabled {
            "[x]"
        } else {
            "[ ]"
        };
        let charging_limit_input = Line::from(vec![
            BatterPageUI::cursor(
                model.battery_state.active_input == BatteryPageInput::EnableCharging,
            ),
            Span::raw(format!("{} Enable Charge Limiting", checkbox)),
        ]);

        let max_charge_input = Line::from(vec![
            BatterPageUI::cursor(model.battery_state.active_input == BatteryPageInput::MaxLimit),
            Span::raw(format!(
                "Stop charging at (Max):  < {} > %",
                model.battery_state.battery_charging.upper
            )),
        ]);

        let min_charge_input = Line::from(vec![
            BatterPageUI::cursor(model.battery_state.active_input == BatteryPageInput::MinLimit),
            Span::raw(format!(
                "Start charging at (Min):  < {} > %",
                model.battery_state.battery_charging.lower
            )),
        ]);

        frame.render_widget(charging_limit_input, row0);
        frame.render_widget(max_charge_input, row1);
        frame.render_widget(min_charge_input, row2);
    }
}
