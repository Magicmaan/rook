pub mod applications;
pub mod maths;

use std::rc::Rc;

use crate::settings::settings::Settings;
use color_eyre::Result;
use serde::{Deserialize, Serialize, ser::SerializeStruct};

pub trait SearchModule {
    fn name(&self) -> &str {
        "Unnamed Module"
    }
    fn register_action_handler(
        &mut self,
        handler: tokio::sync::mpsc::UnboundedSender<crate::common::action::Action>,
    ) -> color_eyre::eyre::Result<()> {
        let _ = handler;
        Ok(())
    }

    /// Register a configuration handler that provides configuration settings if necessary.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration settings.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn register_settings_handler(&mut self, settings: Settings) -> Result<()> {
        let _ = settings; // to appease clippy
        Ok(())
    }
    /// Processes a search query and determines module candidacy.
    ///
    /// This method evaluates whether the module is relevant to the given search query.
    /// For example, a math module would return true for queries like "1+1" or "calculate",
    /// while a file search module might return true for filesystem-related queries.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string to evaluate
    ///
    /// # Returns
    ///
    /// * `bool` - True if this module has results for the query, false otherwise
    fn search(&mut self, query: &str) -> Result<bool>;

    fn execute(&mut self, result: &ListResult) -> () {
        let _ = result;
    }
    // fn get_results(&self)
    fn get_ui_results(&self) -> Vec<ListResult> {
        vec![]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoredResult {
    pub index: usize,
    pub score: u16,
}

fn clone_box<F: Fn() + Send + Sync + 'static>(f: F) -> Box<dyn Fn() + Send + Sync> {
    Box::new(f)
}

pub struct ListResult {
    pub result: String,
    pub score: u16,
    pub launch: Rc<dyn Fn() -> bool + Send + Sync>,
    // pub launch: Rc<dyn Fn() -> bool + Send + Sync>,
}

impl Default for ListResult {
    fn default() -> Self {
        Self {
            result: String::new(),
            score: 0,
            launch: Rc::new(|| false),
            // launch: Rc::new(|| false),
        }
    }
}
impl std::fmt::Debug for ListResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UIResult")
            .field("result", &self.result)
            .field("score", &self.score)
            .finish()
    }
}

impl Clone for ListResult {
    fn clone(&self) -> Self {
        Self {
            result: self.result.clone(),
            score: self.score.clone(),
            launch: self.launch.clone(),
        }
    }
}

impl PartialEq for ListResult {
    fn eq(&self, other: &Self) -> bool {
        self.result == other.result && self.score == other.score
    }
}

impl Eq for ListResult {}

impl std::hash::Hash for ListResult {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.result.hash(state);
        self.score.hash(state);
    }
}
impl Serialize for ListResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("UIResult", 2)?;
        state.serialize_field("result", &self.result)?;
        state.serialize_field("score", &self.score)?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for ListResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct UIResultHelper {
            result: String,
            score: u16,
        }

        let helper = UIResultHelper::deserialize(deserializer)?;
        Ok(ListResult {
            result: helper.result,
            score: helper.score,
            launch: Rc::new(|| false),
        })
    }
}
