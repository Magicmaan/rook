use std::rc::Rc;

use ratatui::crossterm::event::{self, KeyEvent};

use crate::{
    common::{
        app_state::AppState,
        events::{Action, NavigateDirection, Search},
        module_state::ModuleState,
    },
    modules::module::Module,
    settings::settings::Settings,
};

#[derive(Debug, Clone)]
pub struct EventHandler {
    settings: Rc<Settings>,
}

impl EventHandler {
    pub fn new(settings: Rc<crate::settings::settings::Settings>) -> Self {
        Self { settings }
    }

    // process the ratatui events into application events
    pub fn process_events(&self, app_events: &Vec<event::Event>) -> Vec<Action> {
        let mut events = Vec::new();
        for event in app_events {
            match event {
                event::Event::Key(key) => events.extend(self.process_key_events(*key)),
                event::Event::Mouse(_mouse_event) => {
                    match _mouse_event.kind {
                        event::MouseEventKind::Down(_button) => {}
                        _ => {
                            // log::info!("Mouse event: {:?}", _mouse_event);
                        }
                    }
                }
                _ => {
                    log::info!("Unhandled event: {:?}", event);
                }
            }
        }
        events
    }

    fn process_key_events(&self, key_event: event::KeyEvent) -> Vec<Action> {
        let mut events = Vec::new();

        if key_event.kind == event::KeyEventKind::Press {
            events.push(Action::KeyPress(key_event));
            // println!("Key Pressed: {:?}", key_event);
            match key_event.code {
                event::KeyCode::Char('q') => {
                    if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        events.push(Action::Quit);
                    } else {
                        events.push(Action::Search(Search::Add('q')));
                    }
                }
                event::KeyCode::Esc => {
                    events.push(Action::Quit);
                }
                event::KeyCode::Backspace => {
                    events.push(Action::Search(Search::Remove(-1)));
                    // if always search, execute the search event immediately
                    if self.settings.search.always_search {
                        events.push(Action::Search(Search::Execute));
                    }
                }
                event::KeyCode::Delete => {
                    events.push(Action::Search(Search::Remove(1)));
                    // if always search, execute the search event immediately
                    if self.settings.search.always_search {
                        events.push(Action::Search(Search::Execute));
                    }
                }
                event::KeyCode::Enter => {
                    events.push(Action::ItemExecute);
                }
                event::KeyCode::Left => {
                    events.push(Action::Navigate(NavigateDirection::Left, 1));
                }
                event::KeyCode::Right => {
                    events.push(Action::Navigate(NavigateDirection::Right, 1));
                }
                event::KeyCode::Up => {
                    events.push(Action::Navigate(NavigateDirection::Up, 1));
                }
                event::KeyCode::Down => {
                    events.push(Action::Navigate(NavigateDirection::Down, 1));
                }
                event::KeyCode::Tab => {
                    events.push(Action::Navigate(NavigateDirection::Down, 1));
                }
                event::KeyCode::BackTab => {
                    events.push(Action::Navigate(NavigateDirection::Up, 1));
                }
                event::KeyCode::PageUp => {
                    events.push(Action::Navigate(NavigateDirection::Up, 1));
                }
                event::KeyCode::PageDown => {
                    events.push(Action::Navigate(NavigateDirection::Down, 1));
                }
                event::KeyCode::Home => {
                    events.push(Action::Navigate(NavigateDirection::Home, 1));
                }
                event::KeyCode::End => {
                    events.push(Action::Navigate(NavigateDirection::End, 1));
                }
                _ => {
                    let key = match key_event.code {
                        event::KeyCode::Char(c) => c,
                        _ => '\0',
                    };

                    let modifiers = key_event.modifiers;
                    if modifiers.contains(event::KeyModifiers::CONTROL) && key.is_ascii_digit() {
                        println!("Executing application for key: {:?}", key_event);
                        let _idx = if matches!(key, '1'..='9') {
                            (key as u8 - b'1') as usize
                        } else {
                            0
                        };
                        events.push(Action::ItemExecute);
                    } else {
                        events.push(Action::Search(Search::Add(key)));
                        // if always search, execute the search event immediately
                        if self.settings.search.always_search {
                            events.push(Action::Search(Search::Execute));
                        }
                    }
                }
            }
        }
        events
    }
    //
    //
    // handle custom events
    pub fn handle_navigation(&self, event: &Action, state: &mut AppState, settings: &Settings) {
        if let Action::Navigate(direction, amount) = event {
            match direction {
                NavigateDirection::Left => {
                    if (state.ui.get_caret_position() - *amount + 1) > 0 {
                        state
                            .ui
                            .set_caret_position(state.ui.get_caret_position() - *amount);
                    }
                }
                NavigateDirection::Right => {
                    if state.ui.get_caret_position() + *amount
                        < state.ui.search_box_state.query.len() + 1
                    {
                        state
                            .ui
                            .set_caret_position(state.ui.get_caret_position() + *amount);
                    }
                }
                NavigateDirection::Down => {
                    log::info!("Navigating down by {}", amount);
                    let current = state.ui.get_selected_result_index();
                    log::info!("Current selected index: {}", current);
                    let max_index = state.ui.result_box_state.results.len().saturating_sub(1);
                    log::info!("Max index: {}", max_index);
                    let new_index = current.saturating_add(*amount);
                    log::info!("New index: {}", new_index);

                    if settings.ui.results.loopback && new_index > max_index {
                        state.ui.set_selected_result_index(0);
                    } else {
                        state.ui.set_selected_result_index(new_index.min(max_index));
                    }
                }
                NavigateDirection::Up => {
                    let current = state.ui.get_selected_result_index();
                    let new_index = current.saturating_sub(*amount);

                    if settings.ui.results.loopback && current < *amount {
                        state.ui.set_selected_result_index(0);
                    } else {
                        state.ui.set_selected_result_index(new_index);
                    }
                }
                NavigateDirection::Home => {
                    state.ui.set_caret_position(0);
                }
                NavigateDirection::End => {
                    state.ui.set_caret_position(state.search.query.len());
                }
            }
        }
    }

    pub fn handle_search(
        &self,
        event: &Action,
        modules: &mut Vec<Box<dyn Module<State = ModuleState>>>,
        state: &mut AppState,
    ) -> Vec<(usize, bool)> {
        let mut candidates: Vec<(usize, bool)> = Vec::new();

        if let Action::Search(search_event) = event {
            match search_event {
                Search::Execute => {
                    // pass execute function to all modules to determine candidacy
                    // candidacy is simply "what module should handle and display results for this query"
                    for (i, m) in modules.iter_mut().enumerate() {
                        let query = state.search.query.as_str();
                        let candidacy = m.on_search(&query, &state);
                        if candidacy {
                            state.ui.search_box_state.last_search_tick = state.tick;
                            state.ui.result_box_state.last_search_tick = state.tick;
                            state.ui.set_selected_result_index(0);
                        }
                        candidates.push((i, candidacy));
                    }
                }
                Search::Add(c) => {
                    let (pre_query, post_query) =
                        state.search.split_at_caret(state.ui.get_caret_position());
                    state.search.query = format!("{}{}{}", pre_query, c, post_query);
                    state
                        .ui
                        .set_caret_position(state.ui.get_caret_position() + 1);
                    // app_state.search.query.push(c);
                }
                Search::Remove(x) => {
                    let (pre_query, post_query) =
                        state.search.split_at_caret(state.ui.get_caret_position());
                    if x < &0 {
                        // Remove behind cursor (backspace behavior)
                        let chars_to_remove = x.unsigned_abs() as usize;
                        if pre_query.len() >= chars_to_remove {
                            let new_pre_len = pre_query.len() - chars_to_remove;
                            state.search.query =
                                format!("{}{}", &pre_query[..new_pre_len], post_query);
                            state.ui.set_caret_position(
                                state
                                    .ui
                                    .get_caret_position()
                                    .saturating_sub(chars_to_remove),
                            );
                        } else if !pre_query.is_empty() {
                            state.search.query = post_query.to_string();
                            state.ui.set_caret_position(0);
                        }
                    } else if x > &0 {
                        // Remove in front of cursor (delete behavior)
                        let chars_to_remove = *x as usize;
                        if post_query.len() >= chars_to_remove {
                            state.search.query =
                                format!("{}{}", pre_query, &post_query[chars_to_remove..]);
                        } else if !post_query.is_empty() {
                            state.search.query = pre_query.to_string();
                        }
                    }
                }
                _ => {}
            }
        }
        candidates
    }
}
