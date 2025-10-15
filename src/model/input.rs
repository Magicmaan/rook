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
