use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Widget},
};
use std::time::SystemTime;

use crate::settings::settings::{Settings, UISearchSettings};
#[derive(Clone, Default)]
pub struct SearchBox {
    query: String,
    caret_position: usize,
    settings: UISearchSettings,
}

impl SearchBox {
    pub fn new(query: String, caret_position: Option<usize>, settings: UISearchSettings) -> Self {
        Self {
            query: query.clone(),
            caret_position: caret_position.unwrap_or(query.len()),
            settings,
        }
    }
}

impl Widget for SearchBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Settings::new().ui.theme.clone();
        let block = Block::bordered()
            .title("Search")
            .border_type(theme.get_border_type("search"))
            .padding(Padding::new(2, 2, 0, 0))
            .style(theme.get_default_style(Some(crate::model::ui::UISection::Search)));
        // block.render(area, buf);
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");

        let mut line = "".to_string();
        line.push_str(self.settings.pre_query.as_str());
        line.push(' ');

        let caret_query = self.query.clone();
        let (before_caret, after_caret) =
            caret_query.split_at(self.caret_position.min(caret_query.len()));

        let caret = self.settings.caret.clone();
        let mut flash_caret = false;
        // let mut caret_query = before_caret.to_string();
        if self.settings.caret_visible {
            if (since_epoch.as_millis() as u64 / self.settings.caret_blink_rate) % 2 == 0 {
                flash_caret = true;
            }
        }
        // line.push_str(caret_query.as_str());

        let inner_area = block.inner(area);

        // construct the line with styled spans
        // query = "hello world", caret_position = 5
        // before_caret = "hello", after_caret = " world"
        let line = Line::from(vec![
            Span::styled(
                self.settings.pre_query.as_str(),
                Style::default().fg(Color::Blue),
            ),
            Span::raw(" "),
            Span::styled(before_caret, Style::default().fg(Color::White)),
            Span::styled(
                if flash_caret { " " } else { &caret },
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(after_caret, Style::default().fg(Color::White)),
        ]);

        // paragraph.render(inner_area, buf);
        block.render(area, buf);

        let paragraph = Paragraph::new(line)
            .style(theme.get_default_style(Some(crate::model::ui::UISection::Search)));
        paragraph.render(inner_area, buf);

        // paragraph.render(inner_area, buf);
    }
}
