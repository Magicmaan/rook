use crate::{
    common::module_state::UISection,
    effects::{self, rainbow},
    settings::settings::{Settings, UISearchSettings},
    ui::util::collapsed_border,
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
pub struct SearchBox<'a> {
    settings: &'a Settings,
}

impl<'a> SearchBox<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }

    fn construct_line(
        &self,
        pre_query: &str,
        pre_caret: &str,
        caret: &str,
        post_caret: &str,
        post_fix: &str,
        flash_caret: bool,
    ) -> Line<'static> {
        let theme = self.settings.ui.theme.get_search_colors();
        let line: Line<'static> = Line::from(vec![
            // pre_query span
            Span::styled(
                pre_query.to_owned(),
                Style::default().fg(theme.pre_query_text.unwrap()),
            ),
            Span::raw(" ".to_owned()),
            // query span with caret
            Span::styled(
                pre_caret.to_owned(),
                Style::default().fg(theme.text.unwrap()),
            ),
            Span::styled(
                if flash_caret {
                    " ".to_owned()
                } else {
                    caret.to_owned()
                },
                Style::default().fg(theme.caret.unwrap()),
            ),
            Span::styled(
                post_caret.to_owned(),
                Style::default().fg(theme.text.unwrap()),
            ),
            Span::raw(" ".to_owned()),
            Span::styled(
                post_fix.to_owned(),
                Style::default().fg(theme.text_muted.unwrap()),
            ),
            Span::styled(
                " ".repeat(pre_query.chars().count()),
                Style::default().fg(Color::Reset),
            ),
        ]);
        line
    }
}

impl StatefulWidget for SearchBox<'_> {
    type State = SearchBoxState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let theme = self.settings.ui.theme.clone();
        let search_theme = theme.get_search_colors();
        let gap = self.settings.ui.layout.gap;
        let search_settings: UISearchSettings = self.settings.ui.search.clone();

        let default_borders = theme.get_border_type(UISection::Search).to_border_set();
        let (visible_borders, collapsed_borders) = collapsed_border(
            UISection::Search,
            &self.settings.ui.layout.sections,
            default_borders,
        );

        let padding = Padding::new(
            search_settings.padding.saturating_mul(2).max(2),
            search_settings.padding.saturating_mul(2).max(2),
            search_settings.padding,
            search_settings.padding,
        );
        let block = Block::bordered()
            .title(self.settings.ui.layout.title.as_str())
            .title_alignment(self.settings.ui.layout.title_alignment)
            .title_style(Style::default().fg(self.settings.ui.theme.title))
            //
            .border_set(if gap > 0 {
                default_borders
            } else {
                collapsed_borders
            })
            .border_style(theme.get_default_border_style(Some(UISection::Search)))
            // .border_type(theme.get_border_type("results"))
            .borders(if gap > 0 {
                Borders::ALL
            } else {
                visible_borders
            })
            //
            .padding(padding)
            .style(theme.get_default_style(Some(UISection::Search)));
        let inner_area = block.inner(area);

        // render container
        block.render(area, buf);

        // rainbow border effect
        if self.settings.ui.search.rainbow_border {
            let t = state.tick as u32;
            let speed: f32 = self.settings.ui.search.rainbow_border_speed;
            effects::rainbow(search_theme.border.unwrap(), 2000, speed, area, buf, t);
        }

        //
        // Search Box text rendering
        //

        // splice the query to insert the caret
        let caret_query = state.query.clone();
        let (before_caret, after_caret) =
            caret_query.split_at(state.caret_position.min(caret_query.len()));

        // get caret and blink state
        let caret = &search_settings.caret_text;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;
        let flash_caret =
            search_settings.caret_visible && (now / search_settings.caret_blink_rate) % 2 == 0;

        // construct line with styled spans
        // i.e. >> hello wor▋ld
        let line = self.construct_line(
            search_settings.pre_query.as_str(),
            before_caret,
            &caret,
            after_caret,
            state.post_fix.as_str(),
            flash_caret,
        );

        let paragraph = Paragraph::new(line)
            .alignment(self.settings.ui.search.text_alignment)
            .style(Style::default().bg(search_theme.background.unwrap()));

        paragraph.render(inner_area, buf);
    }
}
