use crossterm::event::{KeyEvent, MouseEvent};
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::{
    app::FocusArea, common::layout::RootLayout, components::Component, search_modules::ListResult,
};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
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
    Tick,
    Render,
    Resize(u16, u16), // width, height
    Resume,
    ClearScreen,
    Suspend,
    Error(String),
    Focus(FocusArea),
    FocusNext,
    FocusPrevious,
    UpdateLayout(RootLayout),
    ToggleWizard,
    FocusToggle,
    Unfocus,
}
impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "quit" => Action::Quit,
            "navigate_up" => Action::Navigate(NavigateDirection::Up, 1),
            "navigate_down" => Action::Navigate(NavigateDirection::Down, 1),
            "navigate_left" => Action::Navigate(NavigateDirection::Left, 1),
            "navigate_right" => Action::Navigate(NavigateDirection::Right, 1),
            "navigate_home" => Action::Navigate(NavigateDirection::Home, 1),
            "navigate_end" => Action::Navigate(NavigateDirection::End, 1),
            "focus_next" => Action::FocusNext,
            "focus_previous" => Action::FocusPrevious,
            "suspend" => Action::Suspend,
            "toggle_wizard" => Action::ToggleWizard,
            _ => Action::Error(format!("Unknown action variant: {}", s)),
        }
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Action::from(s.as_str()))
    }
}
