use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEventKind, MouseEventKind};
use std::cmp::min;
use std::rc::Rc;
use std::result;
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::action::Action;
use crate::common::module_state::UISection;
// use crate::common::module_state::{SearchResult, UISection};
use crate::components::Component;
use crate::components::layout::get_root_layout;
use crate::components::list::{List, ListState};
use crate::effects;
use crate::search_modules::ListResult;

use crate::components::util::{IconMode, collapsed_border, number_to_icon};
use crate::settings::settings::{Settings, UIResultsSettings};
use crate::tui::Event;
use ratatui::layout::{Constraint, Layout, Margin, Offset, Position};
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

#[derive(Clone)]
pub struct WizardBox {
    settings: Option<Settings>,
    render_tick: u64,
    delta_time: i32,

    list_state: ListState,
    action_tx: Option<tokio::sync::mpsc::UnboundedSender<Action>>,
    focused: bool,
    area: Rect,
}

impl WizardBox {
    pub fn new() -> Self {
        Self {
            settings: None,
            render_tick: 0,
            delta_time: 0,

            list_state: ListState::default(),
            action_tx: None,
            focused: false,
            area: Rect::default(),
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
    fn focus_area(&self) -> crate::app::FocusArea {
        crate::app::FocusArea::WizardBox
    }
    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<crate::action::Action>,
    ) -> color_eyre::eyre::Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_settings_handler(&mut self, settings: Settings) -> Result<()> {
        self.settings = Some(settings);
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

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        if !self.focused {
            return Ok(None);
        }
        return self.list_state.handle_key_event(&key);
    }
    fn handle_mouse_event(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> Result<Option<Action>> {
        if !self.focused {
            return Ok(None);
        }

        return self.list_state.handle_mouse_event(&mouse, 1);
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
            Action::Focus(focus) => {
                if focus == self.focus_area() && !self.focused {
                    self.focused = true;
                } else if focus != self.focus_area() && self.focused {
                    self.focused = false;
                    self.list_state.select(None);
                }
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
        let inner_area = root.inner(area);
        frame.render_widget(root, area);

        let results: &Vec<ListResult> = &vec![
            ListResult {
                result: "Wizard Step 1: Choose Option A".to_string(),
                score: 100,
                launch: Rc::new(|| {
                    log::info!("Launching Wizard Step 1");
                    true
                }),
            },
            ListResult {
                result: "Wizard Step 2: Configure Settings".to_string(),
                score: 90,
                launch: Rc::new(|| {
                    log::info!("Launching Wizard Step 2");
                    true
                }),
            },
            ListResult {
                result: "Wizard Step 3: Review and Confirm".to_string(),
                score: 80,
                launch: Rc::new(|| {
                    log::info!("Launching Wizard Step 3");
                    true
                }),
            },
        ];

        self.list_state.set_results(results.clone());

        // render list with state
        frame.render_stateful_widget(
            List::new(self.settings.clone().unwrap()),
            inner_area,
            &mut self.list_state,
        );
        // StatefulWidget::render(list, inner_area, frame.buffer_mut(), &mut self.list_state);

        Ok(())
    }
}
