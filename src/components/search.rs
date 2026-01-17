use crate::{
    action::Action,
    common::module_state::UISection,
    components::{Component, util::collapsed_border},
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

#[derive(Clone)]
pub struct SearchBox {
    settings: Option<Settings>,
    render_tick: u64,
    text_area: TextArea<'static>,
    area: Rect,
    focused: bool,
    root_layout: crate::common::layout::RootLayout,
}

impl SearchBox {
    pub fn new() -> Self {
        Self {
            settings: None,
            render_tick: 0,
            focused: true,
            text_area: TextArea::default(),
            area: Rect::default(),
            root_layout: crate::common::layout::RootLayout::default(),
        }
    }
}

impl Component for SearchBox {
    fn area(&self) -> Rect {
        self.area
    }
    fn focus_area(&self) -> crate::app::FocusArea {
        crate::app::FocusArea::Search
    }
    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<crate::action::Action>,
    ) -> color_eyre::eyre::Result<()> {
        let _ = tx; // to appease clippy
        Ok(())
    }

    fn register_settings_handler(&mut self, settings: Settings) -> color_eyre::eyre::Result<()> {
        self.settings = Some(settings); // to appease clippy
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        // key handling is different for OS. windows sends press/release events seperately
        if key.kind != KeyEventKind::Press {
            return Ok(None);
        }
        if !self.focused {
            return Ok(None);
        }
        match key.code {
            KeyCode::Enter => {
                return Ok(Some(Action::Search(crate::action::Search::Execute(
                    self.text_area.lines().concat(),
                ))));
            }
            KeyCode::Up => {
                return Ok(None);
            }
            KeyCode::Down => {
                return Ok(None);
            }
            KeyCode::BackTab => {
                return Ok(None);
            }
            KeyCode::Tab => {
                return Ok(None);
            }
            _ => {
                self.text_area.input(key);
                if self.settings.as_ref().unwrap().search.always_search {
                    return Ok(Some(Action::Search(crate::action::Search::Execute(
                        self.text_area.lines().concat(),
                    ))));
                }
                return Ok(None);
            }
        }
    }

    fn update(
        &mut self,
        action: crate::action::Action,
    ) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        match action {
            crate::action::Action::Tick => {
                // add any logic here that should run on every tick
            }
            crate::action::Action::Render => {
                // add any logic here that should run on every render
                self.render_tick += 1;
            }
            Action::Focus(focus) => {
                if (focus == self.focus_area()) {
                    self.focused = true;
                } else {
                    self.focused = false;
                }
            }
            Action::UpdateLayout(layout) => {
                self.root_layout = layout;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: Rect) -> Result<()> {
        let area = self.root_layout.search_box_area;
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

        let paragraph = Paragraph::new(search_settings.pre_query.clone())
            .alignment(self.settings.as_ref().unwrap().ui.search.text_alignment)
            .style(Style::default().bg(search_theme.background.unwrap()));

        let mut text_region = inner_area.clone();
        text_region.x = text_region.x.saturating_add(3);
        text_region.width = text_region.width.saturating_sub(3);

        frame.render_widget(&self.text_area, text_region);

        frame.render_widget(paragraph, inner_area);
        Ok(())
    }
}
