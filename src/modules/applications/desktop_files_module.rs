use std::rc::Rc;

use crate::{
    common::{
        app_state::AppState,
        application::Application,
        module_state::{ModuleState, UIResult, UIState, UIStateUpdate},
    },
    modules::module::{Module, ModuleData},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DesktopData {
    pub applications: Vec<Application>,
}
impl ModuleData for DesktopData {}

pub struct DesktopFilesModule {
    pub settings: crate::settings::settings::Settings,
    state: ModuleState,
    data: Box<DesktopData>,
}

impl DesktopFilesModule {
    pub fn new(settings: &crate::settings::settings::Settings) -> Self {
        let state = ModuleState::default();
        let applications = crate::modules::applications::desktop::find_desktop_files();

        Self {
            settings: settings.clone(),
            state,
            data: Box::new(DesktopData { applications }),
        }
    }
}

impl Module for DesktopFilesModule {
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
            return Box::new(vec![]);
        }

        let results_formatted = Box::new(
            self.state
                .results
                .iter()
                .map(|score| {
                    let s = score.score;
                    let idx = score.index;

                    let app = self.data.applications.get(idx).unwrap();

                    let app_clone = app.clone();
                    UIResult {
                        result: app.name.clone(),
                        score: s,
                        launch: Rc::new(move || match app_clone.launch() {
                            Ok(_) => {
                                log::info!("Launched application: {}", app_clone.name);
                            }
                            Err(e) => {
                                log::error!(
                                    "Failed to launch application: {}: {}",
                                    app_clone.name,
                                    e
                                );
                            }
                        }),
                    }
                })
                .collect(),
        );

        results_formatted
    }
}
