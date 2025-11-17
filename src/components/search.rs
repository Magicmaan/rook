use crate::{
    common::{action::Action, module_state::UISection},
    components::{Component, layout::get_root_layout, util::collapsed_border},
    effects::{self, rainbow},
    settings::settings::{Settings, UISearchSettings},
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget},
};
use ratatui::{layout::Constraint, widgets::Borders};
use tui_textarea::TextArea;

use std::{rc::Rc, time::SystemTime};
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
    settings: Option<Settings>,
    render_tick: u64,
    query: String,
    caret_position: usize,
    post_fix: String,
    text_area: TextArea<'static>,
    area: Rect,
}

impl SearchBox {
    pub fn new() -> Self {
        Self {
            settings: None,
            render_tick: 0,
            query: String::new(),
            caret_position: 0,
            post_fix: String::new(),
            text_area: TextArea::default(),
            area: Rect::default(),
        }
    }

    fn construct_line(
        &mut self,
        pre_query: &str,
        pre_caret: &str,
        caret: &str,
        post_caret: &str,
        post_fix: &str,
        flash_caret: bool,
    ) -> Line<'static> {
        let theme = self.settings.as_ref().unwrap().ui.theme.get_search_colors();
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

impl Component for SearchBox {
    fn area(&self) -> Rect {
        self.area
    }
    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<crate::common::action::Action>,
    ) -> color_eyre::eyre::Result<()> {
        let _ = tx; // to appease clippy
        Ok(())
    }

    fn register_settings_handler(&mut self, settings: Settings) -> color_eyre::eyre::Result<()> {
        self.settings = Some(settings); // to appease clippy
        Ok(())
    }

    fn init(&mut self, area: ratatui::prelude::Size) -> color_eyre::eyre::Result<()> {
        let _ = area; // to appease clippy
        Ok(())
    }

    fn handle_events(
        &mut self,
        event: Option<crate::tui::Event>,
    ) -> color_eyre::eyre::Result<Option<crate::common::action::Action>> {
        let action = match event {
            Some(crate::tui::Event::Key(key_event)) => self.handle_key_event(key_event)?,
            Some(crate::tui::Event::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event)?,
            _ => None,
        };
        Ok(action)
    }

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<Option<crate::common::action::Action>> {
        // key handling is different for OS. windows sends press/release events seperately
        if key.kind != KeyEventKind::Press {
            return Ok(None);
        }
        match key.code {
            KeyCode::Enter => {
                return Ok(Some(Action::Search(
                    crate::common::action::Search::Execute(self.text_area.lines().concat()),
                )));
            }
            KeyCode::Up => {
                return Ok(None);
            }
            KeyCode::Down => {
                return Ok(None);
            }
            _ => {
                self.text_area.input(key);
                if self.settings.as_ref().unwrap().search.always_search {
                    return Ok(Some(Action::Search(
                        crate::common::action::Search::Execute(self.text_area.lines().concat()),
                    )));
                }
                return Ok(None);
            }
        }
    }

    fn handle_mouse_event(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> color_eyre::eyre::Result<Option<crate::common::action::Action>> {
        let _ = mouse; // to appease clippy
        Ok(None)
    }

    fn update(
        &mut self,
        action: crate::common::action::Action,
    ) -> color_eyre::eyre::Result<Option<crate::common::action::Action>> {
        match action {
            crate::common::action::Action::Tick => {
                // add any logic here that should run on every tick
            }
            crate::common::action::Action::Render => {
                // add any logic here that should run on every render
                self.render_tick += 1;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: Rect) -> Result<()> {
        let area = get_root_layout(area, &self.settings.as_ref().unwrap()).search_box_area;
        self.area = area;
        let theme = self.settings.as_ref().unwrap().ui.theme.clone();
        let search_theme = theme.get_search_colors();
        let gap = self.settings.as_ref().unwrap().ui.layout.gap;
        let search_settings: UISearchSettings = self.settings.as_ref().unwrap().ui.search.clone();

        let default_borders = theme.get_border_type(UISection::Search).to_border_set();
        let (visible_borders, collapsed_borders) = collapsed_border(
            UISection::Search,
            &self.settings.as_ref().unwrap().ui.layout.sections,
            default_borders,
        );

        let padding = Padding::new(
            search_settings.padding.saturating_mul(2).max(2),
            search_settings.padding.saturating_mul(2).max(2),
            search_settings.padding,
            search_settings.padding,
        );
        let block = Block::bordered()
            .title(self.settings.as_ref().unwrap().ui.layout.title.as_str())
            .title_alignment(self.settings.as_ref().unwrap().ui.layout.title_alignment)
            .title_style(Style::default().fg(self.settings.as_ref().unwrap().ui.theme.title))
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
        // block.render(area, buf);
        frame.render_widget(block, area);

        // rainbow border effect
        if self.settings.as_ref().unwrap().ui.search.rainbow_border {
            let t = self.render_tick as u32;
            let speed: f32 = self
                .settings
                .as_ref()
                .unwrap()
                .ui
                .search
                .rainbow_border_speed;
            effects::rainbow(
                search_theme.border.unwrap(),
                2000,
                speed,
                area,
                frame.buffer_mut(),
                t,
            );
        }

        //
        // Search Box text rendering
        //

        // splice the query to insert the caret
        let caret_query = self.query.clone();
        let (before_caret, after_caret) =
            caret_query.split_at(self.caret_position.min(caret_query.len()));

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
            self.post_fix.clone().as_str(),
            flash_caret,
        );

        let paragraph = Paragraph::new(search_settings.pre_query.clone())
            .alignment(self.settings.as_ref().unwrap().ui.search.text_alignment)
            .style(Style::default().bg(search_theme.background.unwrap()));

        let mut text_region = inner_area.clone();
        text_region.x += 3;
        text_region.width -= 3;

        frame.render_widget(&self.text_area, text_region);

        frame.render_widget(paragraph, inner_area);
        Ok(())
    }
}
