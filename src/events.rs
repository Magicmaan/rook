use ratatui::crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    Start,
    Pause,
    Quit,
    //
    KeyPress(KeyEvent),
    KeyUp(KeyEvent),
    KeyDown(KeyEvent),
    KeyHeld(KeyEvent, u16), // KeyEvent, duration in ms
    //
    SearchAdd(char),
    SearchRemove(i8), // number of characters to remove
    SearchClear,
    SearchFocus,
    SearchExecute,
    SearchCancel,
    //
    NavigateDown(usize),  // number of lines
    NavigateUp(usize),    // number of lines
    NavigateLeft(usize),  // number of lines
    NavigateRight(usize), // number of lines
    NavigateHome,
    NavigateEnd,
    Select,
    Back,
}
