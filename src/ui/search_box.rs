use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use std::time::SystemTime;

use crate::{model, settings::UISearchSettings};
#[derive(Clone, Default)]
pub struct SearchBox {
    query: String,
    caret_position: usize,
    settings: UISearchSettings,
}

impl SearchBox {
    pub fn new(
        query: String,
        caret_position: Option<usize>,
        settings: UISearchSettings,
    ) -> Self {
        Self {
            query: query.clone(),
            caret_position: caret_position.unwrap_or(query.len()),
            settings,
        }
    }
}

impl Widget for SearchBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Search")
            .padding(Padding::new(2, 2, 0, 0));
        // block.render(area, buf);
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");
        let caret = "_";

        let mut line = "".to_string();
        line.push_str(self.settings.pre_query.as_str());
        line.push(' ');

        let mut caret_query = self.query.clone();
        let (before_caret, after_caret) =
            caret_query.split_at(self.caret_position.min(caret_query.len()));

        let mut caret = self.settings.caret.clone();
        let mut flash_caret = false;
        // let mut caret_query = before_caret.to_string();
        if self.settings.caret_visible {
            if (since_epoch.as_millis() as u64 / self.settings.caret_blink_rate) % 2 == 0 {
                flash_caret = true;
            }
        }
        // line.push_str(caret_query.as_str());

        let inner_area = block.inner(area);

        let text_layout = Layout::horizontal([
            Constraint::Length((self.settings.pre_query.chars().count() + 1) as u16),
            Constraint::Length(before_caret.chars().count() as u16),
            Constraint::Length(caret.chars().count() as u16),
            Constraint::Length(after_caret.chars().count() as u16),
            Constraint::Fill(1),
        ]);
        let text_chunks = text_layout.split(inner_area);

        let prepend_paragraph =
            Paragraph::new(self.settings.pre_query).style(Style::default().fg(Color::White));
        let pre_query_paragraph =
            Paragraph::new(before_caret).style(Style::default().fg(Color::White));
        let caret_paragraph = Paragraph::new(if flash_caret { " " } else { &caret })
            .style(Style::default().fg(Color::Yellow));
        let post_query_paragraph =
            Paragraph::new(after_caret).style(Style::default().fg(Color::White));
        // let paragraph = Paragraph::new(line).style(Style::default().fg(Color::Black));

        // paragraph.render(inner_area, buf);
        block.render(area, buf);

        prepend_paragraph.render(text_chunks[0], buf);
        pre_query_paragraph.render(text_chunks[1], buf);
        caret_paragraph.render(text_chunks[2], buf);
        post_query_paragraph.render(text_chunks[3], buf);
        // paragraph.render(inner_area, buf);
    }
}
