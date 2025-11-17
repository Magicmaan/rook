use std::rc::Rc;

use crate::{
    common::application::Application,
    search_modules::{ScoredResult, SearchModule, SearchResult},
    settings::settings::Settings,
};
use color_eyre::Result;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DesktopData {
    pub applications: Vec<Application>,
}

pub struct DesktopFilesModule {
    pub settings: Option<Settings>,
    results: Vec<ScoredResult>,
    data: Box<DesktopData>,
}

impl DesktopFilesModule {
    pub fn new() -> Self {
        let applications = crate::search_modules::applications::desktop::find_desktop_files();

        Self {
            settings: None,
            // state,
            results: Vec::new(),
            data: Box::new(DesktopData { applications }),
        }
    }
}

impl SearchModule for DesktopFilesModule {
    fn name(&self) -> &str {
        "desktop_files_module"
    }
    fn register_settings_handler(&mut self, settings: Settings) -> color_eyre::eyre::Result<()> {
        self.settings = Some(settings);
        Ok(())
    }

    fn search(&mut self, query: &str) -> Result<bool> {
        // ignore empty queries
        if query.is_empty() {
            return Ok(false);
        }
        let result = crate::search_modules::applications::desktop::sort_applications(
            &mut self.data.applications,
            query,
        );

        if result.is_empty() {
            log::info!("No applications matched the query: {}", query);
            return Ok(false);
        }

        self.results = result;

        log::info!(
            "Found {} applications matching the query: {}",
            self.results.len(),
            query
        );

        Ok(true)
    }
    fn get_ui_results(&self) -> Vec<SearchResult> {
        self.results
            .iter()
            .map(|score| {
                let s = score.score;
                let idx = score.index;

                let app = self.data.applications.get(idx).unwrap();

                let app_clone = app.clone();
                SearchResult {
                    result: app.name.clone(),
                    score: s,
                    // source_module: self.name().to_string(),
                    launch: Rc::new(move || app_clone.launch()),
                }
            })
            .collect()
    }

    fn execute(&mut self, result: &SearchResult) -> () {
        let _ = result;
    }
}
