use ratatui::crossterm::event::KeyEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavigateDirection {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Search {
    Add(char),
    Remove(i8), // number of characters to remove
    Execute,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    //
    KeyPress(KeyEvent),
    //
    Search(Search),
    ItemExecute, // execute selected item in results
    //
    Navigate(NavigateDirection, usize), // direction, number of lines
                                        // NavigateDown(usize),                // number of lines
                                        // NavigateUp(usize),                  // number of lines
                                        // NavigateLeft(usize),                // number of lines
                                        // NavigateRight(usize),               // number of lines
                                        // NavigateHome,
                                        // NavigateEnd,
}
