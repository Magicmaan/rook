use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use std::cmp::min;
use std::result;

use crate::action::Action;
use crate::common::module_state::UISection;
// use crate::common::module_state::{SearchResult, UISection};

use crate::components::Component;
use crate::components::list::{List, ListState};
use crate::effects;
use crate::search_modules::ListResult;

use crate::components::util::{IconMode, calculate_color_fade, collapsed_border, number_to_icon};
use crate::settings::settings::{Settings, UIResultsSettings};
use crate::tui::Event;
use ratatui::layout::{Constraint, Layout, Margin, Offset};
use ratatui::symbols;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, ListItem, Padding, StatefulWidget, Widget},
};
use serde_json::Number;
use tachyonfx::{Duration, EffectManager, EffectTimer, Interpolation, fx, pattern};

#[derive(Debug, Default, Clone)]
pub struct ResultBoxState {
    pub results: Vec<ListResult>,
    pub previous_results: Vec<ListResult>,

    pub executing_item: Option<usize>,
    pub list_state: ListState,
    pub last_search_tick: u64,
    pub tick: u64,
    pub delta_time: i32,
    pub total_potential_results: usize, // not the number of results shown, but the total of potential i.e. 1500 applications, but only 10 relevant shown
}

#[derive(Clone)]
pub struct ResultsBox {
    settings: Option<Settings>,
    render_tick: u64,
    last_search_tick: u64,
    delta_time: i32,
    results: Vec<ListResult>,
    previous_results: Vec<ListResult>,
    total_potential_results: usize,
    list_state: ListState,
    // list: List,
    action_tx: Option<tokio::sync::mpsc::UnboundedSender<Action>>,
    area: Rect,
    focused: bool,
    root_layout: crate::common::layout::RootLayout,
}

impl ResultsBox {
    pub fn new() -> Self {
        Self {
            settings: None,
            render_tick: 0,
            last_search_tick: 0,
            delta_time: 0,
            results: Vec::new(),
            previous_results: Vec::new(),

            total_potential_results: 0,
            list_state: ListState::default(),
            action_tx: None,
            area: Rect::default(),
            focused: true,
            root_layout: crate::common::layout::RootLayout::default(),
            // list: List::new(),
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
        results: &Vec<ListResult>,
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
                    text_color = calculate_color_fade(
                        theme.text.unwrap(),
                        i.saturating_sub(list_state.offset()),
                        available_height,
                    );
                    muted_color = calculate_color_fade(
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

    fn execute_selected(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if let Some(action_tx) = &self.action_tx {
                action_tx
                    .send(Action::ItemExecute(self.results[selected].clone()))
                    .unwrap_or(());
            }
        }
    }
}

impl Component for ResultsBox {
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
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_settings_handler(&mut self, settings: Settings) -> Result<()> {
        self.settings = Some(settings.clone());

        Ok(())
    }
    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
        if event.is_none() {
            return Ok(None);
        }

        match event.unwrap() {
            Event::Key(key) => {
                return self.handle_key_event(key);
            }
            Event::Mouse(mouse) => {
                return self.handle_mouse_event(mouse);
            }
            _ => {}
        }

        Ok(None)
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        if (!self.focused) {
            return Ok(None);
        }
        return self
            .list_state
            .handle_key_event(&key, self.settings.as_ref().unwrap());
    }
    fn handle_mouse_event(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> Result<Option<Action>> {
        if !self.focused {
            return Ok(None);
        }
        log::info!("Mouse event received results: {:?}", mouse);
        return self
            .list_state
            .handle_mouse_event(&mouse, self.settings.as_ref().unwrap());
    }
    fn update(
        &mut self,
        action: crate::action::Action,
    ) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        match action {
            Action::Render => {
                self.render_tick = self.render_tick.saturating_add(1);
                self.delta_time = 16; // assume ~60fps for now
            }
            Action::SearchResults(results) => {
                self.last_search_tick = self.render_tick;
                self.results = results;
                self.total_potential_results = self.results.len();
                self.list_state.select(Some(0));
            }
            Action::Focus(focus) => {
                if focus == self.focus_area() && !self.focused {
                    self.focused = true;
                } else if focus != self.focus_area() && self.focused {
                    self.focused = false;
                    self.list_state.select(None);
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
        let area = self.root_layout.results_box_area;
        self.area = area;

        let results_settings: UIResultsSettings =
            self.settings.as_ref().unwrap().ui.results.clone();
        let theme = self.settings.as_ref().unwrap().ui.theme.clone();
        let results_theme = theme.get_results_colors();

        // if gap is 0, use the connected border set
        // look at: https://ratatui.rs/recipes/layout/collapse-borders/
        // i.e.
        // │       │
        // ├───────┤
        // │       │
        let gap = self.settings.as_ref().unwrap().ui.layout.gap;
        let padding = results_settings.padding;
        let default_borders = theme.get_border_type(UISection::Results).to_border_set();

        let (visible_borders, collapsed_borders) = collapsed_border(
            UISection::Results,
            &self.settings.as_ref().unwrap().ui.layout.sections,
            default_borders,
        );
        let block = Block::bordered()
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
            // .title("Search")
            .padding(Padding::new(
                padding.saturating_mul(2).max(1),
                padding.saturating_mul(2).max(1),
                padding,
                padding,
            ))
            .style(theme.get_default_style(Some(UISection::Results)));

        // block.render(area, buf);
        let mut inner_area = block.inner(area);
        frame.render_widget(block, area);
        // block.render(area, buf);

        // show number of results
        // positions it inside the padding area
        if results_settings.show_number_of_results {
            // if padding is zero, make space for number of results
            if padding == 0 {
                inner_area.height = inner_area.height.saturating_sub(1);
            }
            let mut chunk = inner_area.clone();

            if results_settings.number_of_results_position
                == crate::settings::settings::VerticalAlignment::Top
            {
                // need to adjust y position if no padding
                if padding == 0 {
                    inner_area.y += 1;
                }
                chunk.y = chunk.y.saturating_sub(padding.min(1));
                chunk.height = 1;
            } else {
                chunk.y = chunk.y.saturating_add(chunk.height);
                chunk.height = 1;
            }

            let num_results = Paragraph::new(format!(
                "{} / {}",
                self.results.len(),
                self.total_potential_results
            ))
            .style(Style::default().fg(results_theme.text_muted.unwrap()))
            .alignment(results_settings.number_of_results_alignment);
            // num_results.render(chunk, buf);
            frame.render_widget(num_results, chunk);
        }

        // rainbow border effect
        if self.settings.as_ref().unwrap().ui.results.rainbow_border {
            let t = self.render_tick as u32;
            let speed: f32 = self
                .settings
                .as_ref()
                .unwrap()
                .ui
                .results
                .rainbow_border_speed;
            effects::rainbow(
                results_theme.border.unwrap(),
                2000,
                speed,
                area,
                frame.buffer_mut(),
                t,
            );
        }

        let results: &Vec<ListResult> = &self.results;
        // let list = self.list.clone();
        self.list_state.set_results(results.clone());

        // render list with state
        frame.render_stateful_widget(
            List::new(self.settings.clone().unwrap()),
            inner_area,
            &mut self.list_state,
        );
        // StatefulWidget::render(list, inner_area, frame.buffer_mut(), &mut self.list_state);

        // fade in effect
        if self.settings.as_ref().unwrap().ui.results.fade_in {
            let mut direction: Option<pattern::AnyPattern> = None;
            if results_settings.fade_top_to_bottom {
                direction = Some(pattern::AnyPattern::Sweep(
                    pattern::SweepPattern::down_to_up(
                        min(area.height as usize, results.len()) as u16
                    ),
                ));
            }
            let tick = self.render_tick.saturating_sub(self.last_search_tick) as u32
                * self.delta_time as u32;

            effects::fade_in(
                Color::Black,
                self.settings.as_ref().unwrap().ui.results.fade_in_duration,
                direction,
                inner_area,
                frame.buffer_mut(),
                tick,
            );
        }
        Ok(())
    }
}
