use crossterm::event::{KeyEvent, MouseEvent};
use serde::{Deserialize, Serialize};

use crate::{components::Component, search_modules::ListResult};

// use crate::common::module_state::SearchResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavigateDirection {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Search {
    Add(char),
    Remove(i8),      // number of characters to remove
    Execute(String), // execute search with given query
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Quit,
    //
    KeyEvent(KeyEvent),
    MouseEvent(MouseEvent),
    //
    Search(Search),
    SearchResults(Vec<ListResult>),
    ItemExecute(ListResult), // execute selected item in results
    //
    Navigate(NavigateDirection, usize), // direction, number of lines
    // NavigateDown(usize),                // number of lines
    // NavigateUp(usize),                  // number of lines
    // NavigateLeft(usize),                // number of lines
    // NavigateRight(usize),               // number of lines
    // NavigateHome,
    // NavigateEnd,
    Tick,
    Render,
    Resize(u16, u16), // width, height
    Resume,
    ClearScreen,
    Suspend,
    Error(String),
    Focus,
    Unfocus,
}
