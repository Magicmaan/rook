use ratatui::{
    Frame,
    crossterm::event,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, StatefulWidget},
};

use crate::{
    events::{self, Event, Search},
    model::{
        model::{Model, RunState},
        module::ModuleState,
    },
    modules::module::Module,
    ui::{results_box::ResultsBox, search_box::SearchBox},
};
use std::rc::Rc;

pub struct ApplicationModule {
    pub settings: crate::settings::settings::Settings,
    state: ModuleState,
}

impl ApplicationModule {
    pub fn new(settings: &crate::settings::settings::Settings) -> Self {
        let mut state = ModuleState::default();
        let applications = crate::applications::find_desktop_files();
        state.data.applications = applications;
        Self {
            settings: settings.clone(),
            state,
        }
    }
}

impl Module for ApplicationModule {
    type State = ModuleState;

    fn update_navigation(&mut self, event: &Event) {
        if let Event::Navigate(direction, amount) = event {
            match direction {
                events::NavigateDirection::Left => {
                    if (self.state.caret_position - *amount + 1) > 0 {
                        self.state.caret_position -= *amount;
                    }
                }
                events::NavigateDirection::Right => {
                    if self.state.caret_position + *amount < self.state.search.query.len() + 1 {
                        self.state.caret_position += *amount;
                    }
                }
                events::NavigateDirection::Down => {
                    let current = self.state.get_selected_result_index();
                    let max_index = self.state.search.results.len().saturating_sub(1);
                    let new_index = current.saturating_add(*amount as usize);

                    if self.settings.ui.results.loopback && new_index > max_index {
                        self.state.result_list_state.select(Some(0));
                    } else {
                        self.state
                            .result_list_state
                            .select(Some(new_index.min(max_index)));
                    }
                }
                events::NavigateDirection::Up => {
                    let current = self.state.get_selected_result_index();
                    let max_index = self.state.search.results.len().saturating_sub(1);
                    let new_index = current.saturating_sub(*amount as usize);

                    if self.settings.ui.results.loopback && current < *amount as usize {
                        self.state.result_list_state.select(Some(max_index));
                    } else {
                        self.state.result_list_state.select(Some(new_index));
                    }
                }
                events::NavigateDirection::Home => {
                    self.state.caret_position = 0;
                }
                events::NavigateDirection::End => {
                    self.state.caret_position = self.state.search.query.len();
                }
            }
        }
    }
    fn update_search(&mut self, event: &Event) {
        if let Event::Search(search_event) = event {
            match search_event {
                Search::Add(c) => {
                    let (pre_query, post_query) =
                        self.state.search.split_at_caret(self.state.caret_position);
                    self.state.search.query = format!("{}{}{}", pre_query, c, post_query);
                    self.state.caret_position += 1;
                    // app_state.search.query.push(c);
                }
                Search::Remove(x) => {
                    let (pre_query, post_query) =
                        self.state.search.split_at_caret(self.state.caret_position);
                    if x < &0 {
                        // Remove behind cursor (backspace behavior)
                        let chars_to_remove = x.unsigned_abs() as usize;
                        if pre_query.len() >= chars_to_remove {
                            let new_pre_len = pre_query.len() - chars_to_remove;
                            self.state.search.query =
                                format!("{}{}", &pre_query[..new_pre_len], post_query);
                            self.state.caret_position =
                                self.state.caret_position.saturating_sub(chars_to_remove);
                        } else if !pre_query.is_empty() {
                            self.state.search.query = post_query.to_string();
                            self.state.caret_position = 0;
                        }
                    } else if x > &0 {
                        // Remove in front of cursor (delete behavior)
                        let chars_to_remove = x.clone() as usize;
                        if post_query.len() >= chars_to_remove {
                            self.state.search.query =
                                format!("{}{}", pre_query, &post_query[chars_to_remove..]);
                        } else if !post_query.is_empty() {
                            self.state.search.query = pre_query.to_string();
                        }
                    }
                }
                Search::Execute => {
                    let query = self.state.search.query.trim();
                    // ignore empty queries
                    if query.is_empty() {
                        self.state.search.results.clear();
                    }

                    let result = crate::model::search::sort_applications(
                        &mut self.state.data.applications,
                        query,
                    );
                    self.state.search.previous_query = query.to_string();
                    self.state.search.previous_results = result.clone();
                    self.state.search.results = result;
                    self.state.result_list_state.select(Some(0));
                    self.state.search.last_search_tick = self.state.tick;
                }
                _ => {}
            }
        }
    }
    fn update(&mut self, events: &Vec<Event>, app_state: &mut Model) {
        for e in events.clone() {
            match e {
                Event::Navigate(_, _) => {
                    self.update_navigation(&e);
                }
                Event::Search(_) => {
                    self.update_search(&e);
                }
                Event::Quit => {
                    app_state.running_state = RunState::Stopped;
                    // println!("Quitting application...");
                }
                Event::ItemExecute => {
                    let index = self.state.result_list_state.selected().unwrap_or(0);
                    // Execute the selected application
                    // if is valid index
                    if let Some(app) = self.state.data.applications.get(
                        self.state
                            .search
                            .results
                            .get(index)
                            .map(|(_, idx)| *idx)
                            .unwrap_or(0),
                    ) {
                        self.state.executing_item = Some(index);
                        if let Err(e) = app.launch() {
                            eprintln!("Failed to launch application {}: {}", app.name, e);
                        } else {
                            println!("Launched application: {}", app.name);
                        }
                    }
                }

                _ => {}
            }
        }

        self.state.tick = app_state.tick;
        self.state.delta_time = app_state.delta_time;
        self.state.time = app_state.time;
    }

    fn render(&mut self, frame: &mut Frame, chunks: Rc<[Rect]>) {
        let search_box = SearchBox::new(&self.settings);

        let result_box = crate::ui::results_box::ResultsBox::new(&self.settings);

        // middle chunk is the gap
        frame.render_stateful_widget(search_box, chunks[0], &mut self.state);
        frame.render_stateful_widget(result_box, chunks[2], &mut self.state);
    }
}
