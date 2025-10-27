use color_eyre::Section;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, Padding};
use ratatui::{DefaultTerminal, crossterm::event};
use tachyonfx::{Duration, EffectRenderer, EffectTimer, fx};

use crate::common::app_state::{self, AppState};
use crate::common::events::{self, Event};
use crate::common::module_state::{ModuleState, UISection, UIStateUpdate};
// use crate::event_handler::{process_events, update_navigation};
use crate::modules::module::{Module, ModuleData};
use crate::ui::results_box::ResultsBox;
use crate::ui::search_box::SearchBox;
use std::rc::Rc;
pub struct App {
    model: app_state::AppState,
    settings: crate::settings::settings::Settings,
    event_handler: crate::event_handler::EventHandler,
    terminal: DefaultTerminal,
    active_modules_idx: Vec<usize>,
    modules_vec: Vec<Box<dyn Module<State = ModuleState>>>,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> Self {
        log::info!("Initializing application...");

        let model = app_state::AppState::default();
        let settings = crate::settings::settings::Settings::new();

        let modules: Vec<Box<dyn Module<State = ModuleState>>> = vec![
            Box::new(
                crate::modules::applications::desktop_files_module::DesktopFilesModule::new(
                    &settings,
                ),
            ),
            Box::new(crate::modules::maths::maths_module::MathsModule::new(
                &settings,
            )),
            Box::new(crate::modules::programs::programs_module::ProgramsModule::new(&settings)),
        ];

        let event_handler = crate::event_handler::EventHandler::new(&settings);

        Self {
            model,
            settings,
            event_handler,
            terminal,
            active_modules_idx: vec![0],
            modules_vec: modules,
        }
    }

    // main application loop
    pub fn run(&mut self) {
        // Main application loop
        self.model.running_state = app_state::RunState::Running;

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
            self.model.ui.search_box_state.tick = self.model.tick;
            self.model.ui.search_box_state.delta_time = self.model.delta_time;
            self.model.ui.result_box_state.tick = self.model.tick;
            self.model.ui.result_box_state.delta_time = self.model.delta_time;
        }
    }

    // handle events and update app state
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
        let events = self.event_handler.process_events(&events);
        for e in events.iter() {
            match e {
                events::Event::Quit => {
                    self.model.running_state = app_state::RunState::Stopped;
                }
                events::Event::Search(_) => {
                    candidates = self
                        .event_handler
                        .handle_search(e, modules, &mut self.model);
                }

                Event::Navigate(_, _) => {
                    self.event_handler
                        .handle_navigation(e, &mut self.model, &self.settings);
                }
                Event::ItemExecute => {
                    let idx = self.model.ui.get_selected_result_index();
                    if let Some(result) = self.model.ui.get_results().get(idx) {
                        (result.launch)();
                    }
                }
                _ => {}
            }
        }

        // Find all candidate module indices
        let candidate_indices: Vec<usize> = candidates
            .iter()
            .filter_map(|(idx, is_candidate)| if *is_candidate { Some(*idx) } else { None })
            .collect();

        if candidate_indices.is_empty() {
            // If no candidates, keep the current active modules
            log::info!("No candidate modules found, keeping current active modules.");
            return;
        }

        self.active_modules_idx = candidate_indices;

        // Optionally, you can store or use candidate_indices elsewhere if needed
        // self.candidate_module_indices = candidate_indices;
    }

    // render the UI
    pub fn render(&mut self) {
        let ui_settings = &self.settings.ui;
        let gap = self.settings.ui.layout.gap;

        let mut ui_update = UIStateUpdate {
            post_fix: "".to_string(),
            results: vec![],
            total_potential_results: 0,
        };

        let mut results_all = vec![];
        let mut total_results = 0;
        for idx in self.active_modules_idx.iter() {
            let module = self.modules_vec[*idx].as_mut();
            let module_results = module.get_results();

            // total_results += module_results.len();
            results_all.extend_from_slice(module_results.as_slice());
        }
        results_all.sort_by(|a, b| b.score.cmp(&a.score));

        ui_update.results = results_all;
        ui_update.total_potential_results = total_results;

        self.model
            .ui
            .set_search_query(self.model.search.query.clone());

        // update search box and results box states
        self.model
            .ui
            .set_search_post_fix(ui_update.post_fix.clone());

        // if ui_update.results.is_empty() {
        //     return;
        // }
        // self.model.ui.result_box_state.previous_results = self.model.ui.get_results().clone();
        self.model.ui.set_results(ui_update.results.clone());
        // self.model.ui.result_box_state.total_potential_results = ui_update.total_potential_results;

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
                if gap > 0 || ui_settings.layout.sections.get(0) != Some(&UISection::Search) {
                    // borders take up extra space, when no gap, their is no bottom border on search box
                    // when their is a gap, extra height must be given for centering
                    search_bar_height += 1;
                }
                let mut constraints: Vec<Constraint> = vec![];
                self.settings
                    .ui
                    .layout
                    .sections
                    .iter()
                    .for_each(|section| match section {
                        UISection::Search => {
                            constraints.push(Constraint::Length(search_bar_height))
                        }
                        UISection::Results => constraints.push(Constraint::Fill(0)),
                        // UISection::Tooltip => constraints.push(Constraint::Length(1)),
                    });
                let mut spaced_constraints: Vec<Constraint> = Vec::new();
                for (i, constraint) in constraints.into_iter().enumerate() {
                    spaced_constraints.push(constraint);
                    if i < self.settings.ui.layout.sections.len() - 1 {
                        spaced_constraints.push(Constraint::Length(gap.saturating_sub(1)));
                    }
                }
                constraints = spaced_constraints;
                let layout = Layout::vertical(constraints);
                let chunks: Rc<[Rect]> = layout.split(area);

                frame.render_stateful_widget(
                    SearchBox::new(&self.settings),
                    chunks[self
                        .settings
                        .ui
                        .layout
                        .sections
                        .iter()
                        .position(|s| *s == UISection::Search)
                        .map(|i| i * 2)
                        .unwrap_or(0)],
                    &mut self.model.ui.search_box_state,
                );
                frame.render_stateful_widget(
                    ResultsBox::new(&self.settings),
                    chunks[self
                        .settings
                        .ui
                        .layout
                        .sections
                        .iter()
                        .position(|s| *s == UISection::Results)
                        .map(|i| i * 2)
                        .unwrap_or(0)],
                    &mut self.model.ui.result_box_state,
                );
            })
            .unwrap();
    }
}
