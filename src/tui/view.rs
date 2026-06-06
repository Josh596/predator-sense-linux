use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::{DefaultTerminal, Frame};
use strum::VariantNames;

use crate::tui::app::{ApplicationState, Page, RunningState};
use crate::tui::components::pages::PageUI;
use crate::tui::components::pages::performance::PerfomancePageUI;
use crate::tui::utils::Action;

#[derive(Default)]
pub struct App {
    state: ApplicationState,
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal) {
        while !(self.state.running_state == RunningState::Done) {
            terminal.draw(|frame| self.render_frame(frame));
            let current_event = event::read().expect("Could not read event");
            self.handle_events(current_event);
        }
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let mut block = Block::default().borders(Borders::all());
        block = self.add_shortcuts_to_block_title(block);

        // Get the inner area of the block;
        let inner_area = block.inner(frame.area());

        // Break it into 2 using vertical Layout;
        let [header, content] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(inner_area);

        frame.render_widget(block, frame.area());

        self.render_top_menu(frame, header);
        // A block widget as the main widget;
        // Then break the inner using Layouts; into header and content;
        // The block should have titles for each actions; so Global actions and then Active Page Actions; which should be rendered at the bottom; how do i do this dynamically?
        // A function to get the shortcuts as str; so i can then loop over them and call block.title_bottom

        match self.state.active_page {
            Page::Battery => {}
            Page::Lighting => {}
            Page::Performance => {
                PerfomancePageUI.render(&mut self.state, frame, content);
            }
        }

        return;
    }

    fn render_top_menu(&self, frame: &mut Frame, rect: Rect) {
        //
        // A block with a bottom border;
        let block = Block::new().borders(Borders::BOTTOM);

        // change this to map; use [1] title
        let titles: Vec<String> = Page::VARIANTS
            .iter()
            .enumerate()
            .map(|(index, title)| format!("[{index}] {title}"))
            .collect();

        // Get index of active page
        let active_page_index = self.state.active_page as usize;

        let tabs = Tabs::new(titles)
            .block(block)
            .highlight_style(Style::new().blue().bold())
            .select(active_page_index);

        frame.render_widget(tabs, rect);
    }

    fn actions(&self) -> Vec<Action> {
        let mut all_actions = vec![Action::new("q", "Quit")];
        let active_page_actions = match self.state.active_page {
            Page::Battery => PerfomancePageUI.actions(),
            Page::Lighting => PerfomancePageUI.actions(),
            Page::Performance => PerfomancePageUI.actions(),
        };

        all_actions.extend(active_page_actions);

        return all_actions;
    }
    fn add_shortcuts_to_block_title<'a>(&'a self, mut block: Block<'a>) -> Block<'a> {
        for action in self.actions() {
            let title = Line::from(vec![
                Span::styled(action.key, Style::new().blue()),
                Span::raw(action.action),
            ])
            .left_aligned();

            block = block.title_bottom(title);
        }
        return block;
    }
    fn handle_events(&mut self, event: Event) {
        if let Some(key) = &event.as_key_press_event() {
            self.handle_key(key);
            self.handle_key_event_for_active_page(key);
        }
    }

    pub fn handle_key(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.state.running_state = RunningState::Done,
            _ => {}
        }
    }

    pub fn handle_key_event_for_active_page(&mut self, key: &KeyEvent) {
        match self.state.active_page {
            Page::Battery => {}
            Page::Lighting => {}
            Page::Performance => PerfomancePageUI.handle_event(&mut self.state, key),
        }
    }
}
