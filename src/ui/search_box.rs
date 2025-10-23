use crate::{
    effects::{self, rainbow},
    model::module_state::UISection,
    settings::settings::{Settings, UISearchSettings},
};
use ratatui::widgets::Borders;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget},
};
use std::time::SystemTime;
use tachyonfx::{Duration, EffectManager, fx, pattern::SweepPattern};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SearchBoxState {
    pub post_fix: String,
    pub query: String,
    pub caret_position: usize,
    pub last_search_tick: u64,
    pub tick: u64,
    pub delta_time: i32,
}

#[derive(Clone)]
pub struct SearchBox {
    settings: Settings,
}

impl SearchBox {
    pub fn new(settings: &Settings) -> Self {
        Self {
            settings: settings.clone(),
        }
    }
}

impl StatefulWidget for SearchBox {
    type State<'b> = SearchBoxState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State<'_>) {
        let theme = self.settings.ui.theme.clone();
        let search_theme = theme.get_search_colors();
        let gap = self.settings.ui.layout.gap;
        let search_settings: UISearchSettings = self.settings.ui.search.clone();

        let padding = self.settings.ui.search.padding;
        let block = Block::bordered()
            .title(self.settings.ui.layout.title.as_str())
            .title_alignment(self.settings.ui.layout.title_alignment)
            .title_style(Style::default().fg(self.settings.ui.theme.title))
            .border_type(theme.get_border_type("search"))
            // if no gap, remove bottom border
            .borders(if gap > 0 {
                Borders::ALL
            } else {
                Borders::TOP | Borders::LEFT | Borders::RIGHT
            })
            .border_style(
                self.settings
                    .ui
                    .theme
                    .get_default_border_style(Some(UISection::Search)),
            )
            .padding(Padding::new(
                padding.saturating_mul(2).max(2),
                padding.saturating_mul(2).max(2),
                padding,
                padding,
            ))
            .style(theme.get_default_style(Some(UISection::Search)));
        let inner_area = block.inner(area);

        // render container
        block.render(area, buf);

        if self.settings.ui.search.rainbow_border {
            effects::rainbow(area, buf, state.tick as u32);
        }
        //
        // Search Box text rendering
        //

        // splice the query to insert the caret
        let caret_query = state.query.clone();
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
        if search_settings.caret_visible
            && (since_epoch.as_millis() as u64 / search_settings.caret_blink_rate).is_multiple_of(2)
        {
            flash_caret = true;
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
                Style::default().fg(search_theme.text.unwrap()),
            ),
            Span::styled(
                if flash_caret { " " } else { &caret },
                Style::default().fg(search_theme.caret.unwrap()),
            ),
            Span::styled(after_caret, Style::default().fg(search_theme.text.unwrap())),
            Span::raw(" "),
            Span::styled(
                state.post_fix.as_str(),
                Style::default().fg(search_theme.text_muted.unwrap()),
            ),
            Span::styled(
                " ".repeat(search_settings.pre_query.chars().count()),
                Style::default().fg(Color::Reset),
            ),
        ]);

        let paragraph = Paragraph::new(line)
            .alignment(self.settings.ui.search.text_alignment)
            .style(Style::default().bg(search_theme.background.unwrap()));

        paragraph.render(inner_area, buf);

        // paragraph.render(inner_area, buf);
    }
}
