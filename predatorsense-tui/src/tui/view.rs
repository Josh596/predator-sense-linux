use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Padding};
use strum::{EnumCount, IntoEnumIterator};

use crate::tui::state::{ApplicationState, Page};
use crate::tui::style::{BG_COLOR, BORDER_STYLE, TEXT_STYLE, TEXT_STYLE_ACTIVE, TEXT_STYLE_DIM};
use crate::tui::utils::Action;
use crate::tui::view::battery::BatteryPage;
use crate::tui::view::page::PageView;

pub mod battery;
pub mod lighting;
pub mod performance;

pub mod page;
#[derive(Default)]
pub struct View;

impl View {
    pub fn render(&self, frame: &mut Frame, state: &mut ApplicationState) {
        let block = Block::bordered().border_style(BORDER_STYLE).bg(BG_COLOR);

        let inner_area = block.inner(frame.area());
        // Break Layout into 3; header, content and footer
        let layout: [Rect; 4] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(inner_area);

        let page_area = layout[2].inner(Margin::new(2, 1));

        frame.render_widget(block, frame.area());
        self.render_title_block(frame, layout[0], state);
        self.render_header(frame, layout[1], state);
        self.render_page(frame, page_area, state);
        self.render_footer(frame, layout[3], state);

        return;
    }

    fn render_title_block(&self, frame: &mut Frame, rect: Rect, _state: &ApplicationState) {
        // Just bottom border
        let block = Block::bordered()
            .borders(Borders::BOTTOM)
            .padding(Padding::horizontal(2));

        let inner_area = block.inner(rect);

        // two layout constrainttl left and right
        let layout: [Rect; 2] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(inner_area);

        let left_content = Line::from(vec![
            Span::from("◢▼◣ LinuxPredator  ".to_uppercase()).style(TEXT_STYLE_ACTIVE),
        ])
        .left_aligned()
        .bold();

        let right_content = Line::from(vec![
            Span::from(format!("{}· {}", "Helios 300", "PH315-54")).style(TEXT_STYLE),
        ])
        .right_aligned()
        .bold();

        frame.render_widget(block, rect);
        frame.render_widget(left_content, layout[0]);
        frame.render_widget(right_content, layout[1]);
    }

    fn render_footer(&self, frame: &mut Frame, rect: Rect, state: &ApplicationState) {
        let block = Block::bordered()
            .borders(Borders::TOP)
            .padding(Padding::horizontal(2));

        let inner_area = block.inner(rect);

        let content = self.get_actions_text(state);

        frame.render_widget(block, rect);
        frame.render_widget(content, inner_area);
    }

    fn render_header(&self, frame: &mut Frame, rect: Rect, state: &ApplicationState) {
        let block = Block::bordered()
            .borders(Borders::BOTTOM)
            .padding(Padding::horizontal(2));

        let inner_area = block.inner(rect);

        let mut header_spans: Vec<Span> = Vec::new();

        for (i, page) in Page::iter().enumerate() {
            if i > 0 {
                header_spans.push(Span::styled(" | ", TEXT_STYLE_DIM));
            }

            let page_id_text = format!("[{}] ", page as usize + 1);
            header_spans.push(Span::styled(page_id_text, TEXT_STYLE_ACTIVE));

            let page_name = page.to_string().to_uppercase();
            let page_style = if page == state.active_page {
                TEXT_STYLE_ACTIVE
            } else {
                TEXT_STYLE_DIM
            };

            header_spans.push(Span::styled(page_name, page_style));
        }

        // 5. Wrap all the spans into a single Line and align the whole thing
        let header = Line::from(header_spans).left_aligned();

        // Asssemble the text
        frame.render_widget(block, rect);
        frame.render_widget(header, inner_area);
        // frame.render_widget(content, inner_area);
    }

    fn actions(&self, state: &ApplicationState) -> Vec<Action> {
        // let page_actions =
        let mut all_actions = vec![
            Action::new("q", "Quit"),
            Action::new(format!("{}-{}", 1, Page::COUNT), "Switch Page"),
        ];

        all_actions.extend(self.get_active_page_view(state).actions(state));

        return all_actions;
    }

    fn get_actions_text<'a>(&self, state: &ApplicationState) -> Line<'a> {
        let mut lines = Vec::new();

        for action in self.actions(state) {
            let key = Span::from(format!("[{}]", action.key)).style(TEXT_STYLE_ACTIVE);
            let action_text = Span::from(action.action).style(TEXT_STYLE_DIM);

            let line =
                Line::from(vec![key, Span::raw(" "), action_text, Span::raw("  ")]).left_aligned();
            lines.push(line);
        }

        return lines.into_iter().fold(Line::default(), |mut acc, line| {
            acc.extend(line);
            acc
        });
    }

    fn get_active_page_view(&self, state: &ApplicationState) -> Box<dyn PageView> {
        match state.active_page {
            Page::Battery => Box::new(BatteryPage::default()),
            Page::Performance => Box::new(performance::PerformancePage::default()),
            Page::Lighting => Box::new(lighting::LightingPage::default()),
            _ => Box::new(BatteryPage::default()),
        }
    }

    fn render_page(&self, frame: &mut Frame, rect: Rect, state: &mut ApplicationState) {
        self.get_active_page_view(state).render(frame, rect, state);
    }
}
