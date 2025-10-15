use ratatui::layout::Rect;

use crate::ui::ui::UISection;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    #[default]
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MouseState {
    pub x: u16,
    pub y: u16,
    pub pressed: bool,
    pub held: bool,
    pub held_duration: u16, // in ms
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyboardState {
    pub pressed_keys: Vec<(ratatui::crossterm::event::KeyCode, u16)>, // KeyCode, duration in ms
    pub last_key: Option<ratatui::crossterm::event::KeyCode>,
    pub held: bool,
    pub held_duration: i32, // in ms
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SearchState {
    pub searching: bool,
    pub query: String,
    pub results: Vec<(u16, usize)>, // (score, index in applications)
}
impl SearchState {
    pub fn split_at_caret(&self, caret_position: usize) -> (&str, &str) {
        self.query.split_at(caret_position.min(self.query.len()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub applications: Vec<crate::applications::Application>,
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
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Model {
    pub running_state: RunState,
    pub tick: u64,
    pub delta_time: i32, // in ms
    pub time: i32,       // in seconds

    pub mouse: MouseState,
    pub keyboard: KeyboardState,
    pub search: SearchState,

    pub ui: UIState,
    pub settings: crate::settings::Settings,
    pub data: Data,
}

impl Model {
    pub fn is_running(&self) -> bool {
        self.running_state == RunState::Running
    }
    pub fn is_paused(&self) -> bool {
        self.running_state == RunState::Paused
    }
}
