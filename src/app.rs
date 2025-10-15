use ratatui::{DefaultTerminal, crossterm::event};

use crate::{events::Event, model::model, ui::ui::UI};

pub struct App {
    model: model::Model,
    terminal: DefaultTerminal,
    ui: UI,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> Self {
        Self {
            model: model::Model::default(),
            terminal: terminal,
            ui: UI::new(),
        }
    }

    pub fn run(&mut self) {
        // Main application loop
        self.model.running_state = model::RunState::Running;

        let applications = crate::applications::find_desktop_files();
        self.model.data.applications = applications;
        self.model.ui.executing_item = None;

        while self.model.is_running() {
            let pre_time = std::time::Instant::now();

            self.update();
            // Handle events and update the model
            // Render the UI
            self.terminal
                .draw(|f| self.ui.draw(&mut self.model, f))
                .unwrap();

            let post_time = std::time::Instant::now();
            let frame_duration = post_time.duration_since(pre_time);
            self.model.delta_time = frame_duration.as_millis() as i32;
            self.model.time += frame_duration.as_secs() as i32;
            self.model.tick += 1;

            self.model.ui.tick = self.model.tick;
            self.model.ui.delta_time = self.model.delta_time;
            self.model.ui.time = self.model.time;
        }
    }

    fn update(&mut self) {
        // Update the app state
        let events = self.handle_events();

        for e in events {
            match e {
                Event::Quit => {
                    self.model.running_state = model::RunState::Stopped;
                    // println!("Quitting application...");
                }
                Event::SearchAdd(c) => {
                    let (pre_query, post_query) = self
                        .model
                        .search
                        .split_at_caret(self.model.ui.caret_position);
                    self.model.search.query = format!("{}{}{}", pre_query, c, post_query);
                    self.model.ui.caret_position += 1;
                    // self.model.search.query.push(c);
                }
                Event::SearchRemove(x) => {
                    let (pre_query, post_query) = self
                        .model
                        .search
                        .split_at_caret(self.model.ui.caret_position);
                    if x < 0 {
                        // Remove behind cursor (backspace behavior)
                        let chars_to_remove = x.unsigned_abs() as usize;
                        if pre_query.len() >= chars_to_remove {
                            let new_pre_len = pre_query.len() - chars_to_remove;
                            self.model.search.query =
                                format!("{}{}", &pre_query[..new_pre_len], post_query);
                            self.model.ui.caret_position =
                                self.model.ui.caret_position.saturating_sub(chars_to_remove);
                        } else if !pre_query.is_empty() {
                            self.model.search.query = post_query.to_string();
                            self.model.ui.caret_position = 0;
                        }
                    } else if x > 0 {
                        // Remove in front of cursor (delete behavior)
                        let chars_to_remove = x as usize;
                        if post_query.len() >= chars_to_remove {
                            self.model.search.query =
                                format!("{}{}", pre_query, &post_query[chars_to_remove..]);
                        } else if !post_query.is_empty() {
                            self.model.search.query = pre_query.to_string();
                        }
                    }
                }
                Event::SearchExecute => {
                    let query = self.model.search.query.trim();
                    // ignore empty queries
                    if query.is_empty() {
                        self.model.search.results.clear();
                    }

                    let result = crate::model::search::sort_applications(
                        &mut self.model.data.applications,
                        query,
                    );
                    self.model.search.results = result;
                    self.model.ui.result_list_state.select(Some(0));
                }

                Event::NavigateLeft(x) => {
                    if (self.model.ui.caret_position - x + 1) > 0 {
                        self.model.ui.caret_position -= x;
                    }
                }
                Event::NavigateRight(x) => {
                    if self.model.ui.caret_position + x < self.model.search.query.len() + 1 {
                        self.model.ui.caret_position += x;
                    }
                }
                Event::NavigateDown(x) => {
                    let current = self.model.ui.result_list_state.selected().unwrap_or(0);
                    let max_index = self.model.search.results.len().saturating_sub(1);
                    let new_index = current.saturating_add(x as usize);

                    if self.model.settings.ui.results.loopback && new_index > max_index {
                        self.model.ui.result_list_state.select(Some(0));
                    } else {
                        self.model
                            .ui
                            .result_list_state
                            .select(Some(new_index.min(max_index)));
                    }
                }
                Event::NavigateUp(x) => {
                    let current = self.model.ui.result_list_state.selected().unwrap_or(0);
                    let max_index = self.model.search.results.len().saturating_sub(1);
                    let new_index = current.saturating_sub(x as usize);

                    if self.model.settings.ui.results.loopback && current < x as usize {
                        self.model.ui.result_list_state.select(Some(max_index));
                    } else {
                        self.model.ui.result_list_state.select(Some(new_index));
                    }
                }
                Event::NavigateHome => {
                    self.model.ui.caret_position = 0;
                }
                Event::NavigateEnd => {
                    self.model.ui.caret_position = self.model.search.query.len();
                }
                Event::AppExecute(index) => {
                    // Execute the selected application
                    // if is valid index
                    if let Some(app) = self.model.data.applications.get(
                        self.model
                            .search
                            .results
                            .get(index)
                            .map(|(_, idx)| *idx)
                            .unwrap_or(0),
                    ) {
                        self.model.ui.executing_item = Some(index);
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
    }

    fn handle_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        if event::poll(std::time::Duration::from_millis(250)).unwrap() {
            // println!("Event polled");
            if let event::Event::Key(key) = event::read().unwrap() {
                events.extend(self.handle_key_event(key));
            }
            // if let event::Event::Mouse(mouse) = event::read().unwrap() {
            //     events.extend(self.handle_mouse_event(mouse));
            // }
            // Drain any additional immediately-available events without blocking
            while event::poll(std::time::Duration::from_millis(0)).unwrap() {
                if let Ok(ev) = event::read() {
                    match ev {
                        event::Event::Key(k) => events.extend(self.handle_key_event(k)),

                        event::Event::Resize(_, _) => { /* handle resize if needed */ }
                        _ => {}
                    }
                }
            }
        }
        events
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Vec<Event> {
        // TODO! use config to map keys to actions
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
                            events.push(Event::SearchAdd('q'));
                        }
                    }
                    event::KeyCode::Backspace => {
                        events.push(Event::SearchRemove(-1));
                        // if always search, execute the search event immediately
                        if self.model.settings.search.always_search {
                            events.push(Event::SearchExecute);
                        }
                    }
                    event::KeyCode::Delete => {
                        events.push(Event::SearchRemove(1));
                        // if always search, execute the search event immediately
                        if self.model.settings.search.always_search {
                            events.push(Event::SearchExecute);
                        }
                    }
                    event::KeyCode::Enter => {
                        events.push(Event::AppExecute(
                            self.model.ui.result_list_state.selected().unwrap_or(0),
                        ));
                    }
                    event::KeyCode::Esc => {
                        events.push(Event::SearchCancel);
                    }
                    event::KeyCode::Left => {
                        events.push(Event::NavigateLeft(1));
                    }
                    event::KeyCode::Right => {
                        events.push(Event::NavigateRight(1));
                    }
                    event::KeyCode::Up => {
                        events.push(Event::NavigateUp(1));
                    }
                    event::KeyCode::Down => {
                        events.push(Event::NavigateDown(1));
                    }
                    event::KeyCode::Tab => {
                        events.push(Event::NavigateDown(1));
                    }
                    event::KeyCode::BackTab => {
                        events.push(Event::NavigateUp(1));
                    }
                    event::KeyCode::PageUp => {
                        events.push(Event::NavigateUp(1));
                    }
                    event::KeyCode::PageDown => {
                        events.push(Event::NavigateDown(1));
                    }
                    event::KeyCode::Home => {
                        events.push(Event::NavigateHome);
                    }
                    event::KeyCode::End => {
                        events.push(Event::NavigateEnd);
                    }
                    _ => {
                        let key = match key_event.code {
                            event::KeyCode::Char(c) => c,
                            _ => '\0',
                        };

                        let modifiers = key_event.modifiers;
                        if modifiers.contains(event::KeyModifiers::CONTROL)
                            || matches!(key, '1'..='9')
                        {
                            println!("Executing application for key: {:?}", key_event);
                            let idx = if matches!(key, '1'..='9') {
                                (key as u8 - b'1') as usize
                            } else {
                                0
                            };
                            events.push(Event::AppExecute(idx));
                        } else {
                            events.push(Event::SearchAdd(key));
                            // if always search, execute the search event immediately
                            if self.model.settings.search.always_search {
                                events.push(Event::SearchExecute);
                            }
                        }
                        // println!("Key Pressed: {:?}", key_event);
                    }
                }
            }
            _ => {}
        }
        events
    }
}
