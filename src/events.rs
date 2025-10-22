use mouse_position::mouse_position::Mouse;
use ratatui::crossterm::event::{self, KeyEvent};

use crate::settings::settings::Settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavigateDirection {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Search {
    Add(char),
    Remove(i8), // number of characters to remove
    Execute,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    Quit,
    //
    KeyPress(KeyEvent),
    //
    Search(Search),
    ItemExecute, // execute selected item in results
    //
    Navigate(NavigateDirection, usize), // direction, number of lines
                                        // NavigateDown(usize),                // number of lines
                                        // NavigateUp(usize),                  // number of lines
                                        // NavigateLeft(usize),                // number of lines
                                        // NavigateRight(usize),               // number of lines
                                        // NavigateHome,
                                        // NavigateEnd,
}

pub fn process_events(app_events: &Vec<event::Event>, settings: &Settings) -> Vec<Event> {
    let mut events = Vec::new();

    // log::info!("Mouse event: {:?}", _mouse_event);
    let screen_pos = match mouse_position::mouse_position::Mouse::get_mouse_position() {
        Mouse::Position { x, y } => (x, y),
        Mouse::Error => (0, 0),
    };
    log::info!("Mouse position: x={}, y={}", screen_pos.0, screen_pos.1);

    for event in app_events {
        match event {
            event::Event::Key(key) => events.extend(process_key_events(settings, *key)),
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

fn process_key_events(settings: &Settings, key_event: event::KeyEvent) -> Vec<Event> {
    let mut events = Vec::new();
    match key_event.kind {
        event::KeyEventKind::Press => {
            events.push(Event::KeyPress(key_event));
            // println!("Key Pressed: {:?}", key_event);
            match key_event.code {
                event::KeyCode::Char('q') => {
                    if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        events.push(Event::Quit);
                    } else {
                        events.push(Event::Search(Search::Add('q')));
                    }
                }
                event::KeyCode::Esc => {
                    events.push(Event::Quit);
                }
                event::KeyCode::Backspace => {
                    events.push(Event::Search(Search::Remove(-1)));
                    // if always search, execute the search event immediately
                    if settings.search.always_search {
                        events.push(Event::Search(Search::Execute));
                    }
                }
                event::KeyCode::Delete => {
                    events.push(Event::Search(Search::Remove(1)));
                    // if always search, execute the search event immediately
                    if settings.search.always_search {
                        events.push(Event::Search(Search::Execute));
                    }
                }
                event::KeyCode::Enter => {
                    events.push(Event::ItemExecute);
                }
                event::KeyCode::Left => {
                    events.push(Event::Navigate(NavigateDirection::Left, 1));
                }
                event::KeyCode::Right => {
                    events.push(Event::Navigate(NavigateDirection::Right, 1));
                }
                event::KeyCode::Up => {
                    events.push(Event::Navigate(NavigateDirection::Up, 1));
                }
                event::KeyCode::Down => {
                    events.push(Event::Navigate(NavigateDirection::Down, 1));
                }
                event::KeyCode::Tab => {
                    events.push(Event::Navigate(NavigateDirection::Down, 1));
                }
                event::KeyCode::BackTab => {
                    events.push(Event::Navigate(NavigateDirection::Up, 1));
                }
                event::KeyCode::PageUp => {
                    events.push(Event::Navigate(NavigateDirection::Up, 1));
                }
                event::KeyCode::PageDown => {
                    events.push(Event::Navigate(NavigateDirection::Down, 1));
                }
                event::KeyCode::Home => {
                    events.push(Event::Navigate(NavigateDirection::Home, 1));
                }
                event::KeyCode::End => {
                    events.push(Event::Navigate(NavigateDirection::End, 1));
                }
                _ => {
                    let key = match key_event.code {
                        event::KeyCode::Char(c) => c,
                        _ => '\0',
                    };

                    let modifiers = key_event.modifiers;
                    if modifiers.contains(event::KeyModifiers::CONTROL) && matches!(key, '0'..='9')
                    {
                        println!("Executing application for key: {:?}", key_event);
                        let _idx = if matches!(key, '1'..='9') {
                            (key as u8 - b'1') as usize
                        } else {
                            0
                        };
                        events.push(Event::ItemExecute);
                    } else {
                        events.push(Event::Search(Search::Add(key)));
                        // if always search, execute the search event immediately
                        if settings.search.always_search {
                            events.push(Event::Search(Search::Execute));
                        }
                    }
                }
            }
        }
        _ => {}
    }
    events
}

pub fn update_navigation(
    event: &Event,
    state: &mut crate::model::module::ModuleState,
    settings: &Settings,
) {
    if let Event::Navigate(direction, amount) = event {
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
                let current = state.ui.get_selected_result_index();
                let max_index = state.search.results.len().saturating_sub(1);
                let new_index = current.saturating_add(*amount as usize);

                if settings.ui.results.loopback && new_index > max_index {
                    state.ui.set_selected_result_index(0);
                } else {
                    state.ui.set_selected_result_index(new_index.min(max_index));
                }
            }
            NavigateDirection::Up => {
                let current = state.ui.get_selected_result_index();
                let new_index = current.saturating_sub(*amount as usize);

                if settings.ui.results.loopback && current < *amount as usize {
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
