use std::{rc::Rc, sync::Arc};

use crate::{
    app::App,
    common::application::Application,
    database::Database,
    search_modules::{ListResult, ScoredResult, SearchModule},
    settings::settings::Settings,
};
use color_eyre::Result;
use futures::executor;
use rusqlite::params;
use tokio::sync::Mutex;
use xdgkit::desktop_entry::DesktopEntry;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DesktopData {
    pub applications: Vec<Application>,
}

pub struct DesktopFilesModule {
    pub settings: Option<Settings>,
    results: Vec<ScoredResult>,
    data: Option<Box<DesktopData>>,
    database: Option<Arc<Mutex<Database>>>,
}

impl DesktopFilesModule {
    pub fn new() -> Self {
        Self {
            settings: None,
            // state,
            results: Vec::new(),
            data: None,
            database: None,
        }
    }
    fn get_database(&self) -> Option<tokio::sync::MutexGuard<'_, Database>> {
        match &self.database {
            Some(arc_mutex) => Some(executor::block_on(arc_mutex.lock())),
            None => None,
        }
    }
}

impl SearchModule for DesktopFilesModule {
    fn name(&self) -> &str {
        "desktop_files_module"
    }
    fn init(&mut self) -> Result<()> {
        if self.database.is_none() {
            return Err(color_eyre::eyre::eyre!(
                "Database handler not registered for DesktopFilesModule"
            ));
        }
        let mut db_applications: Vec<Application> = vec![];
        {
            let mut database = self.get_database().unwrap();

            let conn = database.get_connection_mut();
            let mut stmt = conn
                .prepare(
                    "
                SELECT file_path
                FROM applications
                WHERE file_type = 'desktop_file';
        ",
                )
                .unwrap();
            let apps_iter = stmt.query_map([], |row| {
                let file_path: String = row.get(0)?;
                Ok(file_path)
            });
            for app_name_result in apps_iter.unwrap() {
                match app_name_result {
                    Ok(path) => {
                        log::info!("App matched in database: {}", path);
                        let app =
                            Application::DesktopFile(DesktopEntry::new(path.clone()), path.clone());
                        db_applications.push(app);
                        // You may want to construct your Application here
                    }
                    Err(e) => {
                        log::error!("Failed to get app name from row: {}", e);
                    }
                }
            }
        }
        log::info!("{}, {:?}", db_applications.len(), db_applications);
        self.data = Some(Box::new(DesktopData {
            applications: db_applications.clone(),
        }));
        // self.data
        //     .applications
        //     .extend(db_applications.iter().cloned());
        Ok(())
    }
    fn get_applications(&self) -> Vec<Rc<crate::common::application::Application>> {
        self.data
            .as_ref()
            .unwrap()
            .applications
            .iter()
            .map(|app| Rc::new(app.clone()))
            .collect()
    }
    fn register_settings_handler(&mut self, settings: Settings) -> color_eyre::eyre::Result<()> {
        self.settings = Some(settings);
        Ok(())
    }
    fn register_database_handler(&mut self, database: Arc<Mutex<Database>>) -> Result<()> {
        self.database = Some(database);
        Ok(())
    }

    fn search(&mut self, query: &str) -> Result<bool> {
        // ignore empty queries
        if query.is_empty() {
            return Ok(false);
        }

        let result = crate::search_modules::applications::desktop::sort_applications(
            &mut self.data.as_mut().unwrap().applications,
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
    fn get_ui_results(&self) -> Vec<ListResult> {
        self.results
            .iter()
            .map(|score| {
                let s = score.score;
                let idx = score.index;

                let app = self.data.as_ref().unwrap().applications.get(idx).unwrap();

                let app_clone = app.clone();
                ListResult {
                    result: app.name(),
                    score: s,
                    // source_module: self.name().to_string(),
                    launch: Rc::new(move || app_clone.launch()),
                }
            })
            .collect()
    }

    fn execute(&mut self, result: &ListResult) -> () {
        let _ = result;
    }
}
