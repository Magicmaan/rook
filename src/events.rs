use ratatui::crossterm::event::KeyEvent;

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
    ItemExecute(usize), // index in applications
    //
    NavigateDown(usize),  // number of lines
    NavigateUp(usize),    // number of lines
    NavigateLeft(usize),  // number of lines
    NavigateRight(usize), // number of lines
    NavigateHome,
    NavigateEnd,
    Select,
    Back,
    MouseMove(u16, u16),        // x, y
    MousePress(u16, u16),       // x, y
    MouseDoubleClick(u16, u16), // x, y
    MouseScrollUp(u16, u16),    // x, y
    MouseScrollDown(u16, u16),  // x, y

    Tick,
}
