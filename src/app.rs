use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Padding};
use ratatui::{DefaultTerminal, crossterm::event};

use crate::events::process_events;
use crate::modules::module::Module;
use crate::{events::Event, model::model, ui::ui::UI};
use std::rc::Rc;

pub struct App {
    model: model::Model,
    settings: crate::settings::settings::Settings,
    terminal: DefaultTerminal,
    modules: crate::modules::applications::applications::ApplicationModule,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> Self {
        let model = model::Model::default();
        let settings = crate::settings::settings::Settings::new();
        let modules = crate::modules::applications::applications::ApplicationModule::new(&settings);
        Self {
            model,
            settings,
            terminal,
            modules,
        }
    }

    // main application loop
    pub fn run(&mut self) {
        // Main application loop
        self.model.running_state = model::RunState::Running;

        while self.model.is_running() {
            let pre_time = std::time::Instant::now();

            // Handle events and pass to modules
            self.update();

            // Render the UI
            self.render();

            let post_time = std::time::Instant::now();
            let frame_duration = post_time.duration_since(pre_time);
            self.model.delta_time = frame_duration.as_millis() as i32;
            self.model.time += frame_duration.as_secs() as i32;
            self.model.tick += 1;
        }
    }

    // render the UI
    pub fn render(&mut self) {
        self.terminal
            .draw(|frame| {
                let ui_settings = &self.settings.ui;
                let gap = self.settings.ui.layout.gap;

                let root = Block::new()
                    .style(ui_settings.theme.get_default_style(None))
                    .padding(Padding::new(2, 2, 1, 1));
                let area = root.inner(frame.area());

                frame.render_widget(root, frame.area());

                // create layout with search box, and results
                // takes in gap from settings, and adds extra space in between
                // the connection of borders is handled in results_box and search_box
                let mut search_bar_height = 2;
                if gap > 0 {
                    // borders take up extra space, when no gap, their is no bottom border on search box
                    // when their is a gap, extra height must be given for centering
                    search_bar_height += 1;
                }
                let layout = Layout::vertical([
                    Constraint::Length(search_bar_height),
                    Constraint::Length(gap.saturating_sub(1)),
                    Constraint::Fill(1),
                ]);
                let chunks: Rc<[Rect]> = layout.split(area);

                // pass chunks to modules
                // in future, modules will return some sort of "has results" boolean,
                // then allowing conditional modules
                //i.e. if "zen" then render applications results,
                //i.e. if "1 + 2" then render calculator results, etc.
                self.modules.render(frame, chunks);
            })
            .unwrap();
    }

    fn update(&mut self) {
        let mut events: Vec<event::Event> = Vec::new();
        if event::poll(std::time::Duration::from_millis(250)).unwrap() {
            events.push(event::read().unwrap());
        }
        // Update the app state
        let events = process_events(&events, &self.settings);
        self.modules.update(&events, &mut self.model);
    }

    // fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Vec<Event> {
    //     // TODO! use config to map keys to actions
    //     let mut events = Vec::new();
    //     match key_event.kind {
    //         event::KeyEventKind::Press => {
    //             events.push(Event::KeyPress(key_event));
    //             // println!("Key Pressed: {:?}", key_event);
    //             match key_event.code {
    //                 event::KeyCode::Char('q') => {
    //                     if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
    //                         events.push(Event::Quit);
    //                     } else {
    //                         events.push(Event::SearchAdd('q'));
    //                     }
    //                 }
    //                 event::KeyCode::Backspace => {
    //                     events.push(Event::SearchRemove(-1));
    //                     // if always search, execute the search event immediately
    //                     if self.settings.search.always_search {
    //                         events.push(Event::SearchExecute);
    //                     }
    //                 }
    //                 event::KeyCode::Delete => {
    //                     events.push(Event::SearchRemove(1));
    //                     // if always search, execute the search event immediately
    //                     if self.settings.search.always_search {
    //                         events.push(Event::SearchExecute);
    //                     }
    //                 }
    //                 event::KeyCode::Enter => {
    //                     events.push(Event::ItemExecute(
    //                         self.model.ui.result_list_state.selected().unwrap_or(0),
    //                     ));
    //                 }
    //                 event::KeyCode::Esc => {
    //                     events.push(Event::SearchCancel);
    //                 }
    //                 event::KeyCode::Left => {
    //                     events.push(Event::NavigateLeft(1));
    //                 }
    //                 event::KeyCode::Right => {
    //                     events.push(Event::NavigateRight(1));
    //                 }
    //                 event::KeyCode::Up => {
    //                     events.push(Event::NavigateUp(1));
    //                 }
    //                 event::KeyCode::Down => {
    //                     events.push(Event::NavigateDown(1));
    //                 }
    //                 event::KeyCode::Tab => {
    //                     events.push(Event::NavigateDown(1));
    //                 }
    //                 event::KeyCode::BackTab => {
    //                     events.push(Event::NavigateUp(1));
    //                 }
    //                 event::KeyCode::PageUp => {
    //                     events.push(Event::NavigateUp(1));
    //                 }
    //                 event::KeyCode::PageDown => {
    //                     events.push(Event::NavigateDown(1));
    //                 }
    //                 event::KeyCode::Home => {
    //                     events.push(Event::NavigateHome);
    //                 }
    //                 event::KeyCode::End => {
    //                     events.push(Event::NavigateEnd);
    //                 }
    //                 _ => {
    //                     let key = match key_event.code {
    //                         event::KeyCode::Char(c) => c,
    //                         _ => '\0',
    //                     };

    //                     let modifiers = key_event.modifiers;
    //                     if modifiers.contains(event::KeyModifiers::CONTROL)
    //                         || matches!(key, '1'..='9')
    //                     {
    //                         println!("Executing application for key: {:?}", key_event);
    //                         let idx = if matches!(key, '1'..='9') {
    //                             (key as u8 - b'1') as usize
    //                         } else {
    //                             0
    //                         };
    //                         events.push(Event::ItemExecute(idx));
    //                     } else {
    //                         events.push(Event::SearchAdd(key));
    //                         // if always search, execute the search event immediately
    //                         if self.settings.search.always_search {
    //                             events.push(Event::SearchExecute);
    //                         }
    //                     }
    //                     // println!("Key Pressed: {:?}", key_event);
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    //     events
    // }
}
