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
    Clear,
    Execute,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    Start,
    Pause,
    Quit,
    //
    KeyPress(KeyEvent),
    KeyUp(KeyEvent),
    KeyDown(KeyEvent),
    KeyHeld(KeyEvent, u16), // KeyEvent, duration in ms
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
    Select,
    Back,
    MouseMove(u16, u16),        // x, y
    MousePress(u16, u16),       // x, y
    MouseDoubleClick(u16, u16), // x, y
    MouseScrollUp(u16, u16),    // x, y
    MouseScrollDown(u16, u16),  // x, y

    Tick,
}

pub fn process_events(app_events: &Vec<event::Event>, settings: &Settings) -> Vec<Event> {
    let mut events = Vec::new();

    for event in app_events {
        match event {
            event::Event::Key(key) => events.extend(process_key_events(settings, *key)),
            _ => {}
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
                    if modifiers.contains(event::KeyModifiers::CONTROL) || matches!(key, '1'..='9')
                    {
                        println!("Executing application for key: {:?}", key_event);
                        let idx = if matches!(key, '1'..='9') {
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
