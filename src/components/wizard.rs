use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEventKind, MouseEventKind};
use std::cmp::min;
use std::result;
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::common::action::Action;
use crate::common::module_state::UISection;
// use crate::common::module_state::{SearchResult, UISection};
use crate::components::Component;
use crate::components::layout::get_root_layout;
use crate::effects;
use crate::search_modules::SearchResult;

use crate::components::util::{IconMode, collapsed_border, number_to_icon};
use crate::settings::settings::{Settings, UIResultsSettings};
use ratatui::layout::{Constraint, Layout, Margin, Offset, Position};
use ratatui::symbols;
use ratatui::widgets::{Borders, ListState, Paragraph};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Padding, StatefulWidget, Widget},
};
use serde_json::Number;
use tachyonfx::{Duration, EffectManager, EffectTimer, Interpolation, fx, pattern};

#[derive(Debug, Default, Clone)]
pub struct ResultBoxState {
    pub results: Vec<SearchResult>,
    pub previous_results: Vec<SearchResult>,

    pub executing_item: Option<usize>,
    pub list_state: ListState,
    pub last_search_tick: u64,
    pub tick: u64,
    pub delta_time: i32,
    pub total_potential_results: usize, // not the number of results shown, but the total of potential i.e. 1500 applications, but only 10 relevant shown
}

#[derive(Clone)]
pub struct WizardBox {
    settings: Option<Settings>,
    render_tick: u64,
    last_search_tick: u64,
    delta_time: i32,

    list_state: ScrollViewState,
    action_tx: Option<tokio::sync::mpsc::UnboundedSender<Action>>,
    focused: bool,
    area: Rect,
}

impl WizardBox {
    pub fn new() -> Self {
        Self {
            settings: None,
            render_tick: 0,
            last_search_tick: 0,
            delta_time: 0,

            list_state: ScrollViewState::default(),
            action_tx: None,
            focused: false,
            area: Rect::default(),
        }
    }

    fn multiply_color(&self, color: Color, mult: f64) -> Color {
        if let Color::Rgb(r, g, b) = color {
            let r = (r as f64 * mult).round().min(255.0) as u8;
            let g = (g as f64 * mult).round().min(255.0) as u8;
            let b = (b as f64 * mult).round().min(255.0) as u8;

            Color::Rgb(r, g, b)
        } else {
            color
        }
    }

    fn calculate_color_fade(&self, start_color: Color, position: usize, height: usize) -> Color {
        if let Color::Rgb(_, _, _) = start_color {
            log::trace!(
                "Calculating color fade for position {} of {}",
                position,
                height
            );
            let diff = height.saturating_sub(position);
            if diff < 5 {
                let base_brightness = 1.0;
                let brightness = 1.0
                    - maths_rs::lerp(
                        base_brightness,
                        0.25,
                        (diff as f32 / height as f32).clamp(0.1, 1.0),
                    );
                self.multiply_color(start_color, brightness as f64)
            } else {
                start_color
            }
        } else {
            log::trace!(
                "Color fade not applied for non-RGB color at position {} of {}",
                position,
                height
            );
            // indexed / ANSI colours aren't supported for fine-grained fading, so just return the
            start_color
        }
    }

    fn get_loading_spinner(&self, tick: u64) -> String {
        let remainder = tick % 4;
        if remainder == 0 {
            "◜".to_string()
        } else if remainder == 1 {
            "◝".to_string()
        } else if remainder == 2 {
            "◞".to_string()
        } else {
            "◟".to_string()
        }
    }

    pub fn construct_list(
        &self,
        results: &Vec<SearchResult>,
        number_mode: IconMode,
        executing_item: Option<usize>,
        list_state: &ListState,
        area: Rect,
        tick: u64,
    ) -> Vec<ListItem<'static>> {
        let theme = self
            .settings
            .as_ref()
            .unwrap()
            .ui
            .theme
            .get_results_colors();
        let available_height = area.height as usize;
        let mut i = 1;
        let items: Vec<ListItem<'static>> = results
            .iter()
            // .map(|(score, idx)| {
            .map(|r| {
                let result = &r.result;
                let score = &r.score;

                // space out sections to fit the width
                // let app = &state.data.applications[*idx];

                let score_text = score.to_string();

                // get number icon
                // mode configurable in settings
                let mut prepend_icon = number_to_icon(i, number_mode);
                // let executing_item = state.executing_item;
                // if executing, use loading spinner
                if executing_item.is_some() && i == executing_item.unwrap() + 1 {
                    prepend_icon = self.get_loading_spinner(tick);
                }

                // pad score to end i.e. "App Name       123"
                let line_width = area.width as usize;
                let mut name_width = line_width.saturating_sub(score_text.len() - 1);
                if prepend_icon.trim().is_empty() {
                    name_width = name_width.saturating_sub(4); // extra space if no icon
                } else {
                    name_width = name_width.saturating_sub(prepend_icon.len() + 1); // +1 for space
                }
                let padded_name = format!("{:<width$}", result, width = name_width);

                let mut text_color = theme.text.unwrap();
                let mut muted_color = theme.text_muted.unwrap();

                // calculate list color fade
                if self
                    .settings
                    .as_ref()
                    .unwrap()
                    .ui
                    .results
                    .fade_color_at_bottom
                    && available_height >= 10
                {
                    text_color = self.calculate_color_fade(
                        theme.text.unwrap(),
                        i.saturating_sub(list_state.offset()),
                        available_height,
                    );
                    muted_color = self.calculate_color_fade(
                        theme.text_muted.unwrap(),
                        i.saturating_sub(list_state.offset()),
                        available_height,
                    );
                }

                // construct line
                let line = Line::from(vec![
                    // number index
                    Span::styled(
                        format!("{} ", prepend_icon),
                        Style::default().fg(theme.accent.unwrap()),
                    ),
                    Span::styled(padded_name.clone(), Style::default().fg(text_color)), // name
                    if self.settings.as_ref().unwrap().ui.results.show_scores {
                        Span::styled(score_text.clone(), Style::default().fg(muted_color))
                    } else {
                        Span::raw("")
                    },
                ]);
                i += 1;
                ListItem::new(line)
            })
            .collect::<Vec<ListItem>>();

        items
    }
}

impl Component for WizardBox {
    fn area(&self) -> Rect {
        self.area
    }
    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<crate::common::action::Action>,
    ) -> color_eyre::eyre::Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_settings_handler(&mut self, settings: Settings) -> Result<()> {
        self.settings = Some(settings);
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<Option<crate::common::action::Action>> {
        if key.kind != KeyEventKind::Press {
            return Ok(None);
        }
        match key.code {
            KeyCode::Down => {
                if self.focused {
                    self.list_state.scroll_down();
                }
                Ok(None)
            }
            KeyCode::Up => {
                if self.focused {
                    self.list_state.scroll_up();
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }
    fn handle_mouse_event(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> Result<Option<Action>> {
        match mouse.kind {
            crossterm::event::MouseEventKind::ScrollDown => {
                if self.focused {
                    self.list_state.scroll_down();
                }
                Ok(None)
            }
            crossterm::event::MouseEventKind::ScrollUp => {
                if self.focused {
                    self.list_state.scroll_up();
                }

                Ok(None)
            }

            _ => Ok(None),
        }
    }

    fn update(
        &mut self,
        action: crate::common::action::Action,
    ) -> color_eyre::eyre::Result<Option<crate::common::action::Action>> {
        match action {
            Action::Render => {
                self.render_tick = self.render_tick.saturating_add(1);
                self.delta_time = 16; // assume ~60fps for now
            }
            Action::Focus => {
                log::trace!("Wizard box focused");
                self.focused = true;
            }
            Action::Unfocus => {
                log::trace!("Wizard box unfocused");
                self.focused = false;
            }

            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: Rect) -> Result<()> {
        let area = get_root_layout(area, &self.settings.as_ref().unwrap()).wizard_box_area;
        self.area = area;
        let results_settings: UIResultsSettings =
            self.settings.as_ref().unwrap().ui.results.clone();
        let theme = self.settings.as_ref().unwrap().ui.theme.clone();
        let results_theme = theme.get_results_colors();

        let gap = self.settings.as_ref().unwrap().ui.layout.gap;
        let padding = results_settings.padding;
        let default_borders = theme.get_border_type(UISection::Results).to_border_set();

        let (visible_borders, collapsed_borders) = collapsed_border(
            UISection::Results,
            &self.settings.as_ref().unwrap().ui.layout.sections,
            default_borders,
        );
        let root = Block::bordered()
            .border_set(if gap > 0 {
                default_borders
            } else {
                collapsed_borders
            })
            .border_style(
                self.settings
                    .as_ref()
                    .unwrap()
                    .ui
                    .theme
                    .get_default_border_style(Some(UISection::Results)),
            )
            // .border_type(theme.get_border_type("results"))
            .borders(if gap > 0 {
                Borders::ALL
            } else {
                visible_borders
            })
            .padding(Padding::new(0, padding, padding, padding))
            .title("Wizard");
        let inner = root.inner(area);
        frame.render_widget(root, area);

        let mut content_rect = inner.clone();
        content_rect.height += 20;

        let mut scroll_view = ScrollView::new(content_rect.as_size())
            .horizontal_scrollbar_visibility(tui_scrollview::ScrollbarVisibility::Never);

        let constraints = (0..content_rect.height.saturating_sub(5) as u16)
            .map(|_| Constraint::Length(1))
            .collect::<Vec<_>>();
        let layout = Layout::vertical(constraints);
        let chunks = layout.split(content_rect);

        chunks.iter().enumerate().for_each(|(i, chunk)| {
            let paragraph =
                Paragraph::new(format!("test_item {}", i)).block(Block::default().style(
                    Style::default().bg(if i % 2 == 0 { Color::Blue } else { Color::Cyan }),
                ));
            scroll_view.render_widget(paragraph, *chunk);
        });
        let mut state = ScrollViewState::default();

        scroll_view.render(inner, frame.buffer_mut(), &mut self.list_state);

        Ok(())
    }
}
