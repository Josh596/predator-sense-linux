use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, LineGauge, Padding},
};

use crate::tui::{
    components::inputs::Input,
    state::{ApplicationState, battery::BatteryPageInput, fields::NumericField},
    style::{DANGER_COLOR, GAUGE_STYLE, OK_COLOR, TEXT_DIM_COLOR, TEXT_STYLE_DIM, WARNING_COLOR},
    utils::{Action, cursor},
    view::page::PageView,
};

const LINE_TITLE_MAX_LENGTH: u16 = 20;

#[derive(Default)]
pub struct BatteryPage;

impl BatteryPage {
    fn render_battery_info(frame: &mut Frame, rect: Rect, state: &ApplicationState) {
        let upper_text_layout: [Rect; 3] = Layout::horizontal([
            Constraint::Length(LINE_TITLE_MAX_LENGTH),
            Constraint::Length(10),
            Constraint::Length(4),
        ])
        .areas(rect);

        // Render upper text;
        let title1 = Span::from("Battery Level").style(TEXT_STYLE_DIM);

        let battery_level = 0.6;
        let mut battery_guage = LineGauge::default()
            .ratio(battery_level)
            .unfilled_style(GAUGE_STYLE.fg(TEXT_DIM_COLOR))
            .filled_symbol("█")
            .unfilled_symbol("░")
            .label("")
            .bold();

        // Render the charging info here
        if battery_level > 0.5 {
            battery_guage = battery_guage.style(GAUGE_STYLE.fg(OK_COLOR));
        } else if battery_level > 0.3 {
            battery_guage = battery_guage.style(GAUGE_STYLE.fg(WARNING_COLOR));
        } else {
            battery_guage = battery_guage.style(GAUGE_STYLE.fg(DANGER_COLOR));
        }

        let battery_percentage = Line::from(format!("{}%", (battery_level * 100.0) as u8))
            .style(TEXT_STYLE_DIM)
            .right_aligned();

        frame.render_widget(title1, upper_text_layout[0]);
        frame.render_widget(battery_guage, upper_text_layout[1]);
        frame.render_widget(battery_percentage, upper_text_layout[2]);
    }

    fn render_battery_limit_inputs(frame: &mut Frame, rect: Rect, state: &ApplicationState) {
        let layout: [Rect; 3] = Layout::vertical(Constraint::from_lengths([1, 1, 1])).areas(rect);
        let max_title_length = 25;
        // Enable Charging Limit
        let enable_charging_limit_text = Span::from(format!(
            "{:<width$}",
            "Charging Limit",
            width = max_title_length
        ))
        .style(TEXT_STYLE_DIM);

        let mut enable_charging_limit_line = Line::from(vec![
            cursor(state.battery_page_state.active_input == BatteryPageInput::EnableChargingLimit),
            enable_charging_limit_text,
            Span::from(" "),
        ]);
        enable_charging_limit_line.extend(state.battery_page_state.enable_charging_limit.render());

        // Upper Limit Text

        let upper_limit_text = Span::from(format!(
            "{:<width$}",
            "Upper Charging Limit",
            width = max_title_length
        ))
        .style(TEXT_STYLE_DIM);

        let mut upper_limit_line = Line::from(vec![
            cursor(state.battery_page_state.active_input == BatteryPageInput::UpperChargingLimit),
            upper_limit_text,
            Span::from(" "),
        ]);

        upper_limit_line.extend(state.battery_page_state.upper_charging_limit.render());

        // Lower Limit Text

        let lower_limit_text = Span::from(format!(
            "{:<width$}",
            "Lower Charging Limit",
            width = max_title_length
        ))
        .style(TEXT_STYLE_DIM);

        let mut lower_limit_line = Line::from(vec![
            cursor(state.battery_page_state.active_input == BatteryPageInput::LowerChargingLimit),
            lower_limit_text,
            Span::from(" "),
        ]);
        lower_limit_line.extend(state.battery_page_state.lower_charging_limit.render());

        frame.render_widget(enable_charging_limit_line, layout[0]);
        frame.render_widget(upper_limit_line, layout[1]);
        frame.render_widget(lower_limit_line, layout[2]);
    }
}

impl PageView for BatteryPage {
    fn render(&self, frame: &mut Frame, rect: Rect, state: &ApplicationState) {
        // A block widget; add paddintg around with 2 spaces,
        let block = Block::default().padding(Padding::uniform(2));
        let inner_area = block.inner(rect);

        let layout: [Rect; 3] =
            Layout::vertical(Constraint::from_lengths([1, 1, 3])).areas(inner_area);

        BatteryPage::render_battery_info(frame, layout[0], state);
        BatteryPage::render_battery_limit_inputs(frame, layout[2], state);
    }

    fn actions(&self, state: &ApplicationState) -> Vec<Action> {
        let mut actions = vec![Action::new("↑↓", "Move")];

        if [
            BatteryPageInput::UpperChargingLimit,
            BatteryPageInput::LowerChargingLimit,
        ]
        .contains(&state.battery_page_state.active_input)
        {
            let input_actions = vec![Action::new("←→", "Adjust")];
            actions.extend(input_actions);
        }

        if state.battery_page_state.active_input == BatteryPageInput::EnableChargingLimit {
            let toggle_action = Action::new("Space|Enter", "Toggle");
            actions.push(toggle_action);
        }

        actions
    }
}
