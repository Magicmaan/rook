use maths_rs::vec;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Padding};
use ratatui::{DefaultTerminal, crossterm::event};

use crate::events::{self, Event, process_events, update_navigation};
use crate::model::model;
use crate::model::module::ModuleState;
use crate::modules::module::Module;
use crate::ui::results_box::ResultsBox;
use crate::ui::search_box::SearchBox;
use std::rc::Rc;

pub struct App {
    model: model::Model,
    settings: crate::settings::settings::Settings,
    terminal: DefaultTerminal,
    active_module_idx: usize,
    modules_vec: Vec<Box<dyn Module>>,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> Self {
        log::info!("Initializing application...");

        let model = model::Model::default();
        let settings = crate::settings::settings::Settings::new();

        let modules: Vec<Box<dyn Module>> = vec![
            Box::new(
                crate::modules::applications::desktop_files_module::DesktopFilesModule::new(
                    &settings,
                ),
            ),
            Box::new(crate::modules::maths::maths_module::MathsModule::new(
                &settings,
            )),
        ];

        Self {
            model,
            settings,
            terminal,
            active_module_idx: 0,
            modules_vec: modules,
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
        let ui_settings = &self.settings.ui;
        let gap = self.settings.ui.layout.gap;
        let module = self.modules_vec[self.active_module_idx].as_mut();
        self.terminal
            .draw(|frame| {
                let padding = self.settings.ui.layout.padding;
                let root = Block::new()
                    .style(ui_settings.theme.get_default_style(None))
                    .padding(Padding::new(
                        padding.saturating_mul(2),
                        padding.saturating_mul(2),
                        padding,
                        padding,
                    ));

                let area = root.inner(frame.area());
                frame.render_widget(root, frame.area());

                // create layout with search box, and result
                // takes in gap from settings, and adds extra space in between
                // the connection of borders is handled in results_box and search_box
                let mut search_bar_height = 2 + ui_settings.search.padding.saturating_mul(2);
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
                let state = module.render();

                frame.render_stateful_widget(
                    SearchBox::new(&self.settings),
                    chunks[0],
                    &mut state.search_box_state,
                );
                frame.render_stateful_widget(
                    ResultsBox::new(&self.settings),
                    chunks[2],
                    &mut state.result_box_state,
                );
            })
            .unwrap();
    }

    fn update(&mut self) {
        let modules = &mut self.modules_vec;

        // idx, candidacy
        let mut candidates: Vec<(usize, bool)> = vec![];
        // let module = &mut modules[self.active_module_idx];

        let mut events: Vec<event::Event> = Vec::new();
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            events.push(event::read().unwrap());
        }
        // Update the app state
        let events = process_events(&events, &self.settings);
        for e in events.iter() {
            match e {
                Event::Quit => {
                    self.model.running_state = model::RunState::Stopped;
                }
                Event::Search(search_event) => {
                    // let query = module.get_state().search.query.trim().to_string();
                    // for m in modules.iter_mut() {
                    //     let candidacy = update_search(&e, &mut **m);
                    // }
                    match search_event {
                        events::Search::Execute => {
                            for (i, m) in modules.iter_mut().enumerate() {
                                let query = m.get_state().search.query.trim().to_string();
                                let candidacy = m.on_search(&query, &self.model);
                                candidates.push((i, candidacy));
                            }
                        }
                        _ => {
                            for m in modules.iter_mut() {
                                update_search(&e, &mut **m);
                            }
                        }
                    }
                }

                Event::Navigate(_, _) => {
                    for m in modules.iter_mut() {
                        update_navigation(&e, &mut m.get_state(), &self.settings);
                    }
                } // _ => {
                //     self.modules.update(&events, &mut self.model);
                // }
                Event::ItemExecute => {
                    modules[self.active_module_idx].on_execute(&self.model);
                }
                _ => {}
            }
        }

        let mut new_active_module_idx = candidates
            .iter()
            .find(|(_, is_candidate)| *is_candidate)
            .map(|(idx, _)| *idx)
            .unwrap_or(self.active_module_idx);
        self.active_module_idx = new_active_module_idx;
        // self.modules.update(&events, &mut self.model);
    }
}

fn update_search(event: &Event, module: &mut dyn Module) {
    let state: &mut ModuleState = module.get_state();
    if let Event::Search(search_event) = event {
        match search_event {
            events::Search::Add(c) => {
                let (pre_query, post_query) =
                    state.search.split_at_caret(state.ui.get_caret_position());
                state.search.query = format!("{}{}{}", pre_query, c, post_query);
                state
                    .ui
                    .set_caret_position(state.ui.get_caret_position() + 1);
                // app_state.search.query.push(c);
            }
            events::Search::Remove(x) => {
                let (pre_query, post_query) =
                    state.search.split_at_caret(state.ui.get_caret_position());
                if x < &0 {
                    // Remove behind cursor (backspace behavior)
                    let chars_to_remove = x.unsigned_abs() as usize;
                    if pre_query.len() >= chars_to_remove {
                        let new_pre_len = pre_query.len() - chars_to_remove;
                        state.search.query = format!("{}{}", &pre_query[..new_pre_len], post_query);
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
                    let chars_to_remove = x.clone() as usize;
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
}
