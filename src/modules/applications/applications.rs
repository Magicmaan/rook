use crate::{
    model::module::{Data, ModuleState, Result, UIState},
    modules::module::Module,
};

pub struct ApplicationModule {
    pub settings: crate::settings::settings::Settings,
    state: ModuleState,
}

impl ApplicationModule {
    pub fn new(settings: &crate::settings::settings::Settings) -> Self {
        let mut state = ModuleState::default();
        let applications = crate::modules::applications::desktop::find_desktop_files();
        state.data.applications = applications;
        Self {
            settings: settings.clone(),
            state,
        }
    }
}

impl Module for ApplicationModule {
    type State = ModuleState;
    type Data = Data;

    fn get_state(&mut self) -> &mut Self::State {
        &mut self.state
    }

    fn on_search(&mut self, query: &str) {
        // let query = self.state.search.query.trim();
        // ignore empty queries
        if query.is_empty() {
            self.state.search.results.clear();
        }

        let result = crate::modules::applications::desktop::sort_applications(
            &mut self.state.data.applications,
            query,
        );

        self.state.search.previous_query = query.to_string();
        self.state.search.previous_results = result.clone();
        self.state.search.results = result;
        self.state.ui.set_selected_result_index(0);
    }
    fn on_execute(&mut self) {
        let index = self.state.ui.get_selected_result_index();
        if let Some((_, app_index)) = self.state.search.results.get(index) {
            if let Some(app) = self.state.data.applications.get(*app_index) {
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

                let app = self.state.data.applications.get(idx).unwrap();

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

        &mut self.state.ui
    }
}
