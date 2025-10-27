use std::rc::Rc;

use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};

use crate::ui::{results_box::ResultBoxState, search_box::SearchBoxState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UISection {
    Search,
    Results,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchState {
    pub searching: bool,
    pub query: String,
    pub last_search_tick: u64,
    pub previous_query: String,
    pub previous_results: Vec<(u16, usize)>,
}
impl Default for SearchState {
    fn default() -> Self {
        Self {
            searching: false,
            query: String::new(),
            last_search_tick: 0,
            previous_query: String::new(),
            previous_results: Vec::new(),
        }
    }
}

impl SearchState {
    pub fn split_at_caret(&self, caret_position: usize) -> (&str, &str) {
        self.query.split_at(caret_position.min(self.query.len()))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoredResult {
    pub index: usize,
    pub score: u16,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModuleState {
    // search stuff
    // pub search: SearchState,
    // pub results: Vec<Result>,
    // pub previous_results: Vec<Result>,
    // pub caret_position: usize,
    pub results: Vec<ScoredResult>,
}

impl ModuleState {}

fn clone_box<F: Fn() + Send + Sync + 'static>(f: F) -> Box<dyn Fn() + Send + Sync> {
    Box::new(f)
}

pub struct UIResult {
    pub result: String,
    pub score: String,
    pub launch: Rc<dyn Fn() + Send + Sync>,
}
impl Default for UIResult {
    fn default() -> Self {
        Self {
            result: String::new(),
            score: String::new(),
            launch: Rc::new(|| {}),
        }
    }
}
impl std::fmt::Debug for UIResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UIResult")
            .field("result", &self.result)
            .field("score", &self.score)
            .finish()
    }
}

impl Clone for UIResult {
    fn clone(&self) -> Self {
        Self {
            result: self.result.clone(),
            score: self.score.clone(),
            launch: self.launch.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UIState {
    pub result_box_state: ResultBoxState,

    pub search_box_state: SearchBoxState,
}

impl UIState {
    pub fn get_selected_result_index(&self) -> usize {
        self.result_box_state.list_state.selected().unwrap_or(0)
    }
    pub fn set_selected_result_index(&mut self, index: usize) {
        self.result_box_state.list_state.select(Some(index));
    }
    pub fn set_results(&mut self, results: Vec<UIResult>) {
        log::info!("Setting {} results in UIState", results.len());
        self.result_box_state.results = results;
    }
    pub fn get_results(&self) -> &Vec<UIResult> {
        &self.result_box_state.results
    }
    pub fn set_search_post_fix(&mut self, query: String) {
        self.search_box_state.post_fix = query;
    }
    pub fn set_search_query(&mut self, query: String) {
        self.search_box_state.query = query;
    }
    pub fn get_search_query(&self) -> &String {
        &self.search_box_state.query
    }
    pub fn get_caret_position(&self) -> usize {
        self.search_box_state.caret_position
    }
    pub fn set_caret_position(&mut self, position: usize) {
        self.search_box_state.caret_position = position;
    }
}

pub struct UIStateUpdate {
    pub post_fix: String,
    pub results: Vec<UIResult>,
    pub total_potential_results: usize,
}
