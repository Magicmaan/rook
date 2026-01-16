use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{ListItem, StatefulWidget},
};

use crate::{
    action::Action,
    components::{
        list,
        util::{IconMode, calculate_color_fade, loading_spinner, number_to_icon},
    },
    search_modules::ListResult,
    settings,
    tui::{self, Event},
};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct ListState {
    offset: usize,
    selected: Option<usize>,
    area: Rect,
    results: Option<Vec<ListResult>>,
}
impl ListState {
    pub fn scroll_up_by(&mut self, amount: u16) {
        let selected = self.selected.unwrap_or_default();
        self.select(Some(selected.saturating_sub(amount as usize)));
    }
    pub fn scroll_down_by(&mut self, amount: u16) {
        let selected = self.selected.unwrap_or_default();
        self.select(Some(selected.saturating_add(amount as usize)));
    }
    pub fn select_last(&mut self) {
        self.select(Some(usize::MAX));
    }
    pub fn select_first(&mut self) {
        self.select(Some(0));
    }
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        log::info!("Selected index: {:?}", self.selected);
        if index.is_none() {
            self.offset = 0;
        }
    }
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }
    pub const fn offset(&self) -> usize {
        self.offset
    }

    pub fn set_results(&mut self, results: Vec<ListResult>) {
        self.results = Some(results);
    }
    pub fn results(&self) -> Option<&Vec<ListResult>> {
        self.results.as_ref()
    }

    // pub fn handle_events(&mut self, event: &tui::Event) -> Result<Option<Action>> {
    //     // TODO!: fix the mouse event to adjust for padding etc.
    //     if Some(&self.results()).is_none() {
    //         return Ok(None);
    //     }
    //     match event {
    //         Event::Key(key_event) => self.handle_key_event(key_event),
    //         Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
    //         _ => Ok(None),
    //     }
    // }

    pub fn handle_key_event(
        &mut self,
        key_event: &crossterm::event::KeyEvent,
    ) -> Result<Option<Action>> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(None);
        }
        match key_event.code {
            KeyCode::Up => {
                self.scroll_up_by(1);

                Ok(None)
            }
            KeyCode::Down => {
                self.scroll_down_by(1);

                Ok(None)
            }
            KeyCode::Enter => {
                if let Some(selected) = self.selected() {
                    if let Some(results) = &self.results() {
                        return Ok(Some(Action::ItemExecute(results[selected].clone())));
                    }
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn handle_mouse_event(
        &mut self,
        mouse_event: &crossterm::event::MouseEvent,
        padding: u16,
    ) -> Result<Option<Action>> {
        let results = &self.results().unwrap();
        match mouse_event.kind {
            crossterm::event::MouseEventKind::ScrollDown => {
                // if self.focused {
                log::info!("Scrolling down results box");
                self.scroll_down_by(1);
                // }
                Ok(None)
            }
            crossterm::event::MouseEventKind::ScrollUp => {
                // if self.focused {
                log::info!("Scrolling up results box");

                self.scroll_up_by(1);
                // }
                Ok(None)
            }
            MouseEventKind::Moved => {
                log::info!("Mouse moved in results box");
                log::info!("Mouse at {}, {}", mouse_event.column, mouse_event.row);
                let relative_y = mouse_event.row.saturating_sub(self.area.y);
                if !self.area.contains(Position {
                    x: mouse_event.column,
                    y: mouse_event.row,
                }) {
                    self.select(None);
                    return Ok(None);
                }
                let index = relative_y as usize + self.offset();
                log::info!("Calculated index: {}", index);
                if index < results.len() {
                    self.select(Some(index));
                } else {
                    self.select(None);
                }
                // }
                Ok(None)
            }
            MouseEventKind::Down(button) => {
                if button == MouseButton::Right {
                    log::trace!("Right click, ignoring");
                    return Ok(None);
                }
                if button == MouseButton::Middle {
                    log::trace!("Middle click, ignoring");
                    return Ok(None);
                }
                if !self.area.contains(Position {
                    x: mouse_event.column,
                    y: mouse_event.row,
                }) {
                    return Ok(None);
                }
                if let Some(selected) = self.selected() {
                    if let Some(results) = &self.results() {
                        return Ok(Some(Action::ItemExecute(results[selected].clone())));
                    }
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}

#[derive(Default, Clone)]
pub struct List {
    settings: Option<crate::settings::settings::Settings>,
}

impl List {
    pub fn new(settings: crate::settings::settings::Settings) -> Self {
        Self {
            settings: Some(settings),
        }
    }

    pub fn construct_list(
        list_state: &ListState,
        settings: &crate::settings::settings::Settings,
        number_mode: IconMode,
        executing_item: Option<usize>,
        area: Rect,
        tick: u64,
    ) -> Vec<ListItem<'static>> {
        let theme = settings.ui.theme.get_results_colors();
        let available_height = area.height as usize;
        let mut i = 1;
        let results = list_state.results().unwrap();
        let items: Vec<ListItem<'static>> = results
            .iter()
            // .map(|(score, idx)| {
            .map(|item| {
                let result = &item.result;
                // let score = &r.score;
                let score = &item.score.to_string();
                let mut text_color = theme.text.unwrap();
                let mut muted_color = theme.text_muted.unwrap();
                let mut selected_color = theme.accent.unwrap();

                // get number icon
                // mode configurable in settings
                let mut prepend_icon = number_to_icon(i, number_mode);
                // if executing, use loading spinner
                if executing_item.is_some() && i == executing_item.unwrap() + 1 {
                    prepend_icon = loading_spinner(tick);
                }

                // pad score to end i.e. "App Name       123"
                let line_width = area.width as usize;
                let mut name_width = line_width.saturating_sub(score.len() - 1);
                if prepend_icon.trim().is_empty() {
                    name_width = name_width.saturating_sub(4); // extra space if no icon
                } else {
                    name_width = name_width.saturating_sub(prepend_icon.len() + 1); // +1 for space
                }
                let padded_name = format!("{:<width$}", result, width = name_width);

                // calculate list color fade
                if settings.ui.results.fade_color_at_bottom && available_height >= 10 {
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
                        Style::default().fg(selected_color),
                    ),
                    Span::styled(padded_name.clone(), Style::default().fg(text_color)), // name
                    if settings.ui.results.show_scores {
                        Span::styled(score.clone(), Style::default().fg(muted_color))
                    } else {
                        Span::raw("")
                    },
                ])
                .style(Style::default().bg(
                    if list_state.selected() == Some(i.saturating_sub(1)) {
                        selected_color
                    } else {
                        theme.background.unwrap()
                    },
                ));
                i += 1;
                ListItem::new(line)
            })
            .collect::<Vec<ListItem>>();

        items
    }
}

impl StatefulWidget for List {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.area = area;
        // rendering logic here

        let items = Self::construct_list(
            state,
            self.settings.as_ref().unwrap(),
            self.settings.as_ref().unwrap().ui.results.number_mode,
            None,
            area,
            0,
        );

        let list = ratatui::widgets::List::new(items);
        let mut true_state = ratatui::widgets::ListState::default().with_offset(state.offset());
        true_state.select(state.selected());
        // let state = ratatui::widgets::ListState::default();
        list.render(area, buf, &mut true_state);
    }
}
