use crate::{
    model::module::{ModuleState, Result, UIState},
    modules::{applications::desktop::Application, module::Module},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub applications: Vec<Application>,
}

pub struct DesktopFilesModule {
    pub settings: crate::settings::settings::Settings,
    state: ModuleState,
    data: Data,
}

impl DesktopFilesModule {
    pub fn new(settings: &crate::settings::settings::Settings) -> Self {
        let mut state = ModuleState::default();
        let applications = crate::modules::applications::desktop::find_desktop_files();
        // state.data.applications = applications;
        Self {
            settings: settings.clone(),
            state,
            data: Data {
                applications: applications,
            },
        }
    }
}

impl Module for DesktopFilesModule {
    fn get_state(&mut self) -> &mut ModuleState {
        &mut self.state
    }

    fn on_search(&mut self, query: &str) -> bool {
        // let query = self.state.search.query.trim();
        // ignore empty queries
        if query.is_empty() {
            self.state.search.results.clear();
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

        self.state.search.previous_query = query.to_string();
        self.state.search.previous_results = self.state.search.results.clone();
        self.state.search.results = result;
        self.state.ui.set_selected_result_index(0);

        log::info!(
            "Found {} applications matching the query: {}",
            self.state.search.results.len(),
            query
        );
        log::info!("N of results: {}", self.state.search.results.len());
        true
    }
    fn on_execute(&mut self) {
        let index = self.state.ui.get_selected_result_index();
        if let Some((_, app_index)) = self.state.search.results.get(index) {
            if let Some(app) = self.data.applications.get(*app_index) {
                log::info!("Executing application: {}", app.name);
                if let Err(e) = app.launch() {
                    log::error!("Failed to launch application {}: {}", app.name, e);
                }
            }
        }
    }

    fn render(&mut self) -> &mut UIState {
        let results_formatted = self
            .state
            .search
            .results
            .iter()
            .map(|score| {
                let s = score.0;
                let idx = score.1;

                let app = self.data.applications.get(idx).unwrap();

                Result {
                    result: app.name.clone(),
                    score: s.to_string(),
                }
            })
            .collect();

        self.state.ui.result_box_state.previous_results = self.state.ui.get_results().clone();
        self.state.ui.set_results(results_formatted);

        self.state
            .ui
            .set_search_query(self.state.search.query.clone());

        // self.state.ui.set_search_post_fix("boom".to_string());
        &mut self.state.ui
    }
}
