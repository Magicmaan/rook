use std::rc::Rc;

use crate::common::application::Application;
use crate::{
    common::{
        app_state::AppState,
        module_state::{ModuleState, UIResult, UIState, UIStateUpdate},
    },
    modules::module::{Module, ModuleData},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProgramData {
    pub applications: Vec<Application>,
}
impl ModuleData for ProgramData {}

pub struct ProgramsModule {
    settings: Rc<crate::settings::settings::Settings>,
    state: ModuleState,
    data: Box<ProgramData>,
}

impl ProgramsModule {
    pub fn new(settings: Rc<crate::settings::settings::Settings>) -> Self {
        let state = ModuleState::default();
        let programs = crate::modules::programs::programs::find_programs();

        Self {
            settings,
            state,
            data: Box::new(ProgramData {
                applications: programs,
            }),
        }
    }
}

impl Module for ProgramsModule {
    type State = ModuleState;
    fn get_state(&mut self) -> &mut ModuleState {
        &mut self.state
    }

    fn on_search(&mut self, query: &str, app_state: &AppState) -> bool {
        // ignore empty queries
        if query.is_empty() {
            return false;
        }
        let result = crate::modules::applications::desktop::sort_applications(
            &mut self.data.applications,
            query,
        );

        if result.is_empty() {
            log::info!("No applications matched the query: {}", query);
            return false;
        }

        self.state.results = result;

        log::info!(
            "Found {} applications matching the query: {}",
            self.state.results.len(),
            query
        );
        true
    }

    fn get_results(&mut self) -> Box<Vec<UIResult>> {
        if self.state.results.is_empty() {
            return Box::new(Vec::new());
        }
        Box::new(
            self.state
                .results
                .iter()
                .map(|score| {
                    let s = score.score;
                    let idx = score.index;

                    let app = self.data.applications.get(idx).unwrap().clone();

                    UIResult {
                        result: app.name.clone(),
                        score: s,
                        launch: Rc::new(move || app.launch()),
                    }
                })
                .collect(),
        )
    }
}
