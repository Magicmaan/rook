use crate::common::{
    input::{KeyboardState, MouseState},
    module_state::{SearchState, UIState},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    #[default]
    Running,
    Stopped,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AppState {
    pub running_state: RunState,
    pub tick: u64,
    pub delta_time: i32, // in ms
    pub time: i32,       // in seconds

    pub mouse: MouseState,
    pub keyboard: KeyboardState,
    pub ui: UIState,
    pub search: SearchState,
}

impl AppState {
    pub fn is_running(&self) -> bool {
        self.running_state == RunState::Running
    }
}
