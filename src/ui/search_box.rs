use crate::model::module::UISection;
use crate::{
    model::model::Model,
    settings::settings::{Settings, UISearchSettings},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget},
};
use ratatui::{layout::Alignment, widgets::Borders};
use std::time::SystemTime;
#[derive(Clone)]
pub struct SearchBox<'a> {
    settings: &'a Settings,
}

impl<'a> SearchBox<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }
}

impl<'a> StatefulWidget for SearchBox<'a> {
    type State = crate::model::module::ModuleState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let theme = self.settings.ui.theme.clone();
        let search_theme = theme.get_search_colors();
        let gap = self.settings.ui.layout.gap;
        let search_settings: UISearchSettings = self.settings.ui.search.clone();

        let block = Block::bordered()
            .title("Rook")
            .title_alignment(Alignment::Center)
            .border_type(theme.get_border_type("search"))
            // if no gap, remove bottom border
            .borders(if gap > 0 {
                Borders::ALL
            } else {
                Borders::TOP | Borders::LEFT | Borders::RIGHT
            })
            .padding(Padding::new(2, 2, 0, 0))
            .style(theme.get_default_style(Some(crate::model::module::UISection::Search)));
        let inner_area = block.inner(area);

        // splice the query to insert the caret
        let caret_query = state.search.query.clone();
        let (before_caret, after_caret) =
            caret_query.split_at(state.caret_position.min(caret_query.len()));

        // get caret, and blink state
        let caret = search_settings.caret_text.clone();
        let mut flash_caret = false;
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");

        // let mut caret_query = before_caret.to_string();
        if search_settings.caret_visible {
            if (since_epoch.as_millis() as u64 / search_settings.caret_blink_rate) % 2 == 0 {
                flash_caret = true;
            }
        }

        // construct line with styled spans
        // i.e. >> hello world▋
        let line = Line::from(vec![
            // pre_query span
            Span::styled(
                search_settings.pre_query.as_str(),
                Style::default().fg(search_theme.pre_query_text.unwrap()),
            ),
            Span::raw(" "),
            // query span with caret
            Span::styled(
                before_caret,
                Style::default().fg(search_theme.caret.unwrap()),
            ),
            Span::styled(
                if flash_caret { " " } else { &caret },
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(after_caret, Style::default().fg(Color::White)),
        ]);

        // paragraph.render(inner_area, buf);
        block.render(area, buf);

        let paragraph = Paragraph::new(line)
            .style(theme.get_default_style(Some(crate::model::module::UISection::Search)));
        paragraph.render(inner_area, buf);

        // paragraph.render(inner_area, buf);
    }
}
