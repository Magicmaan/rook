use ratatui::layout::Rect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UISection {
    Search,
    Results,
    Tooltip,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct UIState {
    pub tick: u64,
    pub delta_time: i32, // in ms
    pub time: i32,       // in seconds
    // UI related state can go here
    pub focused_element: Option<String>, // e.g. "search", "results"
    pub caret_position: usize,           // position in the search query
    pub result_list_state: ratatui::widgets::ListState, // state for the results list
    pub sections: Vec<(UISection, Rect)>,
    pub executing_item: Option<usize>, // index in results
}
