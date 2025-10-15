use crate::model::input::{KeyboardState, MouseState};
use crate::model::search::{SearchData, SearchState};
use crate::model::settings::Settings;

use crate::model::ui::UIState;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    #[default]
    Running,
    Paused,
    Stopped,
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
    pub settings: Settings,
    pub data: SearchData,
}

impl Model {
    pub fn is_running(&self) -> bool {
        self.running_state == RunState::Running
    }
    pub fn is_paused(&self) -> bool {
        self.running_state == RunState::Paused
    }
}
