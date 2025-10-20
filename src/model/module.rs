use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};

use crate::model::search::SearchState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UISection {
    Search,
    Results,
    Tooltip,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModuleState {
    pub tick: u64,
    pub delta_time: i32, // in ms
    pub time: i32,       // in seconds

    // data
    pub data: crate::model::search::SearchData,

    // search stuff
    pub search: SearchState,
    pub caret_position: usize,

    // UI related state can go here
    pub focused_element: Option<String>, // e.g. "search", "results"
    pub result_list_state: ratatui::widgets::ListState, // state for the results list
    pub sections: Vec<(UISection, Rect)>,
    pub executing_item: Option<usize>, // index in results
}

impl ModuleState {
    pub fn get_selected_result_index(&self) -> usize {
        self.result_list_state.selected().unwrap_or(0)
    }
}
